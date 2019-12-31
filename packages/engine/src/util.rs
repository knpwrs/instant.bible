use crate::proto::data::{Translation, TranslationData, VerseKey};
use crate::{
    ReverseIndex, ReverseIndexEntry, TranslationVerses, VersearchIndex, TRANSLATION_COUNT,
};
use fst::MapBuilder;
use log::info;
use prost::Message;
use serde::Deserialize;
use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fs;
use std::io::prelude::*;
use std::iter::Iterator;
use std::time::Instant;

#[cfg_attr(test, derive(Debug))]
#[derive(Deserialize)]
pub struct Config {
    pub translation_dir: String,
}

#[cfg_attr(test, derive(Debug))]
#[derive(Clone, Hash, PartialEq, Eq)]
pub struct Tokenized {
    pub source: String,
    pub token: String,
}

impl Ord for Tokenized {
    fn cmp(&self, other: &Self) -> Ordering {
        self.token.cmp(&other.token)
    }
}

impl PartialOrd for Tokenized {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct ScoresAndHighlights {
    scores: Vec<f64>,
    highlights: BTreeSet<String>,
}

pub fn tokenize(input: &str) -> Vec<Tokenized> {
    input
        .split_whitespace()
        .map(|s| Tokenized {
            source: s
                .chars()
                .filter(|c| !c.is_ascii_punctuation() || *c == '\'')
                .collect::<String>(),
            token: s
                .chars()
                // Keeping only alphabetic characters lets users search without
                // concern for apostrophes and the like
                .filter(|c| c.is_alphabetic())
                .collect::<String>()
                .to_uppercase(),
        })
        .collect()
}

pub fn get_index() -> VersearchIndex {
    let config = match envy::from_env::<Config>() {
        Ok(config) => config,
        Err(error) => panic!("{:?}", error),
    };

    let mut total_docs: usize = 0;
    let mut token_scores: BTreeMap<String, HashMap<VerseKey, ScoresAndHighlights>> =
        BTreeMap::new();
    let mut translation_verses: TranslationVerses = HashMap::new();
    let mut highlight_words = BTreeSet::new();

    info!("Loading translations from {:?}", config.translation_dir);

    for entry in fs::read_dir(config.translation_dir).unwrap() {
        let path = entry.unwrap().path();
        if path.is_file() && path.extension().map(|s| s == "pb").unwrap_or(false) {
            let translation_name = path
                .file_stem()
                .expect("Could not get file stem")
                .to_string_lossy()
                .to_string();
            info!("Load translation {:?} from {:?}", translation_name, path);
            let now = Instant::now();
            let mut file_bytes: Vec<u8> = Vec::new();
            fs::File::open(path)
                .unwrap()
                .read_to_end(&mut file_bytes)
                .unwrap();
            let data = TranslationData::decode(file_bytes).expect("Could not parse protobuf");
            let translation_key =
                Translation::from_i32(data.translation).expect("Invalid translation field value");
            info!(
                "Read {} verses in {}ms",
                data.verses.len(),
                now.elapsed().as_millis()
            );
            total_docs = total_docs.max(data.verses.len());
            let now = Instant::now();
            for verse in &data.verses {
                translation_verses
                    .entry(translation_key)
                    .or_insert_with(HashMap::new)
                    .entry(verse.key.unwrap())
                    .or_insert_with(|| verse.text.clone());
                let all_tokens = tokenize(&verse.text);
                let tokens_count = all_tokens.len() as f64;
                // Count up tokens
                for tokenized in all_tokens {
                    let entry = token_scores
                        .entry(tokenized.token)
                        .or_insert_with(HashMap::new)
                        .entry(verse.key.expect("Missing verse key"))
                        // .or_insert_with(|| vec![0.0; TRANSLATION_COUNT]);
                        .or_insert_with(|| ScoresAndHighlights {
                            scores: vec![0.0; TRANSLATION_COUNT],
                            highlights: BTreeSet::new(),
                        });
                    entry.scores[translation_key as usize] += 1.0;
                    entry.highlights.insert(tokenized.source.to_uppercase());
                }
                // Adjust for verse length
                for tokenized in tokenize(&verse.text) {
                    highlight_words.insert(tokenized.source.to_uppercase());
                    let entry = token_scores
                        .get_mut(&tokenized.token)
                        .expect("Token not initialized properly")
                        .get_mut(&verse.key.expect("Missing verse key"))
                        .expect("Scores not initialized properly");
                    for i in &mut entry.scores {
                        *i /= tokens_count;
                    }
                }
            }
            info!(
                "Processed {} verses in {}ms",
                data.verses.len(),
                now.elapsed().as_millis()
            );
        }
    }

    info!("Adjusting scores for inverse document frequency");
    let now = Instant::now();
    for verses in token_scores.values_mut() {
        let count = verses.len();
        for entry in verses.values_mut() {
            for s in entry.scores.iter_mut() {
                *s *= (total_docs as f64 / count as f64).log10();
            }
        }
    }
    info!("Scores adjusted in {}ms", now.elapsed().as_millis());

    let mut build = MapBuilder::memory();
    let mut reverse_index: ReverseIndex = HashMap::new();
    let highlight_words: Vec<_> = highlight_words.iter().cloned().collect();

    let now = Instant::now();

    for (i, (token, entries)) in token_scores.iter().enumerate() {
        build.insert(token.clone(), i as u64).unwrap();
        reverse_index.insert(
            i as u64,
            ReverseIndexEntry {
                verse_scores: entries
                    .iter()
                    .map(|(key, sh)| (*key, sh.scores.clone()))
                    .collect(),
                verse_highlights: entries
                    .iter()
                    .map(|(key, sh)| {
                        (
                            *key,
                            sh.highlights
                                .iter()
                                .map(|s| {
                                    highlight_words
                                        .binary_search(s)
                                        .expect("Could not find index for highlight entry")
                                })
                                .collect(),
                        )
                    })
                    .collect(),
            },
        );
    }

    info!(
        "Indexed {} words in {}ms",
        reverse_index.len(),
        now.elapsed().as_millis()
    );

    info!("Stored {} words for highlighting", highlight_words.len());

    let fst_bytes = build.into_inner().expect("Could not flush bytes for FST");
    info!("FST compiled: {} bytes", fst_bytes.len());

    VersearchIndex::new(
        fst_bytes,
        reverse_index,
        translation_verses,
        highlight_words,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        assert_eq!(
            tokenize("hello, world!"),
            vec![
                Tokenized {
                    source: "hello".to_string(),
                    token: "HELLO".to_string()
                },
                Tokenized {
                    source: "world".to_string(),
                    token: "WORLD".to_string()
                }
            ]
        );
        assert_eq!(
            tokenize("It's all good in the neighborhood which is... good"),
            vec![
                Tokenized {
                    source: "It's".to_string(),
                    token: "ITS".to_string()
                },
                Tokenized {
                    source: "all".to_string(),
                    token: "ALL".to_string(),
                },
                Tokenized {
                    source: "good".to_string(),
                    token: "GOOD".to_string()
                },
                Tokenized {
                    source: "in".to_string(),
                    token: "IN".to_string()
                },
                Tokenized {
                    source: "the".to_string(),
                    token: "THE".to_string()
                },
                Tokenized {
                    source: "neighborhood".to_string(),
                    token: "NEIGHBORHOOD".to_string()
                },
                Tokenized {
                    source: "which".to_string(),
                    token: "WHICH".to_string()
                },
                Tokenized {
                    source: "is".to_string(),
                    token: "IS".to_string()
                },
                Tokenized {
                    source: "good".to_string(),
                    token: "GOOD".to_string()
                },
            ]
        );
    }
}
