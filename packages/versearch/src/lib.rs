pub mod btrie;
pub mod data;

use btrie::{BTrieRoot, PrefixIterator};
use data::{JsonVerse, VerseKey};
use lazy_static::lazy_static;
use regex::Regex;
use std::iter::Peekable;

pub struct VersearchIndex {
    btrie: BTrieRoot<VerseKey>,
}

lazy_static! {
    static ref RE: Regex = Regex::new(r"\W+").unwrap();
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

    pub fn insert_verse(&mut self, verse: &JsonVerse) {
        let key = VerseKey::from(&verse.book, verse.chapter, verse.verse)
            .expect("Could not create VerseKey from input!");
        for word in RE.split(&verse.text.to_uppercase()) {
            // We could store Rc<VerseKey> but the deserialization wouldn't work automatically
            self.btrie.insert(word, key.clone());
        }
    }

    pub fn search(&self, text: &str) -> Option<Vec<VerseKey>> {
        // Step 1: collect all matches
        let mut matching_iters: Vec<Peekable<PrefixIterator<VerseKey>>> = Vec::new();
        for word in RE.split(&text.to_uppercase()) {
            if let Some(iter) = self.btrie.iter_prefix(word) {
                matching_iters.push(iter.peekable());
            }
        }
        // Step 2: find all common matches
        let mut results: Vec<VerseKey> = Vec::new();
        while matching_iters.iter_mut().all(|i| i.peek().is_some()) {
            let check = *(matching_iters.first_mut()?).peek()?;
            if matching_iters.iter_mut().all(|j| j.peek() == Some(&check)) {
                // If all iterators are at the same current value, push result and advance every iterator!
                results.push(check.clone());
                for iter in matching_iters.iter_mut() {
                    iter.next();
                }
            } else {
                // Otherwise only increment the minimum vector
                let mut iter_iter = matching_iters.iter_mut();
                let mut least = iter_iter.next()?;
                for iter in iter_iter {
                    if iter.peek()? < least.peek()? {
                        least = iter;
                    }
                }
                least.next();
            }
        }
        // Step 3: We made it!
        Some(results)
    }

    pub fn index_to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string(&self.btrie)
    }

    pub fn index_to_bincode(&self) -> Result<Vec<u8>, bincode::Error> {
        bincode::serialize(&self.btrie)
    }
}
