use crate::proto::service::response::verse_result::Ranking as ServiceRanking;
use std::cmp::Ordering;
use std::collections::BTreeSet;

#[derive(Eq)]
pub struct InternalServiceRanking {
    pub ranking: ServiceRanking,
    idx: usize,
    query_word_matches: BTreeSet<usize>,
}

impl InternalServiceRanking {
    pub fn new(idx: usize) -> Self {
        Self {
            ranking: ServiceRanking {
                typos: 0,
                query_words: 0,
                proximity: 0,
                exact: 0,
            },
            idx,
            query_word_matches: BTreeSet::new(),
        }
    }

    pub fn inc_typos(&mut self) {
        self.ranking.typos += 1;
    }

    pub fn inc_query_words(&mut self, query_word: usize) {
        self.query_word_matches.insert(query_word);
        self.ranking.query_words = self.query_word_matches.len() as i32;
    }

    pub fn add_proximity(&mut self, prox: i32) {
        self.ranking.proximity += prox;
    }

    pub fn inc_exact(&mut self) {
        self.ranking.exact += 1;
    }

    pub fn to_service_ranking(&self) -> ServiceRanking {
        self.ranking.clone()
    }
}

/// Only consider ranking information for equality
impl PartialEq for InternalServiceRanking {
    fn eq(&self, other: &Self) -> bool {
        self.ranking == other.ranking
    }
}

impl Ord for InternalServiceRanking {
    fn cmp(&self, other: &Self) -> Ordering {
        // Sort by matched query words descending (more words matched == higher rank)
        if self.ranking.query_words != other.ranking.query_words {
            return other.ranking.query_words.cmp(&self.ranking.query_words);
        }
        // Sort by number of exactly matching query words descending (more exact matches == higher rank)
        if self.ranking.exact != other.ranking.exact {
            return other.ranking.exact.cmp(&self.ranking.exact);
        }
        // Sort by total proximity ascending (lower proximity == higher rank)
        if self.ranking.proximity != other.ranking.proximity
            && self.ranking.query_words != other.ranking.query_words
        {
            // Zero proximity is actually maximum proximity
            let self_prox = if self.ranking.proximity == 0 {
                std::i32::MAX
            } else {
                self.ranking.proximity
            };

            let other_prox = if other.ranking.proximity == 0 {
                std::i32::MAX
            } else {
                other.ranking.proximity
            };

            return self_prox.cmp(&other_prox);
        }
        // Sort by typos ascending (fewer typos == higher rank)
        if self.ranking.typos != other.ranking.typos {
            return self.ranking.typos.cmp(&other.ranking.typos);
        }
        // Fall back to translation index
        self.idx.cmp(&other.idx)
    }
}

impl PartialOrd for InternalServiceRanking {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
