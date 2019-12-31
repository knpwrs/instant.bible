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
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::time::Instant;
use util::{tokenize, Tokenized};

pub use util::Config;

/// Different strings can end up creating the same token (e.g., it's and its both
/// produce ITS); therefore, it is important to account for this in the index
/// structure, particularly for when it comes to highlighting.
pub struct ReverseIndexEntry {
    verse_highlights: HashMap<VerseKey, Vec<String>>,
    verse_scores: HashMap<VerseKey, Vec<f64>>,
}

pub type ReverseIndex = HashMap<u64, ReverseIndexEntry>;
pub type TranslationVerses = HashMap<Translation, HashMap<VerseKey, String>>;

static MAX_RESULTS: usize = 20;
static PREFIX_EXPANSION_FACTOR: usize = 2;
static PREFIX_EXPANSION_MINIMUM: usize = 4;
static TYPO_1_LEN: usize = 4;
static TYPO_2_LEN: usize = 8;
static SCORE_EXACT: f64 = 1.0;
static SCORE_INEXACT: f64 = 0.5;
pub static TRANSLATION_COUNT: usize = Translation::Total as usize;

struct ReverseIndexEntryWithMatch<'a> {
    entry: &'a ReverseIndexEntry,
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
    fn traverse_fst(&self, tokens: Vec<Tokenized>) -> BTreeMap<String, ReverseIndexEntryWithMatch> {
        let mut found_indices: BTreeMap<String, ReverseIndexEntryWithMatch> = BTreeMap::new();

        for Tokenized { token, .. } in tokens {
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
                    ReverseIndexEntryWithMatch {
                        entry: self.reverse_index.get(&idx).unwrap(),
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
        found_indices: BTreeMap<String, ReverseIndexEntryWithMatch>,
    ) -> HashMap<VerseKey, (Vec<f64>, BTreeSet<String>)> {
        let mut priority_lists: Vec<_> = found_indices.values().collect();
        priority_lists.sort_by(|a, b| {
            if a.exact_match != b.exact_match {
                a.exact_match.cmp(&b.exact_match)
            } else {
                a.entry.verse_scores.len().cmp(&b.entry.verse_scores.len())
            }
        });
        let candidates_list = priority_lists
            .iter()
            .find(|l| l.exact_match && l.entry.verse_scores.len() >= MAX_RESULTS)
            .unwrap_or_else(|| {
                priority_lists
                    .iter()
                    .find(|l| l.entry.verse_scores.len() >= MAX_RESULTS)
                    .unwrap_or_else(|| priority_lists.last().unwrap())
            });
        let mut result_scores = HashMap::with_capacity(candidates_list.entry.verse_scores.len());
        for key in candidates_list.entry.verse_scores.keys() {
            result_scores.insert(*key, (vec![0f64; TRANSLATION_COUNT], BTreeSet::new()));
        }
        for (result_key, result_sh) in result_scores.iter_mut() {
            for ReverseIndexEntryWithMatch { exact_match, entry } in found_indices.values() {
                if entry.verse_scores.contains_key(&result_key) {
                    let found_scores = entry.verse_scores.get(&result_key).unwrap();
                    let multiplier = if *exact_match {
                        SCORE_EXACT
                    } else {
                        SCORE_INEXACT
                    };
                    // Not entirely needless... interestingly this warning only
                    // started showing up after I made result_sh a tuple
                    #[allow(clippy::needless_range_loop)]
                    for i in 0..result_sh.0.len() {
                        result_sh.0[i] += found_scores[i] * multiplier;
                    }
                }
                if entry.verse_highlights.contains_key(&result_key) {
                    let found_highlights = entry.verse_highlights.get(&result_key).unwrap();
                    result_sh.1.extend(found_highlights.iter().cloned());
                }
            }
        }

        result_scores
    }

    #[inline]
    fn collect_results(
        &self,
        result_scores: HashMap<VerseKey, (Vec<f64>, BTreeSet<String>)>,
    ) -> Vec<VerseResult> {
        result_scores
            .iter()
            .sorted_by(|(_key1, sh1), (_key2, sh2)| {
                sh1.0
                    .iter()
                    .max_by(|i, j| i.partial_cmp(&j).unwrap())
                    .partial_cmp(&sh2.0.iter().max_by(|i, j| i.partial_cmp(&j).unwrap()))
                    .unwrap()
                    .reverse()
            })
            .take(MAX_RESULTS)
            .map(|(key, sh)| VerseResult {
                key: Some(*key),
                translation_scores: sh.0.iter().copied().collect(),
                text: (0..Translation::Total as i32)
                    .map(|i| {
                        self.translation_verses
                            .get(&Translation::from_i32(i).unwrap())
                            .unwrap()
                            .get(&key)
                            .map_or_else(|| "".to_string(), |s| s.clone())
                    })
                    .collect(),
                highlights: sh.1.iter().cloned().collect(),
            })
            .collect()
    }

    /// Perform a search against the index
    pub fn search(&self, text: &str) -> ServiceResponse {
        // Tokenize input text
        let start = Instant::now();
        let tokens = tokenize(text);
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
