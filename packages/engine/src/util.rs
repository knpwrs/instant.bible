use crate::proto::data::{decode_translation_data, Book, Translation, VerseKey, VerseText};
use crate::proto::engine::{
    decode_index_data, IndexData as IndexDataProtoStruct,
    ReverseIndexEntry as ReverseIndexEntryBytes,
};
use crate::TRANSLATION_COUNT;
use anyhow::{anyhow, Context, Result};
use fst::MapBuilder;
use lazy_static::lazy_static;
use log::info;
use regex::Regex;
use serde::Deserialize;
use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io::prelude::*;
use std::io::{self, BufRead};
use std::iter::Iterator;
use std::time::Instant;

#[cfg_attr(test, derive(Debug))]
#[derive(Deserialize)]
pub struct Config {
    pub translation_dir: Option<String>,
    pub crawl_data: Option<String>,
}

#[cfg_attr(test, derive(Debug))]
#[derive(Clone, Hash, PartialEq, Eq)]
pub struct Tokenized {
    pub source: String,
    pub token: String,
}

impl Ord for Tokenized {
    fn cmp(&self, other: &Self) -> Ordering {
        self.token.cmp(&other.token)
    }
}

impl PartialOrd for Tokenized {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct VerseStats {
    counts: Vec<usize>,
    highlights: BTreeSet<String>,
}

type TranslationVerses = BTreeMap<Translation, BTreeMap<VerseKey, String>>;

lazy_static! {
    static ref STOP_WORDS: BTreeSet<&'static str> = {
        let mut s = BTreeSet::new();
        s.insert("THE");
        s.insert("AND");
        s.insert("OF");
        s.insert("TO");
        s.insert("IN");
        // s.insert("I"); // Do not use "I" as a stop word since it prevents searching for "I AM"
        s.insert("A");
        s.insert("IS");
        s.insert("BE");
        s.insert("IT");
        s.insert("ON");
        s
    };
}

pub fn tokenize(input: &str) -> Vec<Tokenized> {
    input
        .split_whitespace()
        .map(|s| Tokenized {
            token: s
                .chars()
                // Keeping only alphanumeric characters lets users search without
                // concern for apostrophes and the like
                .filter(|c| c.is_ascii_alphanumeric())
                .collect::<String>()
                .to_uppercase(),
            source: s
                .chars()
                .enumerate()
                // Like tokens but with apostophes and commas (except trailing commas)
                .filter(|(i, c)| {
                    c.is_ascii_alphanumeric() || *c == '\'' || (*c == ',' && *i != s.len() - 1)
                })
                .map(|(_i, c)| c)
                .collect::<String>(),
        })
        .collect()
}

fn get_config() -> Result<Config> {
    let conf = envy::from_env::<Config>().context("envy failed to read environment")?;
    Ok(conf)
}

/// Given a translation id and a verse key, generates a sequence of bytes which
/// can be used as a key into an FST map
pub fn translation_verses_bytes_key(tidx: u8, vkey: &VerseKey) -> Vec<u8> {
    let capacity = std::mem::size_of::<u8>() + VerseKey::get_byte_size();
    let mut v = Vec::with_capacity(capacity);
    v.extend(&tidx.to_be_bytes());
    v.extend(&vkey.to_be_bytes());
    v
}

/// Reads and returns the bytes of a file located at the given path
fn read_file_bytes(path: &std::path::PathBuf) -> Result<Vec<u8>> {
    let mut file_bytes = Vec::new();
    fs::File::open(path)
        .context("Could not open file")?
        .read_to_end(&mut file_bytes)
        .context("Could not read file")?;
    Ok(file_bytes)
}

// Stores work-in-progress token counts per verse and translation
type WipTokenCountsMap = BTreeMap<String, BTreeMap<VerseKey, VerseStats>>;

