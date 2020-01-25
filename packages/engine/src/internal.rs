use crate::proto::data::VerseKey;
use crate::proto::service::response::verse_result::Ranking as ServiceRanking;
use crate::TRANSLATION_COUNT;
use std::cmp::Ordering;

#[derive(PartialEq, Eq)]
struct InternalServiceRanking {
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
        // Sort by typos ascending (fewer typos == higher rank)
        if self.ranking.typos != other.ranking.typos {
            return self.ranking.typos.cmp(&other.ranking.typos);
        }
        // Sort by matched query words descending (more words matched == higher rank)
        if self.ranking.words != other.ranking.words {
            return other.ranking.words.cmp(&self.ranking.words);
        }
        // Sort by total proximity ascending (lower proximity == higher rank)
        if self.ranking.proximity != other.ranking.proximity {
            return self.ranking.proximity.cmp(&other.ranking.proximity);
        }
        // Sort by number of exactly matching query words descending (more exact matches == higher rank)
        other.ranking.proximity.cmp(&self.ranking.proximity)
    }
}

impl PartialOrd for InternalServiceRanking {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(PartialEq, Eq)]
pub struct VerseMatch {
    pub key: VerseKey,
    pub highlights: Vec<usize>,
    rankings: Vec<InternalServiceRanking>,
}

impl VerseMatch {
    pub fn new(key: VerseKey) -> Self {
        let mut rankings = Vec::with_capacity(TRANSLATION_COUNT);

        for i in 0..TRANSLATION_COUNT {
            rankings.push(InternalServiceRanking::new(i));
        }

        Self {
            key,
            rankings,
            highlights: Vec::new(),
        }
    }

    pub fn inc_typos(&mut self, idx: usize) {
        self.rankings[idx].inc_typos();
    }

    pub fn inc_words(&mut self, idx: usize) {
        self.rankings[idx].inc_words();
    }

    pub fn add_proximity(&mut self, idx: usize, prox: i32) {
        self.rankings[idx].add_proximity(prox);
    }

    pub fn inc_exact(&mut self, idx: usize) {
        self.rankings[idx].inc_exact();
    }

    pub fn extend_highlights(&mut self, hi: &[usize]) {
        self.highlights.extend(hi)
    }

    pub fn to_service_rankings(&self) -> Vec<ServiceRanking> {
        self.rankings
            .iter()
            .map(|r| r.to_service_ranking())
            .collect()
    }

    pub fn top_translation(&self) -> i32 {
        self.rankings
            .iter()
            .enumerate()
            .min_by(|(_, m1), (_, m2)| m1.cmp(m2))
            .unwrap()
            .0 as i32
    }
}

impl PartialOrd for VerseMatch {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let self_min = self.rankings.iter().min()?;
        let other_min = other.rankings.iter().min()?;

        if self_min != other_min {
            self_min.partial_cmp(other_min)
        } else {
            self.key.partial_cmp(&other.key)
        }
    }
}

impl Ord for VerseMatch {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}
