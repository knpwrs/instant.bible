mod data;
pub mod proto;
pub mod util;

use crate::proto::engine::IndexData;
use data::{ReverseIndex, ReverseIndexEntry, VerseMatch};
use fst::{automaton, Automaton, IntoStreamer, Map as FstMap, Streamer};
use itertools::Itertools;
use proto::data::{Translation, VerseKey};
use proto::service::{
    response::{Timings, VerseResult},
    Response as ServiceResponse,
};
use std::collections::HashMap;
use std::time::Instant;
use util::{gramize, translation_verses_bytes_key};

pub use util::{Config, MAX_PROXIMITY};

static MAX_RESULTS: usize = 20;
pub static TRANSLATION_COUNT: usize = Translation::Total as usize;

pub struct VersearchIndex {
    fst_map: FstMap,
    reverse_index: ReverseIndex,
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
            translation_verses_map: FstMap::from_bytes(index_data.translation_verses)
                .expect("Could not load map from translation verses bytes"),
            translation_verses_strings: index_data.translation_verses_strings,
            verse_popularity: FstMap::from_bytes(index_data.popularity)
                .expect("Could not loap map from popularity bytes"),
        }
    }

    #[inline]
    fn traverse_fst(&self, grams: &[String]) -> HashMap<u64, &ReverseIndexEntry> {
        let mut found_indices: HashMap<u64, &ReverseIndexEntry> = HashMap::new();

        for gram in grams {
            if gram.len() < 3 {
                let prefix_automaton = automaton::Str::new(&gram).starts_with();
                let mut stream = self.fst_map.search(prefix_automaton).into_stream();
                while let Some((_, idx)) = stream.next() {
                    found_indices
                        .entry(idx)
                        .or_insert(&self.reverse_index[idx as usize]);
                }
            } else if let Some(idx) = self.fst_map.get(gram) {
                found_indices
                    .entry(idx)
                    .or_insert(&self.reverse_index[idx as usize]);
            }
        }

        found_indices
    }

    #[inline]
    fn score_results(
        &self,
        found_indices: &HashMap<u64, &ReverseIndexEntry>,
    ) -> HashMap<Vec<u8>, VerseMatch> {
        let mut priority_lists: Vec<_> = found_indices.values().collect();
        priority_lists.sort_by_key(|entry| entry.len());
        // Pick a token list to use as result candidates
        let candidates_list = priority_lists
            .iter()
            // First, try to find a list with >= 3x max results
            .find(|entry| entry.len() >= MAX_RESULTS * 3)
            .unwrap_or_else(|| {
                priority_lists
                    .iter()
                    // Second, try to find a list with >= 2x max results
                    .find(|entry| entry.len() >= MAX_RESULTS * 2)
                    .unwrap_or_else(|| {
                        priority_lists
                            .iter()
                            // Third, try to find a list with >= max results
                            .find(|entry| entry.len() >= MAX_RESULTS)
                            // Fall back to just taking the list with the most results
                            .unwrap_or_else(|| priority_lists.last().unwrap())
                    })
            });
        // Construct empty scores map with each candidate verse
        let mut result_scores = HashMap::with_capacity(candidates_list.len());
        for key_bytes in candidates_list.get_verse_keys() {
            let key = VerseKey::from_be_bytes(&key_bytes);
            result_scores.insert(
                key_bytes.clone(),
                VerseMatch::new(key, self.verse_popularity.get(key_bytes).map_or(0, |v| v)),
            );
        }

        // Loop over each candidate verse for scoring
        for (result_key, result_match) in result_scores.iter_mut() {
            // Loop over each found index entry (query word) from the previous step
            for entry in found_indices.values() {
                // Does this found entry match the current verse?
                if let Some(found_counts) = entry.get_counts(&result_key) {
                    for (i, count) in found_counts.iter().enumerate() {
                        // Does the found entry match the current translation?
                        if *count > 0 {
                            // Increment words matched
                            result_match.inc_gram_matches(i);
                        }
                    }
                }

                // Track words to highlight for this result
                if let Some(_found_highlights) = entry.get_highlights(result_key) {
                    // TODO: highlights
                    // result_match.extend_highlights(found_highlights);
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
                    .map(|_i| {
                        0
                        // self.highlight_words
                        //     .get(*i as usize)
                        //     .expect("Invalid highlight word index")
                    })
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
        let grams = gramize(text);
        let tokenize_us = start.elapsed().as_micros() as i32;

        if grams.is_empty() {
            return ServiceResponse {
                results: Vec::new(),
                timings: None,
            };
        }

        // Expand and determine score multiplier for each token
        let start = Instant::now();
        let found_indices = self.traverse_fst(&grams);
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
