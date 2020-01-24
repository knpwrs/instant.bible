use crate::proto::data::VerseKey;
use crate::proto::service::response::verse_result::Ranking as ServiceRanking;
use crate::TRANSLATION_COUNT;
use std::cmp::Ordering;

impl ServiceRanking {
    pub fn new() -> Self {
        Self {
            typos: 0,
            words: 0,
            proximity: 0,
            exact: 0,
        }
    }

    pub fn inc_typos(&mut self) {
        self.typos += 1;
    }

    pub fn inc_words(&mut self) {
        self.words += 1;
    }

    pub fn add_proximity(&mut self, prox: i32) {
        self.proximity += prox;
    }

    pub fn inc_exact(&mut self) {
        self.exact += 1;
    }
}

impl Ord for ServiceRanking {
    fn cmp(&self, other: &Self) -> Ordering {
        // Sort by typos ascending (fewer typos == higher rank)
        if self.typos != other.typos {
            return self.typos.cmp(&other.typos);
        }
        // Sort by matched query words descending (more words matched == higher rank)
        if self.words != other.words {
            return other.words.cmp(&self.words);
        }
        // Sort by total proximity ascending (lower proximity == higher rank)
        if self.proximity != other.proximity {
            return self.proximity.cmp(&other.proximity);
        }
        // Sort by number of exactly matching query words descending (more exact matches == higher rank)
        other.proximity.cmp(&self.proximity)
    }
}

impl PartialOrd for ServiceRanking {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(PartialEq, Eq)]
pub struct VerseMatch {
    pub key: VerseKey,
    pub rankings: Vec<ServiceRanking>,
    pub highlights: Vec<usize>,
}

impl VerseMatch {
    pub fn new(key: VerseKey) -> Self {
        Self {
            key,
            rankings: vec![ServiceRanking::new(); TRANSLATION_COUNT],
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
}

impl PartialOrd for VerseMatch {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let self_min = self.rankings.iter().min()?;
        let other_min = other.rankings.iter().min()?;
        self_min.partial_cmp(other_min)
    }
}

impl Ord for VerseMatch {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}