/// Performs initial processing of verses read from disk
fn process_verses(
    translation_key: Translation,
    verses: &[VerseText],
    translation_verses: &mut TranslationVerses,
    verse_counts: &mut BTreeMap<VerseKey, u64>,
    highlight_words: &mut BTreeSet<String>,
    wip_token_counts: &mut BTreeMap<String, BTreeMap<VerseKey, VerseStats>>,
) {
    for verse in verses {
        let vkey = verse.key.expect("Missing verse key");
        let verse_tokens = tokenize(&verse.text);
        translation_verses
            .entry(translation_key)
            .or_insert_with(BTreeMap::new)
            .entry(vkey)
            .or_insert_with(|| verse.text.clone());
        verse_counts.entry(vkey).or_insert(0);
        // Count up tokens
        for tokenized in verse_tokens
            .iter()
            .filter(|t| !STOP_WORDS.contains(t.token.as_str()))
        {
            // Save word to get a highlight id later
            highlight_words.insert(tokenized.source.to_uppercase());
            // Create new stats entry if needed
            let entry = wip_token_counts
                .entry(tokenized.token.clone())
                .or_insert_with(BTreeMap::new)
                .entry(vkey.clone())
                .or_insert_with(|| VerseStats {
                    counts: vec![0; TRANSLATION_COUNT],
                    highlights: BTreeSet::new(),
                });
            // Increment counts
            entry.counts[translation_key as usize] += 1;
            // Track highlights
            entry.highlights.insert(tokenized.source.to_uppercase());
        }
    }
}

/// Loads translation data from disk and returns the total number of documents
fn load_translation_data(
    translation_verses: &mut TranslationVerses,
    verse_counts: &mut BTreeMap<VerseKey, u64>,
    highlight_words: &mut BTreeSet<String>,
    wip_token_counts: &mut WipTokenCountsMap,
) -> Result<()> {
    let config = get_config().context("load_translation_data")?;
    info!("Loading translations from {:?}", config.translation_dir);
    let translation_dir = config
        .translation_dir
        .ok_or_else(|| anyhow!("Environment TRANSLATION_DIR missing"))?;

    let mut total_docs: usize = 0;

    for entry in
        fs::read_dir(translation_dir).context("Could not read translation data directory")?
    {
        let path = entry
            .context("Could not convert translation data entry to path")?
            .path();
        if path.is_file() && path.extension().map(|s| s == "pb").unwrap_or(false) {
            let translation_name = path
                .file_stem()
                .expect("Could not get file stem")
                .to_string_lossy()
                .to_string();
            info!("Load translation {:?} from {:?}", translation_name, path);
            let now = Instant::now();
            let file_bytes = read_file_bytes(&path).expect("Could not read protobuf file");
            let data = decode_translation_data(&*file_bytes).expect("Could not parse protobuf");
            let translation_key =
                Translation::from_i32(data.translation).expect("Invalid translation field value");
            info!(
                "Read {} verses in {}ms",
                data.verses.len(),
                now.elapsed().as_millis()
            );
            total_docs = total_docs.max(data.verses.len());
            let now = Instant::now();
            process_verses(
                translation_key,
                &data.verses,
                translation_verses,
                verse_counts,
                highlight_words,
                wip_token_counts,
            );
            info!(
                "Processed {} verses in {}ms",
                data.verses.len(),
                now.elapsed().as_millis()
            );
        }
    }

    info!("Total verses loaded (all translations): {}", total_docs);

    Ok(())
}

