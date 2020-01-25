mod internal;
pub mod proto;
pub mod util;

use fst::{automaton, Automaton, IntoStreamer, Map as FstMap};
use fst_levenshtein::Levenshtein;
use internal::VerseMatch;
use itertools::Itertools;
use proto::data::{Translation, VerseKey};
use proto::service::{
    response::{Timings, VerseResult},
    Response as ServiceResponse,
};
use std::collections::HashMap;
use std::time::Instant;
use util::{tokenize, Tokenized};

pub use util::{Config, MAX_PROXIMITY};

/// Different strings can end up creating the same token (e.g., it's and its both
/// produce ITS); therefore, it is important to account for this in the index
/// structure, particularly for when it comes to highlighting.
pub struct ReverseIndexEntry {
    /// VerseKey => Translation Id => Token Count
    counts: HashMap<VerseKey, Vec<usize>>,
    /// VerseKey => Vec<Highlight Word Ids>
    highlights: HashMap<VerseKey, Vec<usize>>,
    /// VerseKey => Token Index => Token Index => Proximity
    proximities: HashMap<VerseKey, HashMap<usize, HashMap<usize, i32>>>,
}

pub type ReverseIndex = HashMap<u64, ReverseIndexEntry>;
pub type TranslationVerses = HashMap<Translation, HashMap<VerseKey, String>>;

static MAX_RESULTS: usize = 20;
static PREFIX_EXPANSION_FACTOR: usize = 2;
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
}

pub struct VersearchIndex {
    fst_map: FstMap,
    reverse_index: ReverseIndex,
    translation_verses: TranslationVerses,
    highlight_words: Vec<String>,
}

impl VersearchIndex {
    #[allow(clippy::new_without_default)]
    pub fn new(
        fst_bytes: Vec<u8>,
        reverse_index: ReverseIndex,
        translation_verses: TranslationVerses,
        highlight_words: Vec<String>,
    ) -> VersearchIndex {
        VersearchIndex {
            fst_map: FstMap::from_bytes(fst_bytes).expect("Could not load map from FST bytes"),
            reverse_index,
            translation_verses,
            highlight_words,
        }
    }

    #[inline]
    fn traverse_fst(&self, tokens: Vec<Tokenized>) -> HashMap<u64, ReverseIndexEntryWithMatch> {
        let mut found_indices: HashMap<u64, ReverseIndexEntryWithMatch> = HashMap::new();

        let mut last_indices: Vec<u64> = Vec::new();

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
                            entry: self.reverse_index.get(&idx).unwrap(),
                            match_type: if is_typo {
                                MatchType::Typo
                            } else {
                                MatchType::Prefix
                            },
                            this_index: *idx,
                            last_indices: last_indices.clone(),
                        });
                if *result == token && token.len() > 1 {
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
        found_indices: HashMap<u64, ReverseIndexEntryWithMatch>,
    ) -> HashMap<VerseKey, VerseMatch> {
        let mut priority_lists: Vec<_> = found_indices.values().collect();
        priority_lists.sort_by(|a, b| {
            if a.match_type != b.match_type {
                a.match_type.cmp(&b.match_type)
            } else {
                a.entry.counts.len().cmp(&b.entry.counts.len())
            }
        });
        let candidates_list = priority_lists
            .iter()
            .find(|l| l.match_type == MatchType::Exact && l.entry.counts.len() >= MAX_RESULTS)
            .unwrap_or_else(|| {
                priority_lists
                    .iter()
                    .find(|l| l.entry.counts.len() >= MAX_RESULTS)
                    .unwrap_or_else(|| priority_lists.last().unwrap())
            });
        let mut result_scores = HashMap::with_capacity(candidates_list.entry.counts.len());
        for key in candidates_list.entry.counts.keys() {
            result_scores.insert(*key, VerseMatch::new(*key));
        }
        for (result_key, result_match) in result_scores.iter_mut() {
            for ReverseIndexEntryWithMatch {
                match_type,
                entry,
                this_index,
                last_indices,
            } in found_indices.values()
            {
                if let Some(found_counts) = entry.counts.get(&result_key) {
                    for (i, count) in found_counts.iter().enumerate() {
                        if *count > 0 {
                            result_match.inc_words(i);
                            match *match_type {
                                MatchType::Exact => result_match.inc_exact(i),
                                MatchType::Typo => result_match.inc_typos(i),
                                _ => {}
                            }
                            let proximity: i32 = if !last_indices.is_empty() {
                                last_indices
                                    .iter()
                                    .map(|li| {
                                        if let Some(m1) = entry.proximities.get(&result_key) {
                                            if let Some(m2) = m1.get(&(*this_index as usize)) {
                                                if let Some(p) = m2.get(&(*li as usize)) {
                                                    return *p;
                                                }
                                            }
                                        }
                                        0
                                    })
                                    .min()
                                    .unwrap_or_else(|| 0)
                            } else {
                                0
                            };
                            result_match.add_proximity(i, proximity);
                        }
                    }
                }

                if let Some(found_highlights) = entry.highlights.get(&result_key) {
                    result_match.extend_highlights(found_highlights);
                }
            }
        }

        result_scores
    }

    #[inline]
    fn collect_results(
        &self,
        results_map: HashMap<proto::data::VerseKey, VerseMatch>,
    ) -> Vec<VerseResult> {
        results_map
            .values()
            .sorted_by(|r1, r2| r1.cmp(r2))
            .take(MAX_RESULTS)
            .map(|r| VerseResult {
                key: Some(r.key),
                top_translation: r.top_translation(),
                text: (0..TRANSLATION_COUNT)
                    .map(|i| {
                        self.translation_verses
                            .get(&Translation::from_i32(i as i32).unwrap())
                            .unwrap()
                            .get(&r.key)
                            .map_or_else(|| "".to_string(), |s| s.clone())
                    })
                    .collect(),
                highlights: r
                    .highlights
                    .iter()
                    .map(|i| {
                        self.highlight_words
                            .get(*i)
                            .expect("Invalid highlight word index")
                    })
                    .cloned()
                    .collect(),
                rankings: r.to_service_rankings(),
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
