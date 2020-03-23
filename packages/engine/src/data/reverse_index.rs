use crate::proto::engine::ReverseIndexEntry as ReverseIndexEntryBytes;
use fst::Map as FstMap;

/// Different strings can end up creating the same token (e.g., it's and its both
/// produce ITS); therefore, it is important to account for this in the index
/// structure, particularly for when it comes to highlighting.
pub struct ReverseIndexEntry {
    /// VerseKey => u64 into...
    map: FstMap,
    /// VerseKey => Translation Id => Token Count
    counts: Vec<Vec<u64>>, // TODO: does this need to be u64? Refactoring would just mean changing the from_bytes stuff
    /// VerseKey => Vec<Highlight Word Ids>
    highlights: Vec<(u64, u64)>,
}

impl ReverseIndexEntry {
    pub fn from_bytes_struct(input: &ReverseIndexEntryBytes) -> Self {
        Self {
            map: FstMap::from_bytes(input.map_bytes.clone())
                .expect("Could not construct map from bytes"),
            counts: input
                .counts_map_data
                .iter()
                .map(|bytes| {
                    let mut v = Vec::new();
                    for i in (0..bytes.len()).step_by(8) {
                        let mut chunk = [0u8; 8];
                        chunk.copy_from_slice(&bytes[i..(i + 8)]);
                        v.push(u64::from_be_bytes(chunk));
                    }
                    v
                })
                .collect(),
            highlights: input
                .highlights_map_data
                .iter()
                .map(|bytes| {
                    let mut first: u64 = 0;
                    let mut last: u64 = 0;
                    for i in (0..bytes.len() / 2).step_by(8) {
                        let mut chunk = [0u8; 8];
                        chunk.copy_from_slice(&bytes[i..(i + 8)]);
                        first = u64::from_be_bytes(chunk);
                    }
                    for i in ((bytes.len() / 2)..bytes.len()).step_by(8) {
                        let mut chunk = [0u8; 8];
                        chunk.copy_from_slice(&bytes[i..(i + 8)]);
                        last = u64::from_be_bytes(chunk);
                    }
                    (first, last)
                })
                .collect(),
        }
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn get_counts(&self, verse_key: &[u8]) -> Option<&Vec<u64>> {
        let idx = self.map.get(verse_key);
        idx.map(|idx| &self.counts[idx as usize])
    }

    pub fn get_verse_keys(&self) -> Vec<Vec<u8>> {
        self.map.stream().into_byte_keys()
    }

    pub fn get_highlights(&self, verse_key: &[u8]) -> Option<(u64, u64)> {
        let idx = self.map.get(verse_key);
        idx.map(|idx| self.highlights[idx as usize])
    }
}

pub type ReverseIndex = Vec<ReverseIndexEntry>;
