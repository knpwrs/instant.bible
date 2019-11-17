use crate::proto::data::TranslationData;
use crate::VersearchIndex;
use log::info;
use prost::Message;
use serde::Deserialize;
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

pub fn get_index() -> VersearchIndex {
    let mut vi = VersearchIndex::new();
    let config = match envy::from_env::<Config>() {
        Ok(config) => config,
        Err(error) => panic!("{:?}", error),
    };

    info!("Loading translations from {:?}", config.translation_dir);
    for entry in fs::read_dir(config.translation_dir).unwrap() {
        let path = entry.unwrap().path();
        if path.is_file() && path.extension().map(|s| s == "pb").unwrap_or(false) {
            let translation = path
                .file_stem()
                .expect("Could not get file stem")
                .to_string_lossy()
                .to_string();
            info!("Load translation {:?} from {:?}", translation, path);
            let now = Instant::now();
            let mut file_bytes: Vec<u8> = Vec::new();
            fs::File::open(path)
                .unwrap()
                .read_to_end(&mut file_bytes)
                .unwrap();
            let data = TranslationData::decode(file_bytes).unwrap();
            info!(
                "Read {} verses in {}ms",
                data.verses.len(),
                now.elapsed().as_millis()
            );
            let now = Instant::now();
            for verse in &data.verses {
                vi.insert_verse(verse);
            }
            info!(
                "Indexed {} verses in {}ms",
                data.verses.len(),
                now.elapsed().as_millis()
            );
        }
    }

    vi
}

#[cfg(test)]
mod tests {
    use super::InterIter;

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
}
