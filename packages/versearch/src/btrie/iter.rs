use super::trie::BTrieNode;
use std::collections::btree_map::Iter as BTreeMapIter;
use std::collections::HashMap;
use std::hash::Hash;
// use std::ops::Deref;
// use std::cmp::Ordering;

pub struct SubTrieIteratorIterator<'a, K: Ord + Copy, F: Hash + Eq + Copy> {
    stack: Vec<&'a BTrieNode<K, F>>,
}

impl<'a, K: Ord + Copy, F: Hash + Eq + Copy> SubTrieIteratorIterator<'a, K, F> {
    pub fn new(node: &'a BTrieNode<K, F>) -> SubTrieIteratorIterator<'a, K, F> {
        SubTrieIteratorIterator { stack: vec![node] }
    }
}

impl<'a, K: Ord + Copy, F: Hash + Eq + Copy> Iterator for SubTrieIteratorIterator<'a, K, F> {
    // type Item = BTreeMapIter<'a, K, HashMap<F, usize>>;
    type Item = BTreeMapIter<'a, K, HashMap<F, usize>>;
    // type Item = impl Iterator<Item = SubTrieIteratorIteratorItem<K, F>>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.stack.pop() {
            Some(node) => {
                self.stack.extend(node.next.values());
                Some(
                    node.counts.iter(), // .map(|item| SubTrieIteratorIteratorItem { item }),
                )
            }
            _ => None,
        }
    }
}

// #[derive(PartialEq, Eq)]
// pub struct SubTrieIteratorIteratorItem<'a, K: Ord + Copy, F: Hash + Eq + Copy> {
//     item: (&'a K, &'a HashMap<F, usize>),
// }

// impl<K: Ord + Copy, F: Hash + Eq + Copy> Deref for SubTrieIteratorIteratorItem<K, F> {
//     type Target = (K, HashMap<F, usize>);

//     fn deref(&self) -> &Self::Target {
//         &self.item
//     }
// }

// impl<K: Ord + Copy, F: Hash + Eq + Copy> Ord for SubTrieIteratorIteratorItem<K, F> {
//     fn cmp(&self, other: &Self) -> Ordering {
//         self.item.0.cmp(&other.item.0)
//     }
// }

// impl<K: Ord + Copy, F: Hash + Eq + Copy> PartialOrd for SubTrieIteratorIteratorItem<K, F> {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         Some(self.cmp(other))
//     }
// }
