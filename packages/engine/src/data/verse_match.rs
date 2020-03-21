use super::InternalServiceRanking;
use crate::proto::data::VerseKey;
use crate::proto::service::response::verse_result::Ranking as ServiceRanking;
use crate::TRANSLATION_COUNT;
use std::cmp::Ordering;

#[derive(PartialEq, Eq)]
pub struct VerseMatch {
    pub key: VerseKey,
    pub highlights: Vec<u64>,
    rankings: Vec<InternalServiceRanking>,
    pub popularity: u64,
}

impl VerseMatch {
    pub fn new(key: VerseKey, popularity: u64) -> Self {
        let mut rankings = Vec::with_capacity(TRANSLATION_COUNT);

        for i in 0..TRANSLATION_COUNT {
            rankings.push(InternalServiceRanking::new(i));
        }

        Self {
            key,
            rankings,
            highlights: Vec::new(),
            popularity,
        }
    }

    pub fn inc_typos(&mut self, idx: usize) {
        self.rankings[idx].inc_typos();
    }

    pub fn inc_query_words(&mut self, idx: usize, query_word: usize) {
        self.rankings[idx].inc_query_words(query_word);
    }

    pub fn add_proximity(&mut self, idx: usize, prox: i32) {
        self.rankings[idx].add_proximity(prox);
    }

    pub fn inc_exact(&mut self, idx: usize) {
        self.rankings[idx].inc_exact();
    }

    pub fn extend_highlights(&mut self, hi: &[u64]) {
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

        if self_min.ranking.query_words != other_min.ranking.query_words {
            other_min
                .ranking
                .query_words
                .partial_cmp(&self_min.ranking.query_words)
        } else {
            other.popularity.partial_cmp(&self.popularity)
        }
    }
}

impl Ord for VerseMatch {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}
