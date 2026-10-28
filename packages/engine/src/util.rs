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
use serde::Deserialize;
use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fs;
use std::io::prelude::*;
use std::io::{self, BufRead, Seek};
use std::iter::Iterator;
use std::sync::Arc;
use std::thread;
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

/// Number of books, used to size crawl data counts
const CRAWL_BOOKS: usize = Book::Revelation as usize + 1;
/// Number of threads used to read crawl data
const CRAWL_THREADS: u64 = 8;

#[derive(Default)]
struct VerseStats {
    counts: [u32; TRANSLATION_COUNT],
    /// Interned highlight word ids
    highlights: Vec<u32>,
}

/// Assigns ids to highlight words in the order they are first seen
#[derive(Default)]
struct HighlightWords {
    ids: HashMap<String, u32>,
}

impl HighlightWords {
    fn intern(&mut self, word: String) -> u32 {
        let next = self.ids.len() as u32;
        *self.ids.entry(word).or_insert(next)
    }

    /// Returns the sorted words and a map of interned id => index in the sorted words
    fn into_sorted(self) -> (Vec<String>, Vec<u32>) {
        let mut words: Vec<_> = self.ids.into_iter().collect();
        words.sort_unstable();
        let mut sorted_ids = vec![0; words.len()];
        for (i, (_, id)) in words.iter().enumerate() {
            sorted_ids[*id as usize] = i as u32;
        }
        (words.into_iter().map(|(w, _)| w).collect(), sorted_ids)
    }
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
    highlight_words: &mut HighlightWords,
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
            let highlight = highlight_words.intern(tokenized.source.to_uppercase());
            // Create new stats entry if needed
            let entry: &mut VerseStats = wip_token_counts
                .entry(tokenized.token.clone())
                .or_insert_with(BTreeMap::new)
                .entry(vkey)
                .or_default();
            // Increment counts
            entry.counts[translation_key as usize] += 1;
            // Track highlights
            if !entry.highlights.contains(&highlight) {
                entry.highlights.push(highlight);
            }
        }
    }
}

