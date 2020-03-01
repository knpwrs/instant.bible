use fst::Map as FstMap;

/// Different strings can end up creating the same token (e.g., it's and its both
/// produce ITS); therefore, it is important to account for this in the index
/// structure, particularly for when it comes to highlighting.
pub struct ReverseIndexEntry {
    /// VerseKey => Translation Id => Token Count
    pub counts_map: FstMap,
    pub counts_data: Vec<Vec<u64>>,
    /// VerseKey => Vec<Highlight Word Ids>
    pub highlights_map: FstMap,
    pub highlights_data: Vec<Vec<u64>>,
}

impl ReverseIndexEntry {
    pub fn from_bytes_struct(input: &ReverseIndexEntryBytes) -> Self {
        Self {
            counts_map: FstMap::from_bytes(input.counts_map_bytes.clone())
                .expect("Could not construct counts_map from bytes"),
            counts_data: input
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
            highlights_map: FstMap::from_bytes(input.highlights_map_bytes.clone())
                .expect("Could not construct highlights_map from bytes"),
            highlights_data: input
                .highlights_map_data
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
        }
    }
}

/// The idea behind this datastructure is to map a verse key to represent the
/// reverse index entry struct in a format which is easy to serialize
pub struct ReverseIndexEntryBytes {
    pub counts_map_bytes: Vec<u8>,
    pub counts_map_data: Vec<Vec<u8>>, // `repeated bytes`, concatenated count bytes
    pub highlights_map_bytes: Vec<u8>,
    pub highlights_map_data: Vec<Vec<u8>>, // `repeated bytes`, bytes are concatenated word ids
}

pub type ReverseIndex = Vec<ReverseIndexEntry>;
