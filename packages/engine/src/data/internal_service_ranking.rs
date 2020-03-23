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

    pub fn inc_gram_matches(&mut self) {
        self.ranking.query_words += 1;
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
        // Fall back to translation index
        self.idx.cmp(&other.idx)
    }
}

impl PartialOrd for InternalServiceRanking {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
