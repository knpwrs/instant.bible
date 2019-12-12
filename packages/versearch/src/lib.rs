pub mod proto;
pub mod util;

use fst::{automaton, Automaton, IntoStreamer, Map as FstMap};
use fst_levenshtein::Levenshtein;
use itertools::Itertools;
use proto::data::VerseKey;
use proto::service::{
    response::{Timings, VerseResult},
    Response as ServiceResponse,
};
use std::collections::{BTreeMap, HashMap};
use std::iter::FromIterator;
use std::time::Instant;
use util::ordered_tokenize;

pub use util::Config;

pub type ReverseIndexList = Vec<(VerseKey, Vec<f64>)>;
pub type ReverseIndex = HashMap<u64, ReverseIndexList>;

const MAX_RESULTS: usize = 20;
const PREFIX_EXPANSION_FACTOR: usize = 2;
const TYPO_1_LEN: usize = 4;
const TYPO_2_LEN: usize = 8;
const SCORE_EXACT: f64 = 1.0;
const SCORE_INEXACT: f64 = 0.5;

struct ReverseIndexListWithMultiplier<'a> {
    list: &'a ReverseIndexList,
    multiplier: f64,
}

pub struct VersearchIndex {
    fst_map: FstMap,
    reverse_index: ReverseIndex,
}

impl VersearchIndex {
    #[allow(clippy::new_without_default)]
    pub fn new(fst_bytes: Vec<u8>, reverse_index: ReverseIndex) -> VersearchIndex {
        VersearchIndex {
            fst_map: FstMap::from_bytes(fst_bytes).expect("Could not load map from FST bytes"),
            reverse_index,
        }
    }

    pub fn search(&self, text: &str) -> ServiceResponse {
        let mut found_lists: BTreeMap<String, ReverseIndexListWithMultiplier> = BTreeMap::new();

        // Tokenize input text
        let start = Instant::now();
        let tokens = ordered_tokenize(text);
        let tokenize_us = start.elapsed().as_micros() as i32;

        // Expand and determine score multiplier for each token
        let start = Instant::now();
        for token in tokens {
            // Attempt a prefix search
            let prefix_automaton = automaton::Str::new(&token).starts_with();
            let mut results = self
                .fst_map
                .search(prefix_automaton)
                .into_stream()
                .into_str_vec()
                .unwrap();

            // If nothing was found in the prefix search then this token was a typo
            if results.is_empty() && token.len() >= TYPO_1_LEN {
                let distance = if token.len() >= TYPO_2_LEN { 2 } else { 1 };
                let lev_automaton = Levenshtein::new(&token, distance).unwrap();
                results.extend(
                    self.fst_map
                        .search(&lev_automaton)
                        .into_stream()
                        .into_str_vec()
                        .unwrap(),
                );
            }

            // Process found tokens
            for (result, idx) in results
                .iter()
                .filter(|(s, _)| s.len() <= token.len() * PREFIX_EXPANSION_FACTOR)
                .cloned()
            {
                let multiplier = if result == token {
                    SCORE_EXACT
                } else {
                    SCORE_INEXACT
                };
                let mut container =
                    found_lists
                        .entry(result)
                        .or_insert_with(|| ReverseIndexListWithMultiplier {
                            list: self.reverse_index.get(&idx).unwrap(),
                            multiplier: 0.0,
                        });
                if multiplier > container.multiplier {
                    container.multiplier = multiplier;
                }
            }
        }
        let fst_us = start.elapsed().as_micros() as i32;

        // Process all collected results
        let start = Instant::now();
        let res: Vec<(VerseKey, Vec<f64>)> = found_lists
            .values()
            .map(|ReverseIndexListWithMultiplier { list, multiplier }| {
                list.iter().map(move |(key, scores)| {
                    (
                        *key,
                        scores.iter().map(|i| i * multiplier).collect::<Vec<f64>>(),
                    )
                })
            })
            .kmerge_by(|(vk1, _), (vk2, _)| vk1 < vk2)
            .coalesce(|(vk1, s1), (vk2, s2)| {
                if vk1 == vk2 {
                    Ok((vk1, s1.iter().zip(s2.iter()).map(|(a, b)| a + b).collect()))
                } else {
                    Err(((vk1, s1), (vk2, s2)))
                }
            })
            .sorted_by(|(_, s1), (_, s2)| {
                s2.iter()
                    .sum::<f64>()
                    .partial_cmp(&s1.iter().sum())
                    .unwrap()
            })
            .take(MAX_RESULTS)
            .collect();
        let rank_us = start.elapsed().as_micros() as i32;

        // Construct and return response
        ServiceResponse {
            results: res
                .iter()
                .map(|(key, scores)| VerseResult {
                    key: Some(*key),
                    translation_scores: HashMap::from_iter(
                        (0..res.len() as u32).zip(scores.iter().copied()),
                    ),
                })
                .collect(),
            found_tokens: found_lists.keys().cloned().collect(),
            timings: Some(Timings {
                tokenize: tokenize_us,
                fst: fst_us,
                rank: rank_us,
                total: tokenize_us + fst_us + rank_us,
            }),
        }
    }
}
