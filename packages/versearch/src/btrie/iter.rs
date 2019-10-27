use super::trie::BTrieNode;
use std::collections::btree_set::Iter as BTreeSetIter;

pub struct SubTrieIterator<'a, T: Ord> {
    stack: Vec<&'a BTrieNode<T>>,
}

impl<'a, T: Ord> SubTrieIterator<'a, T> {
    pub fn new(node: &'a BTrieNode<T>) -> SubTrieIterator<'a, T> {
        SubTrieIterator { stack: vec![node] }
    }
}

impl<'a, T: Ord> Iterator for SubTrieIterator<'a, T> {
    type Item = BTreeSetIter<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.stack.pop() {
            Some(node) => {
                self.stack.extend(node.next.values());
                Some(node.values.iter())
            }
            _ => None,
        }
    }
}
