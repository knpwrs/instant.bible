use crate::proto::service::response::verse_result::Ranking as ServiceRanking;
use std::cmp::Ordering;

#[derive(PartialEq, Eq)]
pub struct InternalServiceRanking {
    ranking: ServiceRanking,
    idx: usize,
}

impl InternalServiceRanking {
    pub fn new(idx: usize) -> Self {
        Self {
            ranking: ServiceRanking {
                typos: 0,
                words: 0,
                proximity: 0,
                exact: 0,
            },
            idx,
        }
    }

    pub fn inc_typos(&mut self) {
        self.ranking.typos += 1;
    }

    pub fn inc_words(&mut self) {
        self.ranking.words += 1;
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

impl Ord for InternalServiceRanking {
    fn cmp(&self, other: &Self) -> Ordering {
        // Sort by number of exactly matching query words descending (more exact matches == higher rank)
        if self.ranking.exact != other.ranking.exact {
            return other.ranking.exact.cmp(&self.ranking.exact);
        }
        // Sort by total proximity ascending (lower proximity == higher rank)
        if self.ranking.proximity != other.ranking.proximity {
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
        // Sort by matched query words descending (more words matched == higher rank)
        if self.ranking.words != other.ranking.words {
            return other.ranking.words.cmp(&self.ranking.words);
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
