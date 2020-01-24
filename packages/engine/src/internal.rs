use crate::proto::data::VerseKey;
use crate::TRANSLATION_COUNT;
use std::cmp::Ordering;

#[derive(Clone, PartialEq, Eq)]
pub struct TranslationMatch {
    /// The number of typos matched
    pub typos: usize,
    /// The number of words matched from the query
    pub words: usize,
    /// The total proximity of adjacent word pairs in the query
    pub proximity: u8,
    /// The number of exact words matched (no prefix or typo)
    pub exact: usize,
}

impl TranslationMatch {
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

    pub fn add_proximity(&mut self, prox: u8) {
        self.proximity += prox;
    }

    pub fn inc_exact(&mut self) {
        self.exact += 1;
    }
}

impl Ord for TranslationMatch {
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

impl PartialOrd for TranslationMatch {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(PartialEq, Eq)]
pub struct VerseMatch {
    pub key: VerseKey,
    pub matches: Vec<TranslationMatch>,
    pub highlights: Vec<usize>,
}

impl VerseMatch {
    pub fn new(key: VerseKey) -> Self {
        Self {
            key,
            matches: vec![TranslationMatch::new(); TRANSLATION_COUNT],
            highlights: Vec::new(),
        }
    }

    pub fn inc_typos(&mut self, idx: usize) {
        self.matches[idx].inc_typos();
    }

    pub fn inc_words(&mut self, idx: usize) {
        self.matches[idx].inc_words();
    }

    pub fn add_proximity(&mut self, idx: usize, prox: u8) {
        self.matches[idx].add_proximity(prox);
    }

    pub fn inc_exact(&mut self, idx: usize) {
        self.matches[idx].inc_exact();
    }

    pub fn extend_highlights(&mut self, hi: &[usize]) {
        self.highlights.extend(hi)
    }
}

impl PartialOrd for VerseMatch {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let self_min = self.matches.iter().min()?;
        let other_min = other.matches.iter().min()?;
        self_min.partial_cmp(other_min)
    }
}

impl Ord for VerseMatch {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}
