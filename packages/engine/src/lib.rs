mod data;
pub mod proto;
pub mod util;

use crate::proto::engine::IndexData;
use data::{ReverseIndex, ReverseIndexEntry, VerseMatch};
use fst::{automaton, Automaton, IntoStreamer, Map as FstMap};
use fst_levenshtein::Levenshtein;
use itertools::Itertools;
use proto::data::{Translation, VerseKey};
use proto::service::{
    response::{Timings, VerseResult},
    Response as ServiceResponse,
};
use std::collections::HashMap;
use std::time::Instant;
use util::{proximity_bytes_key, tokenize, translation_verses_bytes_key, Tokenized};

pub use util::{Config, MAX_PROXIMITY};

static MAX_RESULTS: usize = 20;
static PREFIX_EXPANSION_FACTOR: usize = 3;
static PREFIX_EXPANSION_MINIMUM: usize = 4;
static TYPO_1_LEN: usize = 4;
static TYPO_2_LEN: usize = 8;
pub static TRANSLATION_COUNT: usize = Translation::Total as usize;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
enum MatchType {
    Exact = 0,
    Prefix = 1,
    Typo = 2,
}

struct ReverseIndexEntryWithMatch<'a> {
    entry: &'a ReverseIndexEntry,
    match_type: MatchType,
    this_index: u64,
    last_indices: Vec<u64>,
    qidx: usize,
}

pub struct VersearchIndex {
    fst_map: FstMap,
    reverse_index: ReverseIndex,
    proximities: FstMap,
    highlight_words: Vec<String>,
    translation_verses_map: FstMap,
    translation_verses_strings: Vec<String>,
    verse_popularity: FstMap,
}

impl VersearchIndex {
    #[allow(clippy::new_without_default)]
    pub fn from_index_data_proto_struct(index_data: IndexData) -> Self {
        VersearchIndex {
            fst_map: FstMap::from_bytes(index_data.fst).expect("Could not load map from FST bytes"),
            reverse_index: index_data
                .reverse_index_entries
                .iter()
                .map(|b| ReverseIndexEntry::from_bytes_struct(b))
                .collect(),
            proximities: FstMap::from_bytes(index_data.proximities)
                .expect("Could not load map from proximity bytes"),
            highlight_words: index_data.highlight_words,
            translation_verses_map: FstMap::from_bytes(index_data.translation_verses)
                .expect("Could not load map from translation verses bytes"),
            translation_verses_strings: index_data.translation_verses_strings,
            verse_popularity: FstMap::from_bytes(index_data.popularity)
                .expect("Could not loap map from popularity bytes"),
        }
    }

    #[inline]
    fn traverse_fst(&self, tokens: &[Tokenized]) -> HashMap<u64, ReverseIndexEntryWithMatch> {
        let mut found_indices: HashMap<u64, ReverseIndexEntryWithMatch> = HashMap::new();

        let mut last_indices: Vec<u64> = Vec::new();

        for (qidx, Tokenized { token, .. }) in tokens.iter().enumerate() {
            // Attempt a prefix search
            let prefix_automaton = automaton::Str::new(&token).starts_with();
            let mut results = self
                .fst_map
                .search(prefix_automaton)
                .into_stream()
                .into_str_vec()
                .unwrap();

            // If nothing was found in the prefix search then this token was a typo
            let is_typo = results.is_empty() && token.len() >= TYPO_1_LEN;
            if is_typo {
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
                let mut container =
                    found_indices
                        .entry(*idx)
                        .or_insert_with(|| ReverseIndexEntryWithMatch {
                            entry: &self.reverse_index[*idx as usize],
                            match_type: if is_typo {
                                MatchType::Typo
                            } else {
                                MatchType::Prefix
                            },
                            this_index: *idx,
                            last_indices: last_indices.clone(),
                            qidx,
                        });
                if *result == *token && token.len() > 1 {
                    container.match_type = MatchType::Exact;
                }
            }

            // Store last indices
            last_indices = Vec::new();
            for (_, idx) in results.iter().filter(|(res, _)| {
                // Tokens should be less than an expansion limit with a reasonable expansion for small tokens
                res.len() < (token.len() * PREFIX_EXPANSION_FACTOR).max(PREFIX_EXPANSION_MINIMUM)
            }) {
                last_indices.push(*idx);
            }
        }

        found_indices
    }

