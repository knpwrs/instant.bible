use super::trie::BTrieNode;
use std::collections::btree_set::Range as RangeIter;

pub struct SubTrieIterator<'a, T: Ord> {
    stack: Vec<&'a BTrieNode<T>>,
    iter: RangeIter<'a, T>,
    last_value: Option<&'a T>,
}

impl<'a, T: Ord> SubTrieIterator<'a, T> {
    pub fn new(node: &'a BTrieNode<T>) -> SubTrieIterator<'a, T> {
        SubTrieIterator {
            stack: node.next.values().collect(),
            iter: node.values.range(..),
            last_value: None,
        }
    }
}

impl<'a, T: Ord> Iterator for SubTrieIterator<'a, T>
where
    T: std::fmt::Debug,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        println!("NEXT!");
        match self.iter.next() {
            Some(value) => {
                println!("Some value {:?}", value);
                // If there's a current value, save and return it
                self.last_value = Some(value);
                self.last_value
            }
            None => {
                println!("No value");
                // Otherwise, do we have anything in the stack?
                if self.stack.is_empty() {
                    println!("Empty stack!");
                    self.last_value = None;
                    return self.last_value;
                }
                // If so, start processing the next node
                let node = self.stack.pop()?;
                println!("Got a node with {} values", node.next.values().len());
                self.stack.extend(node.next.values());
                match self.last_value {
                    Some(last_value) => {
                        println!("Assigning new range iter from {:?}", last_value);
                        self.iter = node.values.range(last_value..);
                        match self.iter.next() {
                            Some(next_value) => {
                                // We may need to skip the initial value
                                if next_value == last_value {
                                    println!("Skipping first value");
                                    return self.iter.next();
                                }
                                return Some(next_value);
                            }
                            None => return self.next(),
                        }
                    }
                    None => return None,
                }
            }
        }
    }
}
