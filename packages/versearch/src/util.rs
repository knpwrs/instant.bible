use crate::proto::data::{Translation, TranslationData, VerseKey};
use crate::VersearchIndex;
use log::info;
use prost::Message;
use serde::Deserialize;
use std::cmp::Ordering;
use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::io::prelude::*;
use std::iter::Peekable;
use std::time::Instant;

fn default_translation_dir() -> String {
    "../text/data".to_string()
}

#[derive(Deserialize, Debug)]
pub struct Config {
    #[serde(default = "default_translation_dir")]
    pub translation_dir: String,
}

/// An intersecting iterator
pub struct InterIter<I: Iterator, F> {
    iters: Vec<Peekable<I>>,
    f: F,
}

impl<I: Iterator, F> InterIter<I, F>
where
    I::Item: Clone,
    F: Fn(I::Item, I::Item) -> Ordering,
{
    pub fn new<ItersType>(in_iters: ItersType, f: F) -> Self
    where
        ItersType: IntoIterator,
        ItersType::Item: IntoIterator<Item = I::Item, IntoIter = I>,
    {
        let mut iters: Vec<Peekable<I>> = Vec::new();
        for iter in in_iters {
            iters.push(iter.into_iter().peekable());
        }
        InterIter { iters, f }
    }
}

impl<I: Iterator, F> Iterator for InterIter<I, F>
where
    I::Item: Clone,
    F: Fn(I::Item, I::Item) -> Ordering,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if self.iters.iter_mut().all(|i| i.peek().is_some()) {
            let max = {
                let mut iters_iter = self.iters.iter_mut();
                let mut max = iters_iter.next()?.peek()?;
                for iter in iters_iter {
                    let val = iter.peek()?;
                    if (self.f)(*val, *max) == Ordering::Greater {
                        max = val;
                    }
                }
                max.clone()
            };
            {
                let iters_iter = self.iters.iter_mut();
                for iter in iters_iter {
                    while (self.f)(*iter.peek()?, max) == Ordering::Less {
                        iter.next();
                    }
                }
            }
            {
                let iters_iter = self.iters.iter_mut();
                for iter in iters_iter {
                    iter.next();
                }
            }
            return Some(max);
        }
        None
    }
}

pub fn tokenize(input: &str) -> Vec<String> {
    input
        .to_uppercase()
        .chars()
        .filter(|c| c.is_alphanumeric() || c.is_whitespace())
        .collect::<String>()
        .split_whitespace()
        .map(|s| s.to_string())
        .collect()
}

pub fn get_index() -> VersearchIndex {
    let mut vi = VersearchIndex::new();
    let config = match envy::from_env::<Config>() {
        Ok(config) => config,
        Err(error) => panic!("{:?}", error),
    };

    info!("Loading translations from {:?}", config.translation_dir);
    let mut verse_docs: BTreeMap<VerseKey, HashMap<Translation, String>> = BTreeMap::new();
    for entry in fs::read_dir(config.translation_dir).unwrap() {
        let path = entry.unwrap().path();
        if path.is_file() && path.extension().map(|s| s == "pb").unwrap_or(false) {
            let translation_name = path
                .file_stem()
                .expect("Could not get file stem")
                .to_string_lossy()
                .to_string();
            info!("Load translation {:?} from {:?}", translation_name, path);
            let now = Instant::now();
            let mut file_bytes: Vec<u8> = Vec::new();
            fs::File::open(path)
                .unwrap()
                .read_to_end(&mut file_bytes)
                .unwrap();
            let data = TranslationData::decode(file_bytes).expect("Could not parse protobuf");
            let translation_key =
                Translation::from_i32(data.translation).expect("Invalid translation field value");
            info!(
                "Read {} verses in {}ms",
                data.verses.len(),
                now.elapsed().as_millis()
            );
            for verse in &data.verses {
                verse_docs
                    .entry(verse.key.expect("Missing verse key"))
                    .or_insert_with(HashMap::new)
                    .insert(translation_key, verse.text.clone());
            }
        }
    }

    let now = Instant::now();
    for (key, doc) in &verse_docs {
        vi.insert_doc(key, doc);
    }
    info!(
        "Indexed {} docs in {}ms",
        verse_docs.len(),
        now.elapsed().as_millis()
    );

    vi
}

#[cfg(test)]
mod tests {
    use super::{tokenize, InterIter};

    #[test]
    fn multiple_iters() {
        let v1 = vec![1, 2, 3, 4, 5];
        let v2 = vec![3, 4, 5, 6];
        let v3 = vec![4, 5, 6, 7];
        let res: Vec<&usize> =
            InterIter::new(vec![v1.iter(), v2.iter(), v3.iter()], |a, b| a.cmp(b)).collect();
        assert_eq!(res, vec![&4, &5]);
    }

    #[test]
    fn one_iter() {
        let v1 = vec![1, 2, 3, 4, 5];
        let res: Vec<&usize> = InterIter::new(vec![v1.iter()], |a, b| a.cmp(b)).collect();
        assert_eq!(res, vec![&1, &2, &3, &4, &5]);
    }

    #[test]
    fn no_iters() {
        let res: Vec<&usize> =
            InterIter::new(Vec::new() as Vec<core::slice::Iter<usize>>, |a, b| a.cmp(b)).collect();
        assert_eq!(res, Vec::new() as Vec<&usize>);
    }

    #[test]
    fn empty_iters() {
        let v1 = vec![];
        let v2 = vec![];
        let v3 = vec![];
        let res: Vec<&usize> =
            InterIter::new(vec![v1.iter(), v2.iter(), v3.iter()], |a: &usize, b| {
                a.cmp(b)
            })
            .collect();
        assert_eq!(res, Vec::new() as Vec<&usize>);
    }

    #[test]
    fn multiple_vecs() {
        let v1 = vec![1, 2, 3, 4, 5];
        let v2 = vec![3, 4, 5, 6];
        let v3 = vec![4, 5, 6, 7];
        let res: Vec<usize> = InterIter::new(vec![v1, v2, v3], |a: usize, b| a.cmp(&b)).collect();
        assert_eq!(res, vec![4, 5]);
    }

    #[test]
    fn one_vec() {
        let v1 = vec![1, 2, 3, 4, 5];
        let res: Vec<usize> = InterIter::new(vec![v1], |a: usize, b| a.cmp(&b)).collect();
        assert_eq!(res, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn no_vecs() {
        let res: Vec<usize> =
            InterIter::new(Vec::new() as Vec<Vec<usize>>, |a: usize, b| a.cmp(&b)).collect();
        assert_eq!(res, Vec::new() as Vec<usize>);
    }

    #[test]
    fn empty_vecs() {
        let v1 = vec![];
        let v2 = vec![];
        let v3 = vec![];
        let res: Vec<usize> = InterIter::new(vec![v1, v2, v3], |a: usize, b| a.cmp(&b)).collect();
        assert_eq!(res, Vec::new() as Vec<usize>);
    }

    #[test]
    fn test_tokenize() {
        assert_eq!(tokenize("Hello, World!"), vec!["HELLO", "WORLD"]);
        assert_eq!(tokenize("Thou shaln't foo!"), vec!["THOU", "SHALNT", "FOO"]);
    }
}