    #[inline]
    fn score_results(
        &self,
        found_indices: &HashMap<u64, ReverseIndexEntryWithMatch>,
    ) -> HashMap<Vec<u8>, VerseMatch> {
        let mut priority_lists: Vec<_> = found_indices.values().collect();
        priority_lists.sort_by(|a, b| {
            if a.match_type != b.match_type {
                // Prefer exact matches
                a.match_type.cmp(&b.match_type)
            } else {
                // Order by matches ascending
                a.entry.len().cmp(&b.entry.len())
            }
        });
        // Pick a token list to use as result candidates
        let candidates_list = priority_lists
            .iter()
            // First, try to find a list with >= 3x max results
            .find(|l| l.entry.len() >= MAX_RESULTS * 3)
            .unwrap_or_else(|| {
                priority_lists
                    .iter()
                    // Second, try to find a list with >= 2x max results
                    .find(|l| l.entry.len() >= MAX_RESULTS * 2)
                    .unwrap_or_else(|| {
                        priority_lists
                            .iter()
                            // Third, try to find a list with >= max results
                            .find(|l| l.entry.len() >= MAX_RESULTS)
                            // Fall back to just taking the list with the most results
                            .unwrap_or_else(|| priority_lists.last().unwrap())
                    })
            });
        // Construct empty scores map with each candidate verse
        let mut result_scores = HashMap::with_capacity(candidates_list.entry.len());
        for key_bytes in candidates_list.entry.get_verse_keys() {
            let key = VerseKey::from_be_bytes(&key_bytes);
            result_scores.insert(
                key_bytes.clone(),
                VerseMatch::new(key, self.verse_popularity.get(key_bytes).map_or(0, |v| v)),
            );
        }

        // Loop over each candidate verse for scoring
        for (result_key, result_match) in result_scores.iter_mut() {
            // Loop over each found index entry (query word) from the previous step
            for ReverseIndexEntryWithMatch {
                match_type,
                entry,
                this_index,
                last_indices,
                qidx,
            } in found_indices.values()
            {
                // Does this found entry match the current verse?
                if let Some(found_counts) = entry.get_counts(&result_key) {
                    for (i, count) in found_counts.iter().enumerate() {
                        // Does the found entry match the current translation?
                        if *count > 0 {
                            // Increment words matched
                            result_match.inc_query_words(i, *qidx);
                            // Increment exact/typo matches if necessary
                            match *match_type {
                                MatchType::Exact => result_match.inc_exact(i),
                                MatchType::Typo => result_match.inc_typos(i),
                                _ => {}
                            }
                            // Calculate the proximity between current and last word
                            let proximity = if !last_indices.is_empty() {
                                last_indices
                                    .iter()
                                    .map(|li| {
                                        if let Some(p) = self.proximities.get(proximity_bytes_key(
                                            i as u8,
                                            result_key,
                                            *li as u16,
                                            *this_index as u16,
                                        )) {
                                            p
                                        } else {
                                            0
                                        }
                                    })
                                    .filter(|p| *p != 0)
                                    .min()
                                    .unwrap_or_else(|| 0)
                            } else {
                                0
                            };
                            // Increment proximity
                            result_match.add_proximity(i, proximity as i32);
                        }
                    }
                }

                // Track words to highlight for this result
                if let Some(found_highlights) = entry.get_highlights(result_key) {
                    result_match.extend_highlights(found_highlights);
                }
            }
        }

        // Done scoring!
        result_scores
    }

    #[inline]
    fn collect_results(&self, results_map: &HashMap<Vec<u8>, VerseMatch>) -> Vec<VerseResult> {
        results_map
            .values()
            .sorted_by(|r1, r2| r1.cmp(r2))
            .take(MAX_RESULTS)
            .map(|r| VerseResult {
                key: Some(r.key),
                top_translation: r.top_translation(),
                text: (0..TRANSLATION_COUNT)
                    .map(|i| {
                        let key = translation_verses_bytes_key(i as u8, &r.key);
                        self.translation_verses_map.get(key).map_or_else(
                            || "".to_string(),
                            |idx| {
                                self.translation_verses_strings
                                    .get(idx as usize)
                                    .map_or_else(|| "".to_string(), |s| s.clone())
                            },
                        )
                    })
                    .collect(),
                highlights: r
                    .highlights
                    .iter()
                    .map(|i| {
                        self.highlight_words
                            .get(*i as usize)
                            .expect("Invalid highlight word index")
                    })
                    .cloned()
                    .collect(),
                rankings: r.to_service_rankings(),
                popularity: r.popularity as i32,
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
        let found_indices = self.traverse_fst(&tokens);
        let fst_us = start.elapsed().as_micros() as i32;

        // Score all results
        let start = Instant::now();
        let result_scores = self.score_results(&found_indices);
        let score_us = start.elapsed().as_micros() as i32;

        // Collect ranked results
        let start = Instant::now();
        let results = self.collect_results(&result_scores);
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
