use crate::proto::data::{Translation, TranslationData, VerseKey};
use crate::{ReverseIndex, VersearchIndex};
use fst::MapBuilder;
use log::info;
use prost::Message;
use serde::Deserialize;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fs;
use std::io::prelude::*;
use std::iter::FromIterator;
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
pub struct InterIter<I: Iterator> {
    iters: Vec<Peekable<I>>,
}

impl<I: Iterator> InterIter<I>
where
    I::Item: Ord + Clone,
{
    pub fn new<ItersType>(in_iters: ItersType) -> Self
    where
        ItersType: IntoIterator,
        ItersType::Item: IntoIterator<Item = I::Item, IntoIter = I>,
    {
        let mut iters: Vec<Peekable<I>> = Vec::new();
        for iter in in_iters {
            iters.push(iter.into_iter().peekable());
        }
        InterIter { iters }
    }
}

impl<I: Iterator> Iterator for InterIter<I>
where
    I::Item: Ord + Clone,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if self.iters.iter_mut().all(|i| i.peek().is_some()) {
            let max = {
                let mut iters_iter = self.iters.iter_mut();
                let mut max = iters_iter.next()?.peek()?;
                for iter in iters_iter {
                    let val = iter.peek()?;
                    if val > max {
                        max = val;
                    }
                }
                max.clone()
            };
            {
                let iters_iter = self.iters.iter_mut();
                for iter in iters_iter {
                    while *iter.peek()? < max {
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
        .filter(|c| c.is_alphabetic() || c.is_whitespace())
        .collect::<String>()
        .split_whitespace()
        .map(|s| s.to_string())
        .collect()
}

pub fn ordered_tokenize(input: &str) -> Vec<String> {
    BTreeSet::from_iter(tokenize(input))
        .iter()
        .cloned()
        .collect()
}

pub fn get_index() -> VersearchIndex {
    let config = match envy::from_env::<Config>() {
        Ok(config) => config,
        Err(error) => panic!("{:?}", error),
    };

    let mut word_counts: BTreeMap<String, BTreeMap<VerseKey, [u8; Translation::Total as usize]>> =
        BTreeMap::new();

    info!("Loading translations from {:?}", config.translation_dir);

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
            let now = Instant::now();
            for verse in &data.verses {
                for token in tokenize(&verse.text) {
                    let counts = word_counts
                        .entry(token)
                        .or_insert_with(BTreeMap::new)
                        .entry(verse.key.expect("Missing verse key"))
                        .or_insert_with(|| [0; Translation::Total as usize]);
                    counts[translation_key as usize] += 1;
                }
            }
            info!(
                "Processed {} verses in {}ms",
                data.verses.len(),
                now.elapsed().as_millis()
            );
        }
    }

    let mut build = MapBuilder::memory();
    let mut reverse_index: ReverseIndex = HashMap::new();

    let now = Instant::now();
    for (i, (word, verses)) in word_counts.iter().enumerate() {
        build.insert(word, i as u64).unwrap();
        reverse_index.insert(
            i as u64,
            verses.iter().map(|(key, val)| (*key, *val)).collect(),
        );
    }
    info!(
        "Indexed {} words in {}ms",
        reverse_index.len(),
        now.elapsed().as_millis()
    );

    let fst_bytes = build.into_inner().expect("Could not flush bytes for FST");
    info!("FST compiled: {} bytes", fst_bytes.len());

    VersearchIndex::new(fst_bytes, reverse_index)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multiple_iters() {
        let v1 = vec![1, 2, 3, 4, 5];
        let v2 = vec![3, 4, 5, 6];
        let v3 = vec![4, 5, 6, 7];
        let res: Vec<&usize> = InterIter::new(vec![v1.iter(), v2.iter(), v3.iter()]).collect();
        assert_eq!(res, vec![&4, &5]);
    }

    #[test]
    fn one_iter() {
        let v1 = vec![1, 2, 3, 4, 5];
        let res: Vec<&usize> = InterIter::new(vec![v1.iter()]).collect();
        assert_eq!(res, vec![&1, &2, &3, &4, &5]);
    }

    #[test]
    fn no_iters() {
        let res: Vec<&usize> =
            InterIter::new(Vec::new() as Vec<core::slice::Iter<usize>>).collect();
        assert_eq!(res, Vec::new() as Vec<&usize>);
    }

    #[test]
    fn empty_iters() {
        let v1 = vec![];
        let v2 = vec![];
        let v3 = vec![];
        let res: Vec<&usize> = InterIter::new(vec![v1.iter(), v2.iter(), v3.iter()]).collect();
        assert_eq!(res, Vec::new() as Vec<&usize>);
    }

    #[test]
    fn multiple_vecs() {
        let v1 = vec![1, 2, 3, 4, 5];
        let v2 = vec![3, 4, 5, 6];
        let v3 = vec![4, 5, 6, 7];
        let res: Vec<usize> = InterIter::new(vec![v1, v2, v3]).collect();
        assert_eq!(res, vec![4, 5]);
    }

    #[test]
    fn one_vec() {
        let v1 = vec![1, 2, 3, 4, 5];
        let res: Vec<usize> = InterIter::new(vec![v1]).collect();
        assert_eq!(res, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn no_vecs() {
        let res: Vec<usize> = InterIter::new(Vec::new() as Vec<Vec<usize>>).collect();
        assert_eq!(res, Vec::new() as Vec<usize>);
    }

    #[test]
    fn empty_vecs() {
        let v1 = vec![];
        let v2 = vec![];
        let v3 = vec![];
        let res: Vec<usize> = InterIter::new(vec![v1, v2, v3]).collect();
        assert_eq!(res, Vec::new() as Vec<usize>);
    }

    #[test]
    fn test_tokenize() {
        assert_eq!(tokenize("hello, world!"), vec!["HELLO", "WORLD"]);
        assert_eq!(
            tokenize("It's all good in the neighborhood which is... good"),
            vec![
                "ITS",
                "ALL",
                "GOOD",
                "IN",
                "THE",
                "NEIGHBORHOOD",
                "WHICH",
                "IS",
                "GOOD",
            ]
        );
    }

    #[test]
    fn test_ordered_tokenize() {
        assert_eq!(ordered_tokenize("hello, world!"), vec!["HELLO", "WORLD"]);
        assert_eq!(
            ordered_tokenize("It's all good in the neighborhood which is... good"),
            vec![
                "ALL",
                "GOOD",
                "IN",
                "IS",
                "ITS",
                "NEIGHBORHOOD",
                "THE",
                "WHICH",
            ]
        );
    }
}
