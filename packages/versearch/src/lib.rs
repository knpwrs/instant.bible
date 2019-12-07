pub mod proto;
pub mod util;

use fst::{automaton, IntoStreamer, Map as FstMap};
use proto::data::{Translation, VerseKey};
use std::collections::HashMap;
use util::{ordered_tokenize, InterIter};

pub use util::Config;

pub type ReverseIndex = HashMap<u64, Vec<(VerseKey, [u8; Translation::Total as usize])>>;

const MAX_RESULTS: usize = 20;

pub struct VersearchIndex {
    fst_map: FstMap,
    reverse_index: ReverseIndex,
}

impl VersearchIndex {
    #[allow(clippy::new_without_default)]
    pub fn new(fst_bytes: Vec<u8>, reverse_index: ReverseIndex) -> VersearchIndex {
        VersearchIndex {
            fst_map: FstMap::from_bytes(fst_bytes).expect("Could not load map from FST bytes"),
            reverse_index,
        }
    }

    pub fn search(&self, text: &str) -> Vec<VerseKey> {
        let tokens = ordered_tokenize(text);
        let mut indices: Vec<u64> = Vec::new();

        for token in tokens {
            let auto = automaton::Str::new(&token);
            if let Ok(results) = self.fst_map.search(auto).into_stream().into_str_vec() {
                indices.extend(results.iter().map(|(_, list)| list));
            }
        }

        InterIter::new(indices.iter().map(|index| {
            self.reverse_index
                .get(index)
                .unwrap()
                .iter()
                .map(|(v, _)| v)
        }))
        .take(MAX_RESULTS)
        .copied()
        .collect()
    }
}
