use super::iter::SubTrieIterator;
use super::util::{first_char, shared_prefix};
use itertools::structs::{Dedup, KMerge};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::cmp::Ord;
use std::collections::btree_set::Iter as BTreeSetIter;
use std::collections::{BTreeSet, HashMap};
use std::sync::Arc;

// The "B"ible Trie is an implementation of a Radix Trie which maps strings to
// associated ordered values

#[derive(Deserialize, Serialize)]
pub struct BTrieRoot<T: Ord> {
    next: HashMap<char, BTrieNode<T>>,
    total: usize,
}

pub type PrefixIterator<'a, T> = Dedup<KMerge<BTreeSetIter<'a, T>>>;

impl<T: Ord> BTrieRoot<T> {
    #[allow(clippy::new_without_default)]
    pub fn new() -> BTrieRoot<T> {
        BTrieRoot {
            next: HashMap::new(),
            total: 0,
        }
    }

    pub fn insert(&mut self, key: &str, value: T) {
        if let Some(first) = first_char(key) {
            self.total += 1;
            self.next
                .entry(first)
                .or_insert_with(|| BTrieNode::new(key))
                .insert(key, value);
        }
    }
    pub fn iter_prefix(&self, search_key: &str) -> Option<PrefixIterator<T>> {
        let first = first_char(search_key)?;
        let search_node = self.next.get(&first)?;
        search_node.iter_prefix(search_key)
    }
}

#[derive(Deserialize, Serialize)]
pub struct BTrieNode<T: Ord> {
    pub key: String,
    pub next: HashMap<char, BTrieNode<T>>,
    pub pf: usize,
    pub values: Arc<BTreeSet<T>>,
}

impl<T: Ord> BTrieNode<T> {
    fn new(key: &str) -> BTrieNode<T> {
        BTrieNode {
            key: key.to_string(),
            next: HashMap::new(),
            pf: 0,
            values: Arc::new(BTreeSet::new()),
        }
    }

    pub fn insert(&mut self, key: &str, value: T) {
        if self.key == key {
            // Case 1: We can insert the value here!
            Arc::get_mut(&mut self.values).unwrap().insert(value);
            self.pf += 1;
        } else if key.starts_with(&self.key) {
            // Case 2: The incoming key belongs in a child node
            let tail_key = &key[self.key.len()..];
            if let Some(first) = first_char(tail_key) {
                self.next
                    .entry(first)
                    .or_insert_with(|| BTrieNode::new(tail_key))
                    .insert(tail_key, value);
                self.pf += 1;
            }
        } else if let Some(count) = shared_prefix(&self.key, key) {
            // Case 3: We need to split this node 😱
            let key_prefix: String = self.key.chars().take(count).collect();
            let key_suffix: String = self.key.chars().skip(count).collect();
            if let Some(first) = first_char(&key_suffix) {
                // Get ready to move this node's content to the new child
                let old_next = std::mem::replace(&mut self.next, HashMap::new());
                let old_values = std::mem::replace(&mut self.values, Arc::new(BTreeSet::new()));
                // Replace this node's key
                self.key = key_prefix;
                // Insert a new child into the freshly created next map
                self.next.entry(first).or_insert(BTrieNode {
                    key: key_suffix.clone(),
                    next: old_next,
                    pf: self.pf,
                    values: old_values,
                });
                self.insert(&key, value);
            }
        }
    }

    // pub fn iter_prefix(&self, search_key: &str) -> Option<PrefixIterator<T>> {
    pub fn iter_prefix(&self, search_key: &str) -> Option<PrefixIterator<T>> {
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
        // Step 2: Return dedeup'd iterator for this subtrie
        Some(SubTrieIterator::new(&node).kmerge().dedup())
    }
}

#[cfg(test)]
mod tests {
    use super::BTrieRoot;

    #[allow(clippy::cognitive_complexity)]
    #[test]
    fn test_btrie() {
        let mut trie: BTrieRoot<usize> = BTrieRoot::new();
        assert_eq!(trie.total, 0);
        trie.insert("fast", 1);
        assert_eq!(trie.total, 1);
        trie.insert("fast", 2);
        assert_eq!(trie.total, 2);
        trie.insert("faster", 2);
        assert_eq!(trie.total, 3);
        trie.insert("toaster", 4);
        assert_eq!(trie.total, 4);
        trie.insert("test", 3);
        assert_eq!(trie.total, 5);
        trie.insert("toasting", 5);
        assert_eq!(trie.total, 6);
        assert_eq!(trie.next.keys().len(), 2);
        let node = trie.next.get(&'f').unwrap();
        assert_eq!(node.key, "fast");
        assert_eq!(node.values.len(), 2);
        assert_eq!(node.pf, 3);
        assert!(node.values.contains(&1));
        assert_eq!(node.next.keys().len(), 1);
        let node = node.next.get(&'e').unwrap();
        assert_eq!(node.key, "er");
        assert_eq!(node.values.len(), 1);
        assert_eq!(node.pf, 1);
        assert!(node.values.contains(&2));
        assert_eq!(node.next.keys().len(), 0);
        let node = trie.next.get(&'t').unwrap();
        assert_eq!(node.key, "t");
        assert_eq!(node.values.len(), 0);
        assert_eq!(node.pf, 3);
        assert_eq!(node.next.keys().len(), 2);
        assert!(node.next.contains_key(&'e'));
        assert!(node.next.contains_key(&'o'));
        {
            let node = node.next.get(&'e').unwrap();
            assert_eq!(node.key, "est");
            assert_eq!(node.values.len(), 1);
            assert_eq!(node.pf, 1);
            assert!(node.values.contains(&3));
            assert_eq!(node.next.keys().len(), 0);
        }
        let node = node.next.get(&'o').unwrap();
        assert_eq!(node.key, "oast");
        assert_eq!(node.values.len(), 0);
        assert_eq!(node.pf, 2);
        assert_eq!(node.next.keys().len(), 2);
        {
            let node = node.next.get(&'e').unwrap();
            assert_eq!(node.key, "er");
            assert_eq!(node.values.len(), 1);
            assert_eq!(node.pf, 1);
            assert!(node.values.contains(&4));
            assert_eq!(node.next.keys().len(), 0);
        }
        let node = node.next.get(&'i').unwrap();
        assert_eq!(node.key, "ing");
        assert_eq!(node.values.len(), 1);
        assert_eq!(node.pf, 1);
        assert!(node.values.contains(&5));
        assert_eq!(node.next.keys().len(), 0);

        let res = trie.iter_prefix(&"fa").unwrap();
        {
            let results: Vec<&usize> = res.collect();
            assert_eq!(results, vec![&1, &2])
        }

        let res = trie.iter_prefix(&"t").unwrap();
        {
            let results: Vec<&usize> = res.collect();
            assert_eq!(results, vec![&3, &4, &5])
        }

        let res = trie.iter_prefix(&"to").unwrap();
        {
            let results: Vec<&usize> = res.collect();
            assert_eq!(results, vec![&4, &5])
        }

        // No results
        let res = trie.iter_prefix(&"toasteroven");
        assert!(res.is_none());
    }
}
