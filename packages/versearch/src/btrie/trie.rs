use super::iter::SubTrieIteratorIterator;
use super::util::{first_char, shared_prefix};
use crate::util::tokenize;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::cmp::Ord;
use std::collections::{BTreeMap, HashMap};
use std::hash::Hash;
use std::sync::Arc;

// The "B"ible Trie is an implementation of a Radix Trie which maps strings to
// associated ordered values

#[derive(Deserialize, Serialize)]
pub struct BTrieRoot<K: Ord + Copy, F: Hash + Eq + Copy> {
    next: HashMap<char, BTrieNode<K, F>>,
}

impl<K: Ord + Copy, F: Hash + Eq + Copy> BTrieRoot<K, F> {
    #[allow(clippy::new_without_default)]
    pub fn new() -> BTrieRoot<K, F> {
        BTrieRoot {
            next: HashMap::new(),
        }
    }

    pub fn insert_doc(&mut self, id: &K, doc: &HashMap<F, String>) {
        for (field, text) in doc {
            let tokens = tokenize(text);
            for token in tokens {
                if let Some(first) = first_char(&token) {
                    self.next
                        .entry(first)
                        .or_insert_with(|| BTrieNode::new(&token))
                        .insert_field(id, field, &token);
                }
            }
        }
    }

    pub fn iter_prefix(
        &self,
        search_key: &str,
    ) -> Option<impl Iterator<Item = (K, HashMap<F, usize>)>> {
        let first = first_char(search_key)?;
        let search_node = self.next.get(&first)?;
        search_node.iter_prefix(search_key)
    }
}

#[derive(Deserialize, Serialize)]
pub struct BTrieNode<K: Ord + Copy, F: Hash + Eq + Copy> {
    pub key: String,
    pub next: HashMap<char, BTrieNode<K, F>>,
    pub counts: Arc<BTreeMap<K, HashMap<F, usize>>>,
}

impl<K: Ord + Copy, F: Hash + Eq + Copy> BTrieNode<K, F> {
    fn new(key: &str) -> BTrieNode<K, F> {
        BTrieNode {
            key: key.to_string(),
            next: HashMap::new(),
            counts: Arc::new(BTreeMap::new()),
        }
    }

    pub fn insert_field(&mut self, id: &K, field: &F, token: &str) {
        if self.key == token {
            // Case 1: We can insert the value here!
            let count = Arc::get_mut(&mut self.counts)
                .unwrap()
                .entry(id.clone())
                .or_insert_with(|| HashMap::new())
                .entry(field.clone())
                .or_insert(0);
            *count += 1;
        } else if token.starts_with(&self.key) {
            // Case 2: The incoming key belongs in a child node
            let tail_key = &token[self.key.len()..];
            if let Some(first) = first_char(tail_key) {
                self.next
                    .entry(first)
                    .or_insert_with(|| BTrieNode::new(tail_key))
                    .insert_field(id, field, tail_key);
            }
        } else if let Some(count) = shared_prefix(&self.key, token) {
            // Case 3: We need to split this node 😱
            let key_prefix: String = self.key.chars().take(count).collect();
            let key_suffix: String = self.key.chars().skip(count).collect();
            if let Some(first) = first_char(&key_suffix) {
                // Get ready to move this node's content to the new child
                let old_next = std::mem::replace(&mut self.next, HashMap::new());
                let old_counts = std::mem::replace(&mut self.counts, Arc::new(BTreeMap::new()));
                // Replace this node's key
                self.key = key_prefix;
                // Insert a new child into the freshly created next map
                self.next.entry(first).or_insert(BTrieNode {
                    key: key_suffix.clone(),
                    next: old_next,
                    counts: old_counts,
                });
                self.insert_field(id, field, token);
            }
        }
    }

