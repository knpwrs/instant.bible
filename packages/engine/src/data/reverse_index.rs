use std::ops::Range;

/// Dense id of a verse within the index. Ids are assigned in verse key order, so sorting by id
/// is the same as sorting by `VerseKey`.
pub type VerseId = u16;

/// Maps every token id to the verses containing it.
///
/// All entries are stored in flat arrays (compressed sparse row layout) rather than one set of
/// heap allocations per token and per verse: entry `t` owns postings
/// `entry_offsets[t]..entry_offsets[t + 1]`, and the verses of an entry are sorted ascending so
/// entries can be intersected and binary searched.
///
/// Different strings can end up creating the same token (e.g., it's and its both produce ITS);
/// therefore, each posting tracks the highlight words which produced the token in that verse.
#[derive(Default)]
pub struct ReverseIndex {
    entry_offsets: Vec<u32>,
    /// Posting => verse
    verses: Vec<VerseId>,
    /// Posting => bitmask of translations in which the verse contains the token (so at most 8
    /// translations are supported)
    translations: Vec<u8>,
    /// Posting => range into `highlights`
    highlight_offsets: Vec<u32>,
    /// Highlight word ids
    highlights: Vec<u32>,
}

impl ReverseIndex {
    pub fn with_capacity(entries: usize, postings: usize) -> Self {
        let mut entry_offsets = Vec::with_capacity(entries + 1);
        entry_offsets.push(0);
        let mut highlight_offsets = Vec::with_capacity(postings + 1);
        highlight_offsets.push(0);
        Self {
            entry_offsets,
            verses: Vec::with_capacity(postings),
            translations: Vec::with_capacity(postings),
            highlight_offsets,
            highlights: Vec::with_capacity(postings),
        }
    }

    /// Appends a posting to the entry currently being built. Postings must be pushed in
    /// ascending verse order.
    pub fn push_posting(&mut self, verse: VerseId, translations: u8, highlights: &[u32]) {
        self.verses.push(verse);
        self.translations.push(translations);
        self.highlights.extend_from_slice(highlights);
        self.highlight_offsets.push(self.highlights.len() as u32);
    }

    /// Finishes the entry currently being built
    pub fn finish_entry(&mut self) {
        self.entry_offsets.push(self.verses.len() as u32);
    }

    pub fn shrink_to_fit(&mut self) {
        self.entry_offsets.shrink_to_fit();
        self.verses.shrink_to_fit();
        self.translations.shrink_to_fit();
        self.highlight_offsets.shrink_to_fit();
        self.highlights.shrink_to_fit();
    }

    /// Range of postings for the given entry
    #[inline]
    pub fn postings(&self, entry: usize) -> Range<usize> {
        self.entry_offsets[entry] as usize..self.entry_offsets[entry + 1] as usize
    }

    #[inline]
    pub fn len(&self, entry: usize) -> usize {
        self.postings(entry).len()
    }

    /// Sorted verses of the given entry
    #[inline]
    pub fn verses(&self, entry: usize) -> &[VerseId] {
        &self.verses[self.postings(entry)]
    }

    #[inline]
    pub fn translations(&self, posting: usize) -> u8 {
        self.translations[posting]
    }

    #[inline]
    pub fn highlights(&self, posting: usize) -> &[u32] {
        &self.highlights
            [self.highlight_offsets[posting] as usize..self.highlight_offsets[posting + 1] as usize]
    }
}
