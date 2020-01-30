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

pub static MAX_PROXIMITY: u64 = 8;

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

struct VerseStats {
    counts: Vec<usize>,
    highlights: BTreeSet<String>,
}

pub fn tokenize(input: &str) -> Vec<Tokenized> {
    input
        .split_whitespace()
        .map(|s| Tokenized {
            token: s
                .chars()
                // Keeping only alphanumeric characters lets users search without
                // concern for apostrophes and the like
                .filter(|c| c.is_ascii_alphanumeric())
                .collect::<String>()
                .to_uppercase(),
            source: s
                .chars()
                .enumerate()
                // Like tokens but with apostophes and commas (except trailing commas)
                .filter(|(i, c)| {
                    c.is_ascii_alphanumeric() || *c == '\'' || (*c == ',' && *i != s.len() - 1)
                })
                .map(|(_i, c)| c)
                .collect::<String>(),
        })
        .collect()
}

/// Given a translation id, a verse key, and two word ids, generates a sequence
/// of bytes which can be used as a key into an FST map
pub fn proximity_bytes_key(tidx: u8, vkey: &VerseKey, w1i: u16, w2i: u16) -> Vec<u8> {
    let capacity =
        std::mem::size_of::<u8>() + std::mem::size_of::<u16>() * 2 + VerseKey::get_byte_size();
    let mut v = Vec::with_capacity(capacity);
    v.extend(&tidx.to_be_bytes());
    v.extend(&vkey.to_be_bytes());
    v.extend(&w1i.to_be_bytes());
    v.extend(&w2i.to_be_bytes());
    v
}

pub fn get_index() -> VersearchIndex {
    let start = Instant::now();
    let config = match envy::from_env::<Config>() {
        Ok(config) => config,
        Err(error) => panic!("{:?}", error),
    };

    let mut total_docs: usize = 0;
    let mut token_counts: BTreeMap<String, HashMap<VerseKey, VerseStats>> = BTreeMap::new();
    let mut wip_proximities = BTreeMap::new();
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
                .expect("Could not open protobuf file")
                .read_to_end(&mut file_bytes)
                .expect("Could not read protobuf file");
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
                let vkey = verse.key.expect("Missing verse key");
                let verse_tokens = tokenize(&verse.text);
                // Count up tokens
                for (i, tokenized) in verse_tokens.iter().enumerate() {
                    // Save word to get a highlight id later
                    highlight_words.insert(tokenized.source.to_uppercase());
                    // Create new stats entry if needed
                    let entry = token_counts
                        .entry(tokenized.token.clone())
                        .or_insert_with(HashMap::new)
                        .entry(vkey.clone())
                        .or_insert_with(|| VerseStats {
                            counts: vec![0; TRANSLATION_COUNT],
                            highlights: BTreeSet::new(),
                        });
                    // Increment counts
                    entry.counts[translation_key as usize] += 1;
                    // Track highlights
                    entry.highlights.insert(tokenized.source.to_uppercase());
                    // Track proximities
                    for (j, other_tokenized) in verse_tokens.iter().enumerate().skip(i + 1) {
                        let prox = (j - i) as u64;
                        wip_proximities
                            .entry(translation_key as usize)
                            .or_insert_with(BTreeMap::new)
                            .entry(vkey.clone())
                            .or_insert_with(BTreeMap::new)
                            .entry(tokenized.token.clone())
                            .or_insert_with(BTreeMap::new)
                            .entry(other_tokenized.token.clone())
                            .and_modify(|p: &mut u64| {
                                if prox < *p {
                                    *p = prox;
                                } else if prox > MAX_PROXIMITY {
                                    *p = MAX_PROXIMITY
                                }
                            })
                            .or_insert(prox);
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

    let mut build = MapBuilder::memory();
    let mut reverse_index: ReverseIndex = HashMap::new();
    let highlight_words: Vec<_> = highlight_words.iter().cloned().collect();

    let now = Instant::now();

    for (i, (token, entries)) in token_counts.iter().enumerate() {
        build.insert(token.clone(), i as u64).unwrap();

        let mut highlights = HashMap::new();

        for (key, vs) in entries {
            let indices = vs
                .highlights
                .iter()
                .map(|s| {
                    highlight_words
                        .binary_search(s)
                        .expect("Could not find index for highlight entry")
                })
                .collect();
            highlights.insert(*key, indices);
        }

        reverse_index.insert(
            i as u64,
            ReverseIndexEntry {
                counts: entries
                    .iter()
                    .map(|(key, vs)| (*key, vs.counts.clone()))
                    .collect(),
                highlights,
            },
        );
    }

    let fst_bytes = build.into_inner().expect("Could not flush bytes for FST");
    info!("FST compiled: {} bytes", fst_bytes.len());

    info!(
        "Indexed {} tokens in {}ms",
        reverse_index.len(),
        now.elapsed().as_millis()
    );

    info!("Stored {} words for highlighting", highlight_words.len());

    let now = Instant::now();
    let ordered_tokens: Vec<_> = token_counts.keys().cloned().collect();
    let mut proximities_build = MapBuilder::memory();

    // Construct proximity map
    for (tidx, m1) in &wip_proximities {
        for (vkey, m2) in m1 {
            for (w1, m3) in m2 {
                let w1i = ordered_tokens
                    .binary_search(w1)
                    .expect("Could not find index for token") as u16;
                for (w2, p) in m3 {
                    let w2i = ordered_tokens
                        .binary_search(w2)
                        .expect("Could not find index for token")
                        as u16;
                    proximities_build
                        .insert(proximity_bytes_key(*tidx as u8, vkey, w1i, w2i), *p)
                        .unwrap();
                }
            }
        }
    }

    let proximities = proximities_build
        .into_inner()
        .expect("Could not flush proximities map bytes");

    info!(
        "Proximities FST compiled: {} tokens, {} bytes in {}ms",
        ordered_tokens.len(),
        proximities.len(),
        now.elapsed().as_millis()
    );

    info!("get_index done in {}ms", start.elapsed().as_millis());

    VersearchIndex::new(
        fst_bytes,
        reverse_index,
        proximities,
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
