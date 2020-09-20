mod data;
pub mod proto;
pub mod util;

use crate::proto::engine::IndexData;
use data::{ReverseIndex, ReverseIndexEntry, VerseMatch};
use fst::{automaton, raw, Automaton, IntoStreamer, Map as FstMap};
use itertools::Itertools;
use proto::data::{Translation, VerseKey};
use proto::service::{response::VerseResult, Response as ServiceResponse};
use std::collections::HashMap;
use util::{tokenize, translation_verses_bytes_key, Tokenized};
// Previously this module was using wasm-timer, however, it turns out wasm-timer's Instant::now()
// doesn't work in web workers https://github.com/tomaka/wasm-timer/issues/12
// use wasm_timer::Instant;

pub use util::Config;

static MAX_RESULTS: usize = 20;
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
    qidx: usize,
}

pub struct VersearchIndex {
    fst_map: FstMap<Vec<u8>>,
    reverse_index: ReverseIndex,
    highlight_words: Vec<String>,
    translation_verses_map: FstMap<Vec<u8>>,
    translation_verses_strings: Vec<String>,
    verse_popularity: FstMap<Vec<u8>>,
}

impl VersearchIndex {
    #[allow(clippy::new_without_default)]
    pub fn from_index_data_proto_struct(index_data: IndexData) -> Self {
        VersearchIndex {
            fst_map: FstMap::from(
                raw::Fst::new(index_data.fst).expect("Could not load map from FST bytes"),
            ),
            reverse_index: index_data
                .reverse_index_entries
                .iter()
                .map(|b| ReverseIndexEntry::from_bytes_struct(b))
                .collect(),
            highlight_words: index_data.highlight_words,
            translation_verses_map: FstMap::from(
                raw::Fst::new(index_data.translation_verses)
                    .expect("Could not load map from verses bytes"),
            ),
            translation_verses_strings: index_data.translation_verses_strings,
            verse_popularity: FstMap::from(
                raw::Fst::new(index_data.popularity)
                    .expect("Could not load map from popularity bytes"),
            ),
        }
    }

    #[inline]
    fn traverse_fst(&self, tokens: &[Tokenized]) -> HashMap<u64, ReverseIndexEntryWithMatch> {
        let mut found_indices: HashMap<u64, ReverseIndexEntryWithMatch> = HashMap::new();

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
                let lev_automaton = automaton::Levenshtein::new(&token, distance).unwrap();
                results.extend(
                    self.fst_map
                        .search(&lev_automaton)
                        .into_stream()
                        .into_str_vec()
                        .unwrap(),
                );
            }

            // Sort results by token length (undo lexicographical iteration)
            results.sort_by_key(|(t, _)| t.len());

            // Process found tokens
            for (mid, (result, rid)) in results.iter().enumerate() {
                let mut container =
                    found_indices
                        .entry(*rid)
                        .or_insert_with(|| ReverseIndexEntryWithMatch {
                            entry: &self.reverse_index[*rid as usize],
                            match_type: if is_typo {
                                MatchType::Typo
                            } else {
                                MatchType::Prefix
                            },
                            qidx,
                        });
                // This is an exact result if
                //   1. The result token matches the query token OR this is the first result token
                //   2. The token length is greater than 1
                if (*result == *token || mid == 0) && token.len() > 1 {
                    container.match_type = MatchType::Exact;
                }
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
                qidx,
                ..
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
        // See comment on wasm_timer above
        // let start = Instant::now();
        let tokens = tokenize(text);
        // let tokenize_us = start.elapsed().as_micros() as i32;

        // If we have no tokens (empty search), bail
        if tokens.is_empty() {
            return ServiceResponse {
                results: Vec::new(),
                timings: None,
            };
        }

        // Expand and determine score multiplier for each token
        // See comment on wasm_timer above
        // let start = Instant::now();
        let found_indices = self.traverse_fst(&tokens);
        // let fst_us = start.elapsed().as_micros() as i32;

        // If we found no index entries (no valid words), bail
        if found_indices.is_empty() {
            return ServiceResponse {
                results: Vec::new(),
                timings: None,
            };
        }

        // Score all results
        // See comment on wasm_timer above
        // let start = Instant::now();
        let result_scores = self.score_results(&found_indices);
        // let score_us = start.elapsed().as_micros() as i32;

        // Collect ranked results
        // See comment on wasm_timer above
        // let start = Instant::now();
        let results = self.collect_results(&result_scores);
        // let rank_us = start.elapsed().as_micros() as i32;

        // Construct and return response
        ServiceResponse {
            results,
            timings: None,
            // See comment on wasm_timer above
            // timings: Some(Timings {
            //     tokenize: tokenize_us,
            //     fst: fst_us,
            //     score: score_us,
            //     rank: rank_us,
            //     total: tokenize_us + fst_us + score_us + rank_us,
            // }),
        }
    }
}
