pub mod proto;
pub mod util;

use fst::{automaton, Automaton, IntoStreamer, Map as FstMap};
use fst_levenshtein::Levenshtein;
use itertools::Itertools;
use proto::data::{Translation, VerseKey};
use proto::service::{
    response::{Timings, VerseResult},
    Response as ServiceResponse,
};
use std::collections::{BTreeMap, HashMap};
use std::time::Instant;
use util::ordered_tokenize;

pub use util::Config;

pub type ReverseIndexScores = HashMap<VerseKey, Vec<f64>>;
pub type ReverseIndex = HashMap<u64, ReverseIndexScores>;
pub type TranslationVerses = HashMap<Translation, HashMap<VerseKey, String>>;

const MAX_RESULTS: usize = 20;
const PREFIX_EXPANSION_FACTOR: usize = 2;
const PREFIX_EXPANSION_MINIMUM: usize = 4;
const TYPO_1_LEN: usize = 4;
const TYPO_2_LEN: usize = 8;
const SCORE_EXACT: f64 = 1.0;
const SCORE_INEXACT: f64 = 0.5;
pub const TRANSLATION_COUNT: usize = Translation::Total as usize;

struct ReverseIndexScoresWithMultiplier<'a> {
    index: &'a ReverseIndexScores,
    exact_match: bool,
}

pub struct VersearchIndex {
    fst_map: FstMap,
    reverse_index: ReverseIndex,
    translation_verses: TranslationVerses,
}

impl VersearchIndex {
    #[allow(clippy::new_without_default)]
    pub fn new(
        fst_bytes: Vec<u8>,
        reverse_index: ReverseIndex,
        translation_verses: TranslationVerses,
    ) -> VersearchIndex {
        VersearchIndex {
            fst_map: FstMap::from_bytes(fst_bytes).expect("Could not load map from FST bytes"),
            reverse_index,
            translation_verses,
        }
    }

    #[inline]
    fn traverse_fst(
        &self,
        tokens: Vec<String>,
    ) -> BTreeMap<String, ReverseIndexScoresWithMultiplier> {
        let mut found_indices: BTreeMap<String, ReverseIndexScoresWithMultiplier> = BTreeMap::new();

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
                let mut container = found_indices.entry(result.clone()).or_insert_with(|| {
                    ReverseIndexScoresWithMultiplier {
                        index: self.reverse_index.get(&idx).unwrap(),
                        exact_match: false,
                    }
                });
                if *result == token && token.len() > 1 {
                    container.exact_match = true;
                }
            }
        }

        found_indices
    }

    #[inline]
    fn score_results(
        &self,
        found_indices: BTreeMap<String, ReverseIndexScoresWithMultiplier>,
    ) -> HashMap<VerseKey, Vec<f64>> {
        let mut priority_lists: Vec<_> = found_indices.values().collect();
        priority_lists.sort_by(|a, b| {
            if a.exact_match != b.exact_match {
                a.exact_match.cmp(&b.exact_match)
            } else {
                a.index.len().cmp(&b.index.len())
            }
        });
        let primary_list = priority_lists
            .iter()
            .find(|l| l.exact_match && l.index.len() >= MAX_RESULTS)
            .unwrap_or_else(|| {
                priority_lists
                    .iter()
                    .find(|l| l.index.len() >= MAX_RESULTS)
                    .unwrap_or_else(|| priority_lists.last().unwrap())
            });
        let mut result_scores = HashMap::with_capacity(primary_list.index.len());
        for key in primary_list.index.keys() {
            result_scores.insert(*key, vec![0f64; TRANSLATION_COUNT]);
        }
        for (result_key, result_scores) in result_scores.iter_mut() {
            for ReverseIndexScoresWithMultiplier { exact_match, index } in found_indices.values() {
                if index.contains_key(&result_key) {
                    let found_scores = index.get(&result_key).unwrap();
                    let multiplier = if *exact_match {
                        SCORE_EXACT
                    } else {
                        SCORE_INEXACT
                    };
                    for i in 0..result_scores.len() {
                        result_scores[i] += found_scores[i] * multiplier;
                    }
                }
            }
        }

        result_scores
    }

    #[inline]
    fn collect_results(&self, result_scores: HashMap<VerseKey, Vec<f64>>) -> Vec<VerseResult> {
        result_scores
            .iter()
            .sorted_by(|(_key1, scores1), (_key2, scores2)| {
                scores1
                    .iter()
                    .sum::<f64>()
                    .partial_cmp(&scores2.iter().sum())
                    .unwrap()
                    .reverse()
            })
            .take(MAX_RESULTS)
            .map(|(key, scores)| VerseResult {
                key: Some(*key),
                translation_scores: scores.iter().copied().collect(),
                total_score: scores.iter().sum(),
                text: (0..Translation::Total as i32)
                    .map(|i| {
                        self.translation_verses
                            .get(&Translation::from_i32(i).unwrap())
                            .unwrap()
                            .get(&key)
                            .map_or_else(|| "".to_string(), |s| s.clone())
                    })
                    .collect(),
            })
            .collect()
    }

    /// Perform a search against the index
    pub fn search(&self, text: &str) -> ServiceResponse {
        // Tokenize input text
        let start = Instant::now();
        let tokens = ordered_tokenize(text);
        let tokenize_us = start.elapsed().as_micros() as i32;

        if tokens.is_empty() {
            return ServiceResponse {
                results: Vec::new(),
                timings: None,
            };
        }

        // Expand and determine score multiplier for each token
        let start = Instant::now();
        let found_indices = self.traverse_fst(tokens);
        let fst_us = start.elapsed().as_micros() as i32;

        // Score all results
        let start = Instant::now();
        let result_scores = self.score_results(found_indices);
        let score_us = start.elapsed().as_micros() as i32;

        // Collect ranked results
        let start = Instant::now();
        let results = self.collect_results(result_scores);
        let rank_us = start.elapsed().as_micros() as i32;

        // Construct and return response
        ServiceResponse {
            results,
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