    pub fn iter_prefix(
        &self,
        search_key: &str,
    ) -> Option<impl Iterator<Item = (K, HashMap<F, usize>)>> {
        let mut node = self;
        let mut char_count = node.key.len();
        let target_count = search_key.chars().count();
        // Step 1: Walk down the tree as far as we can
        while char_count < target_count {
            if let Some(search_char) = search_key.chars().nth(char_count) {
                if let Some(next_node) = node.next.get(&search_char) {
                    char_count += next_node.key.chars().count();
                    node = next_node;
                } else {
                    // No futher nodes! The desired prefix does not exist in the trie!
                    return None;
                }
            }
        }
        // Step 2: Return m'rg'd'd'up'd iterator for this subtrie
        Some(
            SubTrieIteratorIterator::new(&node)
                .kmerge_by(|(k1, _), (k2, _)| k1 < k2)
                .map(|(k, m)| (*k, *m))
                .coalesce(|(k1, c1), (k2, c2)| {
                    if k1 == k2 {
                        let mut new_counts = HashMap::new();
                        for counts in &[c1, c2] {
                            for (field, count) in counts.iter() {
                                let new_count = new_counts.entry(*field).or_insert(0);
                                *new_count += count;
                            }
                        }
                        Ok((k1, new_counts))
                    } else {
                        Err(((k1, c1), (k2, c2)))
                    }
                }),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::BTrieRoot;
    use super::HashMap;
    use cascade::cascade;

    #[allow(clippy::cognitive_complexity)]
    #[test]
    fn test_btrie() {
        let mut trie: BTrieRoot<usize, usize> = BTrieRoot::new();
        let mut doc1 = HashMap::new();
        doc1.insert(1, "fast".to_string());
        trie.insert_doc(&1, &doc1);
        let mut doc2 = HashMap::new();
        doc2.insert(1, "faster".to_string());
        trie.insert_doc(&2, &doc2);
        let mut doc4 = HashMap::new();
        doc4.insert(1, "toaster".to_string());
        trie.insert_doc(&4, &doc4);
        let mut doc3 = HashMap::new();
        doc3.insert(3, "test".to_string());
        trie.insert_doc(&3, &doc3);
        let mut doc5 = HashMap::new();
        doc5.insert(1, "toasting".to_string());
        trie.insert_doc(&5, &doc5);
        assert_eq!(trie.next.keys().len(), 2);
        let node = trie.next.get(&'F').unwrap();
        assert_eq!(node.key, "FAST");
        assert_eq!(node.counts.len(), 1);
        // assert!(node.counts.contains(&1));
        assert_eq!(node.next.keys().len(), 1);
        let node = node.next.get(&'E').unwrap();
        assert_eq!(node.key, "ER");
        assert_eq!(node.counts.len(), 1);
        // assert!(node.counts.contains(&2));
        assert_eq!(node.next.keys().len(), 0);
        let node = trie.next.get(&'T').unwrap();
        assert_eq!(node.key, "T");
        assert_eq!(node.counts.len(), 0);
        assert_eq!(node.next.keys().len(), 2);
        assert!(node.next.contains_key(&'E'));
        assert!(node.next.contains_key(&'O'));
        {
            let node = node.next.get(&'E').unwrap();
            assert_eq!(node.key, "EST");
            assert_eq!(node.counts.len(), 1);
            // assert!(node.counts.contains(&3));
            assert_eq!(node.next.keys().len(), 0);
        }
        let node = node.next.get(&'O').unwrap();
        assert_eq!(node.key, "OAST");
        assert_eq!(node.counts.len(), 0);
        assert_eq!(node.next.keys().len(), 2);
        {
            let node = node.next.get(&'E').unwrap();
            assert_eq!(node.key, "ER");
            assert_eq!(node.counts.len(), 1);
            // assert!(node.counts.contains(&4));
            assert_eq!(node.next.keys().len(), 0);
        }
        let node = node.next.get(&'I').unwrap();
        assert_eq!(node.key, "ING");
        assert_eq!(node.counts.len(), 1);
        // assert!(node.counts.contains(&5));
        assert_eq!(node.next.keys().len(), 0);

        let res = trie.iter_prefix(&"FA").unwrap();
        {
            let results: Vec<(usize, std::collections::HashMap<usize, usize>)> = res.collect();
            assert_eq!(results[0].0, 1);
            assert_eq!(
                results[0].1,
                cascade! {
                    HashMap::new();
                    ..insert(1 as usize, 1 as usize);
                }
            );
        }

        // let res = trie.iter_prefix(&"T").unwrap();
        // {
        //     let results: Vec<&usize> = res.collect();
        //     assert_eq!(results, vec![&3, &4, &5])
        // }

        // let res = trie.iter_prefix(&"TO").unwrap();
        // {
        //     let results: Vec<&usize> = res.collect();
        //     assert_eq!(results, vec![&4, &5])
        // }

        // No results
        let res = trie.iter_prefix(&"toasteroven");
        assert!(res.is_none());
    }
}