/// Build and return a reverse index, fst bytes, and vector of highlight words
fn build_reverse_index(
    highlight_words: &BTreeSet<String>,
    wip_token_counts: &WipTokenCountsMap,
) -> (Vec<ReverseIndexEntryBytes>, Vec<u8>, Vec<String>) {
    let mut build = MapBuilder::memory();
    let mut reverse_index = Vec::with_capacity(wip_token_counts.len());
    let highlight_words: Vec<_> = highlight_words.iter().cloned().collect();

    for (i, (token, entries)) in wip_token_counts.iter().enumerate() {
        build.insert(token.clone(), i as u64).unwrap();

        let mut map_builder = MapBuilder::memory();
        let mut counts_map_data = Vec::new();
        let mut highlights_map_data = Vec::new();

        for (i, (key, vs)) in entries.iter().enumerate() {
            let counts_bytes: Vec<u8> = vs
                .counts
                .iter()
                .flat_map(|c| {
                    (*c as u64)
                        .to_be_bytes()
                        .iter()
                        .copied()
                        .collect::<Vec<u8>>()
                })
                .collect();
            let highlight_index_bytes: Vec<u8> = vs
                .highlights
                .iter()
                .flat_map(|s| {
                    (highlight_words
                        .binary_search(s)
                        .expect("Could not find index for highlight entry")
                        as u64)
                        .to_be_bytes()
                        .iter()
                        .copied()
                        .collect::<Vec<u8>>()
                })
                .collect();

            let key_bytes = key.to_be_bytes();
            map_builder
                .insert([key_bytes[0], key_bytes[1], key_bytes[2]], i as u64)
                .expect("Could not insert into reverse index entry map");
            counts_map_data.push(counts_bytes);
            highlights_map_data.push(highlight_index_bytes);
        }

        reverse_index.push(ReverseIndexEntryBytes {
            map_bytes: map_builder
                .into_inner()
                .expect("Could not construct counts map bytes"),
            counts_map_data,
            highlights_map_data,
        });
    }

    let fst_bytes = build.into_inner().expect("Could not flush bytes for FST");
    info!("FST compiled: {} bytes", fst_bytes.len());
    info!("Stored {} words for highlighting", highlight_words.len());

    (reverse_index, fst_bytes, highlight_words)
}

fn build_translation_verses_bytes(
    translation_verses: &TranslationVerses,
) -> Result<(Vec<u8>, Vec<String>)> {
    let mut strings = Vec::new();
    let mut build = MapBuilder::memory();

    for (tidx, verses) in translation_verses.iter() {
        for (verse_key, text) in verses {
            let key = translation_verses_bytes_key(*tidx as u8, verse_key);
            build
                .insert(key, strings.len() as u64)
                .context("Could not insert into translation verses map builder")?;
            strings.push(text.clone());
        }
    }

    let bytes = build
        .into_inner()
        .context("Could not build translation verses fst bytes")?;

    Ok((bytes, strings))
}

/// Loads crawl data from disk
fn load_crawl_data(verse_rankings: &mut BTreeMap<VerseKey, u64>) -> Result<()> {
    let config = get_config().context("load_crawl_data")?;

    let crawl_data = config
        .crawl_data
        .ok_or_else(|| anyhow!("Environment CRAWL_DATA missing"))?;

    let re = Regex::new(r"^(.+)\s+(\d{1,3}):(\d{1,3})$")
        .context("Could not compile regex for parsing crawl data")?;

    // I really did try to avoid this...
    let file = fs::File::open(crawl_data).context("Could not open crawl data file")?;
    for line in io::BufReader::new(file).lines().filter_map(Result::ok) {
        if let Some(caps) = re.captures(&line) {
            if let (Some(book), Some(chapter), Some(verse)) =
                (caps.get(1), caps.get(2), caps.get(3))
            {
                if let (Ok(book), Ok(chapter), Ok(verse)) = (
                    Book::from_string(book.as_str()),
                    chapter.as_str().parse::<u32>(),
                    verse.as_str().parse::<u32>(),
                ) {
                    let key = VerseKey {
                        book: book as i32,
                        chapter,
                        verse,
                    };
                    verse_rankings.entry(key).and_modify(|count| {
                        *count += 1;
                    });
                }
            }
        }
    }

    Ok(())
}

