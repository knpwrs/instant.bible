pub mod btrie;
pub mod proto;
pub mod util;

use btrie::{BTrieRoot, SubTrieIterator};
use lazy_static::lazy_static;
use proto::data::{VerseKey, VerseText};
use regex::Regex;
use util::InterIter;

pub use util::Config;

const MAX_RESULTS: usize = 20;

pub struct VersearchIndex {
    btrie: BTrieRoot<VerseKey>,
}

lazy_static! {
    static ref RE: Regex = Regex::new(r"[\s.]+").unwrap();
}

impl VersearchIndex {
    #[allow(clippy::new_without_default)]
    pub fn new() -> VersearchIndex {
        VersearchIndex {
            btrie: BTrieRoot::new(),
        }
    }

    pub fn from_index_bincode(bytes: Vec<u8>) -> VersearchIndex {
        VersearchIndex {
            btrie: bincode::deserialize(&bytes).unwrap(),
        }
    }

    pub fn insert_verse(&mut self, verse: &VerseText) {
        for word in RE.split(&verse.text.to_uppercase()) {
            // We could store Rc<VerseKey> but the deserialization wouldn't work automatically
            let vkey = verse.key.as_ref().unwrap();
            self.btrie.insert(word, vkey.clone());
        }
    }

    pub fn search(&self, text: &str) -> Option<Vec<VerseKey>> {
        // Step 1: collect all matches
        let mut matching_iters: Vec<SubTrieIterator<VerseKey>> = Vec::new();
        for word in RE.split(&text.to_uppercase()) {
            if let Some(iter) = self.btrie.iter_prefix(word) {
                matching_iters.push(iter);
            }
        }
        // Step 2: find all common matches
        let results: Vec<VerseKey> = InterIter::new(matching_iters)
            .copied()
            .take(MAX_RESULTS)
            .collect();
        // Step 3: We made it!
        Some(results)
    }

    pub fn index_to_bincode(&self) -> Result<Vec<u8>, bincode::Error> {
        bincode::serialize(&self.btrie)
    }
}
