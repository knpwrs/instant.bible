use crate::proto::data::{Translation, TranslationData, VerseKey};
use crate::{ReverseIndex, TranslationVerses, VersearchIndex, TRANSLATION_COUNT};
use fst::MapBuilder;
use log::info;
use prost::Message;
use serde::Deserialize;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fs;
use std::io::prelude::*;
use std::iter::{FromIterator, Iterator};
use std::time::Instant;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub translation_dir: String,
}

pub fn tokenize(input: &str) -> Vec<String> {
    input
        .to_uppercase()
        .chars()
        .filter(|c| c.is_alphabetic() || c.is_whitespace())
        .collect::<String>()
        .split_whitespace()
        .map(|s| s.to_string())
        .collect()
}

pub fn ordered_tokenize(input: &str) -> Vec<String> {
    BTreeSet::from_iter(tokenize(input))
        .iter()
        .cloned()
        .collect()
}

pub fn get_index() -> VersearchIndex {
    let config = match envy::from_env::<Config>() {
        Ok(config) => config,
        Err(error) => panic!("{:?}", error),
    };

    let mut total_docs: usize = 0;
    let mut token_scores: BTreeMap<String, HashMap<VerseKey, Vec<f64>>> = BTreeMap::new();
    let mut translation_verses: TranslationVerses = HashMap::new();

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
                for token in all_tokens {
                    let counts = token_scores
                        .entry(token)
                        .or_insert_with(HashMap::new)
                        .entry(verse.key.expect("Missing verse key"))
                        .or_insert_with(|| vec![0.0; TRANSLATION_COUNT]);
                    counts[translation_key as usize] += 1.0;
                }
                // Adjust for verse length
                for token in ordered_tokenize(&verse.text) {
                    let scores = token_scores
                        .get_mut(&token)
                        .expect("Token not initialized properly")
                        .get_mut(&verse.key.expect("Missing verse key"))
                        .expect("Scores not initialized properly");
                    for i in scores {
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
        for scores in verses.values_mut() {
            for s in scores.iter_mut() {
                *s *= (total_docs as f64 / count as f64).log10();
            }
        }
    }
    info!("Scores adjusted in {}ms", now.elapsed().as_millis());

    let mut build = MapBuilder::memory();
    let mut reverse_index: ReverseIndex = HashMap::new();

    let now = Instant::now();
    for (i, (word, verses)) in token_scores.iter().enumerate() {
        build.insert(word, i as u64).unwrap();
        reverse_index.insert(i as u64, verses.clone());
    }
    info!(
        "Indexed {} words in {}ms",
        reverse_index.len(),
        now.elapsed().as_millis()
    );

    let fst_bytes = build.into_inner().expect("Could not flush bytes for FST");
    info!("FST compiled: {} bytes", fst_bytes.len());

    VersearchIndex::new(fst_bytes, reverse_index, translation_verses)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        assert_eq!(tokenize("hello, world!"), vec!["HELLO", "WORLD"]);
        assert_eq!(
            tokenize("It's all good in the neighborhood which is... good"),
            vec![
                "ITS",
                "ALL",
                "GOOD",
                "IN",
                "THE",
                "NEIGHBORHOOD",
                "WHICH",
                "IS",
                "GOOD",
            ]
        );
    }

    #[test]
    fn test_ordered_tokenize() {
        assert_eq!(ordered_tokenize("hello, world!"), vec!["HELLO", "WORLD"]);
        assert_eq!(
            ordered_tokenize("It's all good in the neighborhood which is... good"),
            vec![
                "ALL",
                "GOOD",
                "IN",
                "IS",
                "ITS",
                "NEIGHBORHOOD",
                "THE",
                "WHICH",
            ]
        );
    }
}