/// Processes crawl data after it is loaded and produce an FST map of verse => count
fn build_verse_counts_fst(verse_counts: &BTreeMap<VerseKey, u64>) -> Result<Vec<u8>> {
    let mut builder = MapBuilder::memory();

    for (key, count) in verse_counts {
        let bytes = key.to_be_bytes();
        builder
            .insert(bytes, *count)
            .context("Could not insert into verse counts fst")?;
    }

    builder
        .into_inner()
        .context("Could not build verse counts bytes")
}

/// Creates and returns a search index
pub fn create_index_proto_struct() -> IndexDataProtoStruct {
    let start = Instant::now();

    let mut wip_token_counts = BTreeMap::new();
    let mut verse_counts = BTreeMap::new();
    let mut translation_verses: TranslationVerses = BTreeMap::new();
    let mut highlight_words = BTreeSet::new();

    load_translation_data(
        &mut translation_verses,
        &mut verse_counts,
        &mut highlight_words,
        &mut wip_token_counts,
    )
    .expect("Could not load data from disk");

    let now = Instant::now();

    let (reverse_index_bytes, fst_bytes, highlight_words) =
        build_reverse_index(&highlight_words, &wip_token_counts);

    info!("Indexed data {}ms", now.elapsed().as_millis());

    let (translation_verses_bytes, translation_verses_strings) =
        build_translation_verses_bytes(&translation_verses)
            .expect("Could not construct translation verses fst map");

    let now = Instant::now();
    info!("Building popularlity index");
    load_crawl_data(&mut verse_counts).expect("Could not load crawl data");
    let popularity_bytes =
        build_verse_counts_fst(&verse_counts).expect("Could not construct popularity index");
    info!(
        "Done building popularity index for {} verses in {}ms ({} bytes)",
        verse_counts.len(),
        now.elapsed().as_millis(),
        popularity_bytes.len()
    );

    info!(
        "get_index_proto_struct done in {}ms",
        start.elapsed().as_millis()
    );

    IndexDataProtoStruct {
        fst: fst_bytes,
        reverse_index_entries: reverse_index_bytes,
        highlight_words,
        translation_verses: translation_verses_bytes,
        translation_verses_strings,
        popularity: popularity_bytes,
    }
}

pub fn get_index_proto_struct_from_disk() -> Result<IndexDataProtoStruct> {
    let bytes = read_file_bytes(&std::path::PathBuf::from("index.pb"))?;
    decode_index_data(&bytes).context("get_index_proto_struct_from_disk")
}

pub fn get_or_create_index_proto_struct() -> IndexDataProtoStruct {
    match get_index_proto_struct_from_disk() {
        Ok(data) => data,
        _ => create_index_proto_struct(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        assert_eq!(
            tokenize("hello, world!"),
            vec![
                Tokenized {
                    source: "hello".to_string(),
                    token: "HELLO".to_string()
                },
                Tokenized {
                    source: "world".to_string(),
                    token: "WORLD".to_string()
                }
            ]
        );
        assert_eq!(
            tokenize("It's all good in the neighborhood which is... good"),
            vec![
                Tokenized {
                    source: "It's".to_string(),
                    token: "ITS".to_string()
                },
                Tokenized {
                    source: "all".to_string(),
                    token: "ALL".to_string(),
                },
                Tokenized {
                    source: "good".to_string(),
                    token: "GOOD".to_string()
                },
                Tokenized {
                    source: "in".to_string(),
                    token: "IN".to_string()
                },
                Tokenized {
                    source: "the".to_string(),
                    token: "THE".to_string()
                },
                Tokenized {
                    source: "neighborhood".to_string(),
                    token: "NEIGHBORHOOD".to_string()
                },
                Tokenized {
                    source: "which".to_string(),
                    token: "WHICH".to_string()
                },
                Tokenized {
                    source: "is".to_string(),
                    token: "IS".to_string()
                },
                Tokenized {
                    source: "good".to_string(),
                    token: "GOOD".to_string()
                },
            ]
        );
    }
}