/// Loads translation data from disk and returns the total number of documents
fn load_translation_data(
    translation_verses: &mut TranslationVerses,
    verse_counts: &mut BTreeMap<VerseKey, u64>,
    highlight_words: &mut HighlightWords,
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
    highlight_words: HighlightWords,
    wip_token_counts: WipTokenCountsMap,
) -> (Vec<ReverseIndexEntryBytes>, Vec<u8>, Vec<String>) {
    let mut build = MapBuilder::memory();
    let mut reverse_index = Vec::with_capacity(wip_token_counts.len());
    let (highlight_words, sorted_ids) = highlight_words.into_sorted();
    let mut highlights = Vec::new();

    // Consume the work-in-progress data as it is indexed to save memory
    for (i, (token, entries)) in wip_token_counts.into_iter().enumerate() {
        build.insert(token, i as u64).unwrap();

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
            // Highlights are stored in sorted order
            highlights.clear();
            highlights.extend(vs.highlights.iter().map(|id| sorted_ids[*id as usize]));
            highlights.sort_unstable();
            let highlight_index_bytes: Vec<u8> = highlights
                .iter()
                .flat_map(|id| u64::from(*id).to_be_bytes().to_vec())
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

    // Only verses which exist are counted. Map each book/chapter/verse to a slot in a dense
    // table so counting doesn't need to look up a map for every line.
    let mut slots = vec![0u32; CRAWL_BOOKS * 256 * 256];
    for (i, key) in verse_rankings.keys().enumerate() {
        if let Some(idx) =
            Book::from_i32(key.book).and_then(|b| crawl_index(b, key.chapter, key.verse))
        {
            slots[idx] = i as u32 + 1;
        }
    }
    let slots = Arc::new(slots);

    // Crawl data is large, count it in parallel chunks
    let len = fs::metadata(&crawl_data)
        .context("Could not open crawl data file")?
        .len();
    let verses = verse_rankings.len();
    let handles: Vec<_> = (0..CRAWL_THREADS)
        .map(|i| {
            let (path, slots) = (crawl_data.clone(), Arc::clone(&slots));
            let range = (len * i / CRAWL_THREADS, len * (i + 1) / CRAWL_THREADS);
            thread::spawn(move || count_crawl_chunk(&path, range, &slots, verses))
        })
        .collect();
    let mut counts = vec![0u64; verses];
    for handle in handles {
        let chunk_counts = handle
            .join()
            .map_err(|_| anyhow!("Crawl data thread panicked"))??;
        for (count, chunk_count) in counts.iter_mut().zip(chunk_counts) {
            *count += chunk_count;
        }
    }

    for (count, crawl_count) in verse_rankings.values_mut().zip(counts) {
        *count += crawl_count;
    }

    Ok(())
}

/// Index into the dense crawl data slots table
fn crawl_index(book: Book, chapter: u32, verse: u32) -> Option<usize> {
    if chapter < 256 && verse < 256 {
        Some(((book as usize * 256) + chapter as usize) * 256 + verse as usize)
    } else {
        None
    }
}

/// Counts references to each verse slot in the lines of crawl data starting in the given byte
/// range
fn count_crawl_chunk(
    path: &str,
    (start, end): (u64, u64),
    slots: &[u32],
    verses: usize,
) -> Result<Vec<u64>> {
    let mut counts = vec![0u64; verses];
    let mut file = fs::File::open(path).context("Could not open crawl data file")?;
    // Start reading at the start of the first line which starts within the range
    let mut pos = start.saturating_sub(1);
    file.seek(io::SeekFrom::Start(pos))
        .context("Could not seek crawl data")?;
    let mut reader = io::BufReader::with_capacity(1 << 20, file);
    let mut buf = Vec::new();
    if start > 0 {
        pos += reader
            .read_until(b'\n', &mut buf)
            .context("Could not read crawl data")? as u64;
    }
    while pos < end {
        buf.clear();
        let read = reader
            .read_until(b'\n', &mut buf)
            .context("Could not read crawl data")?;
        if read == 0 {
            break;
        }
        pos += read as u64;
        // Strip line endings and skip invalid UTF-8 like `BufRead::lines` does
        if buf.last() == Some(&b'\n') {
            buf.pop();
            if buf.last() == Some(&b'\r') {
                buf.pop();
            }
        }
        let slot = std::str::from_utf8(&buf)
            .ok()
            .and_then(parse_crawl_line)
            .and_then(|(book, chapter, verse)| {
                crawl_index(Book::from_string(book).ok()?, chapter, verse)
            })
            .map_or(0, |idx| slots[idx]);
        if slot != 0 {
            counts[slot as usize - 1] += 1;
        }
    }
    Ok(counts)
}

/// Splits 1 to 3 trailing ASCII digits off of a string
fn split_trailing_number(s: &str) -> Option<(&str, u32)> {
    let digits = s.bytes().rev().take_while(u8::is_ascii_digit).count();
    if digits == 0 || digits > 3 {
        return None;
    }
    let (rest, number) = s.split_at(s.len() - digits);
    Some((rest, number.parse().ok()?))
}

/// Parses a line of crawl data of the form `BOOK CHAPTER:VERSE`, equivalent to matching
/// `^(.+)\s+(\d{1,3}):(\d{1,3})$`
fn parse_crawl_line(line: &str) -> Option<(&str, u32, u32)> {
    let (rest, verse) = split_trailing_number(line)?;
    let (rest, chapter) = split_trailing_number(rest.strip_suffix(':')?)?;
    // Greedy `(.+)\s+` leaves exactly one whitespace character before the chapter
    let mut chars = rest.chars();
    if !chars.next_back()?.is_whitespace() || chars.as_str().is_empty() {
        return None;
    }
    Some((chars.as_str(), chapter, verse))
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
    let mut highlight_words = HighlightWords::default();

    load_translation_data(
        &mut translation_verses,
        &mut verse_counts,
        &mut highlight_words,
        &mut wip_token_counts,
    )
    .expect("Could not load data from disk");

    let now = Instant::now();

    let (reverse_index_bytes, fst_bytes, highlight_words) =
        build_reverse_index(highlight_words, wip_token_counts);

    info!("Indexed data {}ms", now.elapsed().as_millis());

    let (translation_verses_bytes, translation_verses_strings) =
        build_translation_verses_bytes(&translation_verses)
            .expect("Could not construct translation verses fst map");
    drop(translation_verses);

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
    fn test_count_crawl_chunks() {
        let data = "GENESIS 1:1\nJOHN 3:16\r\nJOHN 3:16\nnot a verse\n\nJOHN 3:17\nGENESIS 1:1";
        let path = std::env::temp_dir().join(format!("crawl-test-{}.txt", std::process::id()));
        fs::write(&path, data).unwrap();
        let path_str = path.to_str().unwrap();
        let mut slots = vec![0u32; CRAWL_BOOKS * 256 * 256];
        slots[crawl_index(Book::Genesis, 1, 1).unwrap()] = 1;
        slots[crawl_index(Book::John, 3, 16).unwrap()] = 2;
        let len = data.len() as u64;
        let count = |start, end| count_crawl_chunk(path_str, (start, end), &slots, 2).unwrap();
        assert_eq!(count(0, len), vec![2, 2]);
        // Every line is counted exactly once no matter where chunks are split
        for a in 0..=len {
            for b in a..=len {
                let (x, y, z) = (count(0, a), count(a, b), count(b, len));
                let total: Vec<u64> = (0..2).map(|i| x[i] + y[i] + z[i]).collect();
                assert_eq!(total, vec![2, 2], "split at {} and {}", a, b);
            }
        }
        fs::remove_file(path).unwrap();
    }

    #[test]
    fn test_parse_crawl_line() {
        // Must behave exactly like the regex it replaced
        let re = regex::Regex::new(r"^(.+)\s+(\d{1,3}):(\d{1,3})$").unwrap();
        for line in &[
            "JOHN 3:16",
            "SONG OF SOLOMON 2:1",
            "FIRST JOHN 1:9",
            "JOHN  3:16",
            "JOHN\t3:16",
            "JOHN\u{a0}3:16",
            " 3:16",
            "  3:16",
            "3:16",
            "JOHN 3:",
            "JOHN :16",
            "JOHN 3:1234",
            "JOHN 1234:5",
            "JOHN 012:016",
            "JOHN 3:16 ",
            "JOHN 3 16",
            "JOHN 3::16",
            "JOHN 3:16:1",
            "JOHN ٣:16",
            "JOHN 3:1٣",
            "JOHN ٣16:5",
            "SECOND JOHN 3",
            "",
        ] {
            let expected = re.captures(line).and_then(|caps| {
                Some((
                    caps.get(1)?.as_str(),
                    caps.get(2)?.as_str().parse::<u32>().ok()?,
                    caps.get(3)?.as_str().parse::<u32>().ok()?,
                ))
            });
            assert_eq!(parse_crawl_line(line), expected, "line {:?}", line);
        }
    }

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
