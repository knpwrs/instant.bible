pub mod proto;
pub mod util;

use fst::{automaton, Automaton, IntoStreamer, Map as FstMap};
use fst_levenshtein::Levenshtein;
use proto::data::{Translation, VerseKey};
use proto::service::{
    response::{Timings, VerseResult},
    Response as ServiceResponse,
};
use std::collections::{BTreeMap, BinaryHeap, HashMap};
use std::time::Instant;
use util::ordered_tokenize;

pub use util::Config;

pub type ReverseIndexList = Vec<(VerseKey, Vec<f64>)>;
pub type ReverseIndex = HashMap<u64, ReverseIndexList>;

const MAX_RESULTS: usize = 20;
const PREFIX_EXPANSION_FACTOR: usize = 2;
const PREFIX_EXPANSION_MINIMUM: usize = 4;
const TYPO_1_LEN: usize = 4;
const TYPO_2_LEN: usize = 8;
const SCORE_EXACT: f64 = 1.0;
const SCORE_INEXACT: f64 = 0.5;

struct ReverseIndexListWithMultiplier<'a> {
    list: &'a ReverseIndexList,
    exact_match: bool,
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
            for (result, idx) in results.iter().filter(|(res, _)| {
                // Tokens should be less than an expansion limit with a reasonable expansion for small tokens
                res.len() < (token.len() * PREFIX_EXPANSION_FACTOR).max(PREFIX_EXPANSION_MINIMUM)
            }) {
                let mut container = found_lists.entry(result.clone()).or_insert_with(|| {
                    ReverseIndexListWithMultiplier {
                        list: self.reverse_index.get(&idx).unwrap(),
                        exact_match: false,
                    }
                });
                if *result == token {
                    container.exact_match = true;
                }
            }
        }
        let fst_us = start.elapsed().as_micros() as i32;

        // Score all results
        let start = Instant::now();
        let mut priority_lists: Vec<_> = found_lists.values().collect();
        priority_lists.sort_by(|a, b| {
            if a.exact_match != b.exact_match {
                a.exact_match.cmp(&b.exact_match)
            } else {
                a.list.len().cmp(&b.list.len())
            }
        });
        let primary_list = priority_lists
            .iter()
            .find(|l| l.exact_match && l.list.len() >= MAX_RESULTS)
            .unwrap_or_else(|| priority_lists.last().unwrap());
        let mut result_scores = HashMap::with_capacity(primary_list.list.len());
        for (key, _) in primary_list.list {
            result_scores.insert(*key, vec![0f64; Translation::Total as usize]);
        }
        for ReverseIndexListWithMultiplier { exact_match, list } in found_lists.values() {
            for (key, scores) in *list {
                result_scores.entry(*key).and_modify(|previous_scores| {
                    for (previous_score, new_score) in previous_scores.iter_mut().zip(scores.iter())
                    {
                        let multiplier = if *exact_match {
                            SCORE_EXACT
                        } else {
                            SCORE_INEXACT
                        };
                        *previous_score += new_score * multiplier;
                    }
                });
            }
        }
        let score_us = start.elapsed().as_micros() as i32;

        // Collect ranked results
        let start = Instant::now();
        let mut rank_heap = BinaryHeap::with_capacity(primary_list.list.len());
        for (key, scores) in result_scores {
            rank_heap.push(VerseResult {
                key: Some(key),
                translation_scores: scores.iter().copied().collect(),
                total_score: scores.iter().sum(),
            });
        }
        let count = MAX_RESULTS.min(rank_heap.len());
        let mut res = Vec::with_capacity(count);
        for _ in 0..count {
            res.push(rank_heap.pop().unwrap());
        }
        let rank_us = start.elapsed().as_micros() as i32;

        // Construct and return response
        ServiceResponse {
            results: res,
            found_tokens: found_lists.keys().cloned().collect(),
            timings: Some(Timings {
                tokenize: tokenize_us,
                fst: fst_us,
                score: score_us,
                rank: rank_us,
                total: tokenize_us + fst_us + score_us + rank_us,
            }),
        }
    }
}
