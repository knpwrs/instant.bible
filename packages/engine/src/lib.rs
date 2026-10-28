mod data;
pub mod proto;
mod typo;
pub mod util;

use crate::proto::engine::IndexData;
use anyhow::{anyhow, bail, Context, Result};
use data::{RawIndex, ReverseIndex, StringTable, VerseId};
use fst::{IntoStreamer, Map as FstMap, Streamer};
use proto::data::{Translation, VerseKey};
use proto::service::{
    response::{verse_result::Ranking as ServiceRanking, VerseResult},
    Response as ServiceResponse,
};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::hash::{BuildHasherDefault, Hasher};
use typo::Typo;
use util::{tokenize, Tokenized};
// Previously this module was using wasm-timer, however, it turns out wasm-timer's Instant::now()
// doesn't work in web workers https://github.com/tomaka/wasm-timer/issues/12
// use wasm_timer::Instant;

pub use util::Config;

static MAX_RESULTS: usize = 20;
static TYPO_1_LEN: usize = 4;
static TYPO_2_LEN: usize = 8;
pub const TRANSLATION_COUNT: usize = Translation::Total as usize;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
enum MatchType {
    Exact = 0,
    Prefix = 1,
    Typo = 2,
}

/// Hashes integer keys with a single multiplication, much faster than the default SipHash
#[derive(Default)]
struct IntHasher(u64);

impl Hasher for IntHasher {
    fn finish(&self) -> u64 {
        self.0
    }

    fn write(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.write_u64(u64::from(*byte) ^ self.0.rotate_left(8));
        }
    }

    fn write_u64(&mut self, n: u64) {
        self.0 = n.wrapping_mul(0x9e37_79b9_7f4a_7c15);
    }

    fn write_usize(&mut self, n: usize) {
        self.write_u64(n as u64);
    }
}

type IntHashMap<K, V> = HashMap<K, V, BuildHasherDefault<IntHasher>>;

/// A reverse index entry (token) found while traversing the FST
struct FoundEntry {
    entry: usize,
    match_type: MatchType,
    qidx: usize,
}

#[derive(Clone, Copy, Default)]
struct Ranking {
    /// The number of distinct query words matched
    query_words: i32,
    exact: i32,
    typos: i32,
}

impl Ranking {
    /// Orders rankings best first: more query words matched, then more exact matches, then fewer
    /// typos
    fn cmp_best_first(&self, other: &Self) -> Ordering {
        other
            .query_words
            .cmp(&self.query_words)
            .then(other.exact.cmp(&self.exact))
            .then(self.typos.cmp(&other.typos))
    }
}

/// A verse being scored
#[derive(Clone, Default)]
struct Candidate {
    rankings: [Ranking; TRANSLATION_COUNT],
    /// The query word (plus one) currently being counted toward `rankings[_].query_words` and the
    /// translations it has already been counted for
    counting_qidx: u32,
    counted: u8,
    /// Head of this candidate's list of matched postings in `Scored::matches` (plus one)
    matches_head: u32,
}

impl Candidate {
    fn top_ranking(&self) -> (usize, &Ranking) {
        self.rankings
            .iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| a.cmp_best_first(b))
            .unwrap()
    }

    /// Key which orders candidates best first: most query words matched (in any translation),
    /// then most popular, then by verse (candidate index order is verse order)
    fn sort_key(&self, idx: usize, popularity: u64) -> u128 {
        let query_words = self
            .rankings
            .iter()
            .map(|r| r.query_words)
            .max()
            .unwrap_or(0);
        (u128::from(u32::MAX - query_words as u32) << 96)
            | (u128::from(u64::MAX - popularity) << 32)
            | idx as u128
    }
}

struct Scored<'a> {
    /// Candidate => verse
    verses: &'a [VerseId],
    candidates: Vec<Candidate>,
    /// Linked lists of (posting, next) matched for each candidate, used for highlighting
    matches: Vec<(u32, u32)>,
}

pub struct VersearchIndex {
    /// Token => reverse index entry, used for typo searches
    fst_map: FstMap<Vec<u8>>,
    /// Sorted tokens and their reverse index entries, used for prefix searches
    tokens: StringTable,
    token_entries: Vec<u32>,
    reverse_index: ReverseIndex,
    highlight_words: StringTable,
    /// VerseId => packed verse key (book << 16 | chapter << 8 | verse)
    verse_keys: Vec<u32>,
    /// VerseId => popularity
    verse_popularity: Vec<u64>,
    /// VerseId * TRANSLATION_COUNT + Translation => text
    verse_texts: StringTable,
}

fn pack_key(bytes: &[u8]) -> Result<u32> {
    match *bytes {
        [book, chapter, verse] => Ok(u32::from_be_bytes([0, book, chapter, verse])),
        _ => Err(anyhow!("Invalid verse key length {}", bytes.len())),
    }
}

fn read_u64s<'a>(bytes: &'a [u8]) -> impl Iterator<Item = u64> + 'a {
    bytes.chunks_exact(8).map(|chunk| {
        let mut buf = [0u8; 8];
        buf.copy_from_slice(chunk);
        u64::from_be_bytes(buf)
    })
}

/// Calls `f(i, j)` for every `a[i] == b[j]`, given two sorted slices without duplicates where
/// `a` is the shorter slice. Gallops through `b` so this is fast when `b` is much longer.
#[inline]
fn intersect<F: FnMut(usize, usize)>(a: &[VerseId], b: &[VerseId], mut f: F) {
    let mut lo = 0;
    for (i, v) in a.iter().enumerate() {
        let rest = &b[lo..];
        let mut hi = 1;
        while hi < rest.len() && rest[hi - 1] < *v {
            hi *= 2;
        }
        let start = hi / 2;
        match rest[start..hi.min(rest.len())].binary_search(v) {
            Ok(j) => {
                f(i, lo + start + j);
                lo += start + j + 1;
            }
            Err(j) => lo += start + j,
        }
        if lo >= b.len() {
            break;
        }
    }
}

impl VersearchIndex {
    #[allow(clippy::new_without_default)]
    pub fn from_index_data_proto_struct(index_data: IndexData) -> Self {
        Self::from_raw(&RawIndex::from_proto(&index_data)).expect("Could not load index data")
    }

    /// Loads the index from `index.pb` in the working directory, or creates it if it can't be
    /// loaded
    pub fn get_or_create() -> Self {
        std::fs::read("index.pb")
            .map_err(anyhow::Error::from)
            .and_then(|bytes| Self::from_index_bytes(&bytes))
            .unwrap_or_else(|_| {
                Self::from_index_data_proto_struct(util::create_index_proto_struct())
            })
    }

    /// Loads the index directly from an encoded `IndexData` protobuf. This is faster and uses
    /// far less memory than decoding the protobuf first.
    pub fn from_index_bytes(bytes: &[u8]) -> Result<Self> {
        Self::from_raw(&RawIndex::from_bytes(bytes)?)
    }

    fn from_raw(raw: &RawIndex) -> Result<Self> {
        // Every verse is normally listed in the popularity and translation verses maps which are
        // much faster to read than every reverse index entry, fall back to reading the reverse
        // index entries if they turn out to be incomplete
        let mut verse_keys = Self::listed_verse_keys(raw)?;
        let reverse_index = match Self::build_reverse_index(raw, &verse_keys)? {
            Some(reverse_index) => reverse_index,
            None => {
                verse_keys = Self::indexed_verse_keys(raw)?;
                Self::build_reverse_index(raw, &verse_keys)?
                    .ok_or_else(|| anyhow!("Could not assign verse ids"))?
            }
        };
        let verse_id = |key: u32| verse_keys.binary_search(&key).ok().map(|id| id as VerseId);

        // Popularity of each verse
        let mut verse_popularity = vec![0; verse_keys.len()];
        let popularity =
            FstMap::new(raw.popularity).context("Could not load map from popularity bytes")?;
        let mut stream = popularity.stream();
        while let Some((key, count)) = stream.next() {
            if let Some(verse) = verse_id(pack_key(key)?) {
                verse_popularity[verse as usize] = count;
            }
        }

        // Text of each verse in each translation
        let mut text_idx = vec![None; verse_keys.len() * TRANSLATION_COUNT];
        let translation_verses =
            FstMap::new(raw.translation_verses).context("Could not load map from verses bytes")?;
        let mut stream = translation_verses.stream();
        let mut text_bytes = 0;
        while let Some((key, idx)) = stream.next() {
            let (tidx, key) = key
                .split_first()
                .ok_or_else(|| anyhow!("Invalid translation verses key"))?;
            let tidx = *tidx as usize;
            if tidx >= TRANSLATION_COUNT {
                continue;
            }
            let verse = verse_id(pack_key(key)?);
            let text = raw.translation_verses_strings.get(idx as usize);
            if let (Some(verse), Some(text)) = (verse, text) {
                text_idx[verse as usize * TRANSLATION_COUNT + tidx] = Some(text);
                text_bytes += text.len();
            }
        }
        let mut verse_texts = StringTable::with_capacity(text_idx.len(), text_bytes);
        for text in text_idx {
            verse_texts.push(text.map_or("", |s| s));
        }

        // Sorted tokens
        let fst_map = FstMap::new(raw.fst.to_vec()).context("Could not load map from FST bytes")?;
        let mut tokens = StringTable::with_capacity(fst_map.len(), 0);
        let mut token_entries = Vec::with_capacity(fst_map.len());
        let mut stream = fst_map.stream();
        while let Some((token, entry)) = stream.next() {
            if entry as usize >= raw.entries.len() {
                bail!("Invalid reverse index entry for token");
            }
            tokens.push(std::str::from_utf8(token).context("Invalid UTF-8 in token")?);
            token_entries.push(entry as u32);
        }
        tokens.shrink_to_fit();

        Ok(VersearchIndex {
            fst_map,
            tokens,
            token_entries,
            reverse_index,
            highlight_words: raw.highlight_words.iter().copied().collect(),
            verse_keys,
            verse_popularity,
            verse_texts,
        })
    }

    /// Sorts, deduplicates, and checks verse keys, the index of each key is its `VerseId`
    fn finish_verse_keys(mut verse_keys: Vec<u32>) -> Result<Vec<u32>> {
        verse_keys.sort_unstable();
        verse_keys.dedup();
        verse_keys.shrink_to_fit();
        // Scoring stores candidate indices plus one as VerseIds, so there must be fewer verses
        // than VerseId::MAX
        if verse_keys.len() > VerseId::MAX as usize {
            bail!("Too many verses in index: {}", verse_keys.len());
        }
        Ok(verse_keys)
    }

    /// Verse keys listed in the popularity and translation verses maps
    fn listed_verse_keys(raw: &RawIndex) -> Result<Vec<u32>> {
        let mut verse_keys = Vec::new();
        let popularity =
            FstMap::new(raw.popularity).context("Could not load map from popularity bytes")?;
        let mut stream = popularity.keys();
        while let Some(key) = stream.next() {
            verse_keys.push(pack_key(key)?);
        }
        let translation_verses =
            FstMap::new(raw.translation_verses).context("Could not load map from verses bytes")?;
        let mut stream = translation_verses.keys();
        while let Some(key) = stream.next() {
            verse_keys.push(pack_key(key.get(1..).unwrap_or_default())?);
        }
        Self::finish_verse_keys(verse_keys)
    }

    /// Verse keys found in the reverse index
    fn indexed_verse_keys(raw: &RawIndex) -> Result<Vec<u32>> {
        let mut counts = Vec::new();
        let mut highlights = Vec::new();
        let mut verse_keys = Vec::new();
        for entry in &raw.entries {
            let map = FstMap::new(entry.read(&mut counts, &mut highlights)?)
                .context("Could not load reverse index entry map")?;
            let mut stream = map.keys();
            while let Some(key) = stream.next() {
                verse_keys.push(pack_key(key)?);
            }
        }
        Self::finish_verse_keys(verse_keys)
    }

    /// Builds the reverse index, returns `None` if it contains a verse missing from `verse_keys`
    fn build_reverse_index(raw: &RawIndex, verse_keys: &[u32]) -> Result<Option<ReverseIndex>> {
        let mut counts = Vec::new();
        let mut highlights = Vec::new();
        let mut highlight_ids = Vec::new();
        let mut reverse_index = ReverseIndex::with_capacity(raw.entries.len(), 0);
        for entry in &raw.entries {
            let map = FstMap::new(entry.read(&mut counts, &mut highlights)?)
                .context("Could not load reverse index entry map")?;
            let mut stream = map.stream();
            while let Some((key, idx)) = stream.next() {
                let verse = match verse_keys.binary_search(&pack_key(key)?) {
                    Ok(id) => id as VerseId,
                    Err(_) => return Ok(None),
                };
                let idx = idx as usize;
                let entry_counts = counts
                    .get(idx)
                    .ok_or_else(|| anyhow!("Missing reverse index counts"))?;
                if entry_counts.len() > TRANSLATION_COUNT * 8 {
                    bail!("Reverse index entry has too many translations");
                }
                let translations = read_u64s(entry_counts)
                    .enumerate()
                    .filter(|(_, count)| *count > 0)
                    .fold(0u8, |mask, (i, _)| mask | 1 << i);
                highlight_ids.clear();
                for id in read_u64s(highlights.get(idx).map_or(&[][..], |h| h)) {
                    if id as usize >= raw.highlight_words.len() {
                        bail!("Invalid highlight word index");
                    }
                    highlight_ids.push(id as u32);
                }
                reverse_index.push_posting(verse, translations, &highlight_ids);
            }
            reverse_index.finish_entry();
        }
        reverse_index.shrink_to_fit();
        Ok(Some(reverse_index))
    }

    #[inline]
    fn traverse_fst(&self, tokens: &[Tokenized]) -> Vec<FoundEntry> {
        let mut found: Vec<FoundEntry> = Vec::new();
        let mut found_positions: IntHashMap<usize, usize> = IntHashMap::default();
        // (token length, token == query token, entry)
        let mut results: Vec<(usize, bool, usize)> = Vec::new();

        for (qidx, Tokenized { token, .. }) in tokens.iter().enumerate() {
            results.clear();
            let query = token.as_bytes();

            // Attempt a prefix search, tokens with a common prefix are contiguous in the sorted
            // token table
            let start = self.tokens.partition_point(|t| t.as_bytes() < query);
            let end = self
                .tokens
                .partition_point(|t| t.as_bytes() < query || t.as_bytes().starts_with(query));
            results.extend((start..end).map(|i| {
                let t = self.tokens.get(i).as_bytes();
                (t.len(), t == query, self.token_entries[i] as usize)
            }));

            // If nothing was found in the prefix search then this token was a typo
            let is_typo = results.is_empty() && token.len() >= TYPO_1_LEN;
            if is_typo {
                let distance = if token.len() >= TYPO_2_LEN { 2 } else { 1 };
                let mut stream = self
                    .fst_map
                    .search(Typo::new(token, distance))
                    .into_stream();
                while let Some((t, rid)) = stream.next() {
                    results.push((t.len(), t == query, rid as usize));
                }
            }

            // Results are in lexicographical order, find the first shortest result
            let shortest = results
                .iter()
                .enumerate()
                .min_by_key(|(_, (len, _, _))| *len)
                .map(|(i, _)| i);

            // Process found tokens
            for (i, (_, is_equal, rid)) in results.iter().enumerate() {
                let position = *found_positions.entry(*rid).or_insert_with(|| {
                    found.push(FoundEntry {
                        entry: *rid,
                        match_type: if is_typo {
                            MatchType::Typo
                        } else {
                            MatchType::Prefix
                        },
                        qidx,
                    });
                    found.len() - 1
                });
                // This is an exact result if
                //   1. The result token matches the query token OR this is the shortest result
                //      token
                //   2. The token length is greater than 1
                if (*is_equal || Some(i) == shortest) && token.len() > 1 {
                    found[position].match_type = MatchType::Exact;
                }
            }
        }

        // Entries are in query word order which `score_results` relies on
        found
    }

    #[inline]
    fn score_results(&self, found: &[FoundEntry]) -> Scored<'_> {
        let mut priority_lists: Vec<_> = found.iter().collect();
        priority_lists.sort_by(|a, b| {
            // Prefer exact matches, then order by matches ascending
            a.match_type
                .cmp(&b.match_type)
                .then(
                    self.reverse_index
                        .len(a.entry)
                        .cmp(&self.reverse_index.len(b.entry)),
                )
                .then(a.entry.cmp(&b.entry))
        });
        // Pick a token list to use as result candidates. First, try to find a list with >= 3x
        // max results, then >= 2x, then >= 1x, and fall back to the list with the most results.
        let candidates_list = [3, 2, 1]
            .iter()
            .find_map(|m| {
                priority_lists
                    .iter()
                    .find(|l| self.reverse_index.len(l.entry) >= MAX_RESULTS * m)
            })
            .unwrap_or_else(|| priority_lists.last().unwrap());

        let candidate_verses = self.reverse_index.verses(candidates_list.entry);
        let mut scored = Scored {
            verses: candidate_verses,
            candidates: vec![Candidate::default(); candidate_verses.len()],
            matches: Vec::with_capacity(candidate_verses.len() * 2),
        };

        // Score each found index entry (query word) against the candidate verses. Entries which
        // are not much longer than the candidate list are scanned with a dense VerseId =>
        // candidate lookup table; candidates are binary searched in longer entries.
        let mut candidate_slots: Option<Vec<VerseId>> = None;
        for FoundEntry {
            entry,
            match_type,
            qidx,
        } in found
        {
            let postings_start = self.reverse_index.postings(*entry).start;
            let entry_verses = self.reverse_index.verses(*entry);
            let candidates = &mut scored.candidates;
            let matches = &mut scored.matches;
            let mut score = |c: usize, p: usize| {
                let candidate = &mut candidates[c];
                let posting = postings_start + p;
                // Track the posting to highlight words for this result
                matches.push((posting as u32, candidate.matches_head));
                candidate.matches_head = matches.len() as u32;
                // Found entries are ordered by query word so a query word is only counted once
                // per translation
                if candidate.counting_qidx != *qidx as u32 + 1 {
                    candidate.counting_qidx = *qidx as u32 + 1;
                    candidate.counted = 0;
                }
                let translations = self.reverse_index.translations(posting);
                for (i, ranking) in candidate.rankings.iter_mut().enumerate() {
                    // Does the found entry match the current translation?
                    if translations & (1 << i) != 0 {
                        if candidate.counted & (1 << i) == 0 {
                            ranking.query_words += 1;
                        }
                        match match_type {
                            MatchType::Exact => ranking.exact += 1,
                            MatchType::Typo => ranking.typos += 1,
                            _ => {}
                        }
                    }
                }
                candidate.counted |= translations;
            };

            if entry_verses.len() <= candidate_verses.len() * 2 {
                let slots = candidate_slots.get_or_insert_with(|| {
                    let mut slots = vec![0; self.verse_keys.len()];
                    for (c, verse) in candidate_verses.iter().enumerate() {
                        // There are fewer verses (and therefore candidates) than VerseId::MAX
                        slots[*verse as usize] = c as VerseId + 1;
                    }
                    slots
                });
                for (p, verse) in entry_verses.iter().enumerate() {
                    let slot = slots[*verse as usize];
                    if slot != 0 {
                        score(slot as usize - 1, p);
                    }
                }
            } else {
                intersect(candidate_verses, entry_verses, score);
            }
        }

        // Done scoring!
        scored
    }

    #[inline]
    fn collect_results(&self, scored: Scored) -> Vec<VerseResult> {
        let Scored {
            verses,
            candidates,
            matches,
        } = scored;
        // Keep the best MAX_RESULTS candidates in a max-heap so the worst of them is on top
        let mut best: BinaryHeap<u128> = BinaryHeap::with_capacity(MAX_RESULTS + 1);
        for (i, (c, verse)) in candidates.iter().zip(verses).enumerate() {
            let key = c.sort_key(i, self.verse_popularity[*verse as usize]);
            if best.len() < MAX_RESULTS {
                best.push(key);
            } else if key < *best.peek().unwrap() {
                best.pop();
                best.push(key);
            }
        }
        let order = best.into_sorted_vec();

        order
            .iter()
            .map(|key| {
                let c = &candidates[*key as u32 as usize];
                let verse = verses[*key as u32 as usize] as usize;
                let key = self.verse_keys[verse].to_be_bytes();
                // Matches are linked newest first, highlight in the order they were found
                let mut postings = Vec::new();
                let mut next = c.matches_head;
                while next != 0 {
                    let (posting, n) = matches[next as usize - 1];
                    postings.push(posting as usize);
                    next = n;
                }
                let highlights = postings
                    .iter()
                    .rev()
                    .flat_map(|&posting| self.reverse_index.highlights(posting))
                    .map(|&id| self.highlight_words.get(id as usize).to_string())
                    .collect();
                VerseResult {
                    key: Some(VerseKey::from_be_bytes(&key[1..])),
                    top_translation: c.top_ranking().0 as i32,
                    text: (0..TRANSLATION_COUNT)
                        .map(|i| {
                            self.verse_texts
                                .get(verse * TRANSLATION_COUNT + i)
                                .to_string()
                        })
                        .collect(),
                    highlights,
                    rankings: c
                        .rankings
                        .iter()
                        .map(|r| ServiceRanking {
                            typos: r.typos,
                            query_words: r.query_words,
                            exact: r.exact,
                        })
                        .collect(),
                    popularity: self.verse_popularity[verse] as i32,
                }
            })
            .collect()
    }

    /// Perform a search against the index
    pub fn search(&self, text: &str) -> ServiceResponse {
        // Tokenize input text
        let tokens = tokenize(text);

        // If we have no tokens (empty search), bail
        if tokens.is_empty() {
            return ServiceResponse {
                results: Vec::new(),
                timings: None,
            };
        }

        // Expand and determine score multiplier for each token
        let found = self.traverse_fst(&tokens);

        // If we found no index entries (no valid words), bail
        if found.is_empty() {
            return ServiceResponse {
                results: Vec::new(),
                timings: None,
            };
        }

        // Score all results
        let scored = self.score_results(&found);

        // Collect ranked results
        let results = self.collect_results(scored);

        // Construct and return response
        ServiceResponse {
            results,
            timings: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::proto::engine::ReverseIndexEntry;
    use fst::MapBuilder;
    use prost::Message;

    fn fst_bytes(entries: &[(&[u8], u64)]) -> Vec<u8> {
        let mut builder = MapBuilder::memory();
        for (key, value) in entries {
            builder.insert(key, *value).unwrap();
        }
        builder.into_inner().unwrap()
    }

    fn u64s(values: &[u64]) -> Vec<u8> {
        values
            .iter()
            .flat_map(|v| v.to_be_bytes().to_vec())
            .collect()
    }

    /// A tiny index with two tokens. LOVED appears in Genesis 2:1 which is not listed in the
    /// popularity or translation verses maps.
    fn index_data() -> IndexData {
        IndexData {
            fst: fst_bytes(&[(b"LOVE", 0), (b"LOVED", 1)]),
            reverse_index_entries: vec![
                ReverseIndexEntry {
                    map_bytes: fst_bytes(&[(&[0, 1, 1], 0), (&[0, 1, 2], 1)]),
                    counts_map_data: vec![u64s(&[1, 0, 0]), u64s(&[0, 2, 1])],
                    highlights_map_data: vec![u64s(&[0]), u64s(&[0, 1])],
                },
                ReverseIndexEntry {
                    map_bytes: fst_bytes(&[(&[0, 1, 2], 0), (&[0, 2, 1], 1)]),
                    counts_map_data: vec![u64s(&[0, 1, 0]), u64s(&[1, 0, 0])],
                    highlights_map_data: vec![u64s(&[2]), u64s(&[2])],
                },
            ],
            highlight_words: vec!["LOVE".into(), "LOVE,".into(), "LOVED".into()],
            translation_verses: fst_bytes(&[(&[0, 0, 1, 1], 0), (&[1, 0, 1, 2], 1)]),
            translation_verses_strings: vec!["Love one.".into(), "Love two, loved.".into()],
            popularity: fst_bytes(&[(&[0, 1, 1], 3), (&[0, 1, 2], 7)]),
        }
    }

    fn summary(index: &VersearchIndex, query: &str) -> Vec<String> {
        index
            .search(query)
            .results
            .iter()
            .map(|r| {
                let key = r.key.unwrap();
                let mut highlights = r.highlights.clone();
                highlights.sort();
                format!(
                    "{}:{}:{} top={} pop={} text={:?} hl={:?} rank={:?}",
                    key.book,
                    key.chapter,
                    key.verse,
                    r.top_translation,
                    r.popularity,
                    r.text,
                    highlights,
                    r.rankings
                        .iter()
                        .map(|x| (x.query_words, x.exact, x.typos))
                        .collect::<Vec<_>>()
                )
            })
            .collect()
    }

    #[test]
    fn search_small_index() {
        // Expected results were produced by the previous implementation of the engine
        let expected: &[(&str, &[&str])] = &[
            (
                "l",
                &[
                    r#"0:1:2 top=1 pop=7 text=["", "Love two, loved.", ""] hl=["LOVE", "LOVE,", "LOVED"] rank=[(0, 0, 0), (1, 0, 0), (1, 0, 0)]"#,
                    r#"0:2:1 top=0 pop=0 text=["", "", ""] hl=["LOVED"] rank=[(1, 0, 0), (0, 0, 0), (0, 0, 0)]"#,
                ],
            ),
            (
                "lov",
                &[
                    r#"0:1:2 top=1 pop=7 text=["", "Love two, loved.", ""] hl=["LOVE", "LOVE,", "LOVED"] rank=[(0, 0, 0), (1, 1, 0), (1, 1, 0)]"#,
                    r#"0:2:1 top=0 pop=0 text=["", "", ""] hl=["LOVED"] rank=[(1, 0, 0), (0, 0, 0), (0, 0, 0)]"#,
                ],
            ),
            (
                "loved",
                &[
                    r#"0:1:2 top=1 pop=7 text=["", "Love two, loved.", ""] hl=["LOVED"] rank=[(0, 0, 0), (1, 1, 0), (0, 0, 0)]"#,
                    r#"0:2:1 top=0 pop=0 text=["", "", ""] hl=["LOVED"] rank=[(1, 1, 0), (0, 0, 0), (0, 0, 0)]"#,
                ],
            ),
            (
                "love lov",
                &[
                    r#"0:1:2 top=1 pop=7 text=["", "Love two, loved.", ""] hl=["LOVE", "LOVE,", "LOVED"] rank=[(0, 0, 0), (1, 1, 0), (1, 1, 0)]"#,
                    r#"0:2:1 top=0 pop=0 text=["", "", ""] hl=["LOVED"] rank=[(1, 0, 0), (0, 0, 0), (0, 0, 0)]"#,
                ],
            ),
            ("lvoed", &[]),
            ("zzz", &[]),
            ("", &[]),
        ];
        let index = VersearchIndex::from_index_data_proto_struct(index_data());
        for (query, results) in expected {
            assert_eq!(summary(&index, query), *results, "query {:?}", query);
        }
        // Typo searches
        assert_eq!(summary(&index, "lovde").len(), 2);
        // Typo search on very long tokens used to panic
        assert!(index.search(&"a".repeat(200)).results.is_empty());
    }

    #[test]
    fn load_paths_match() {
        let data = index_data();
        let mut bytes = Vec::new();
        data.encode(&mut bytes).unwrap();
        let from_proto = VersearchIndex::from_index_data_proto_struct(data);
        let from_bytes = VersearchIndex::from_index_bytes(&bytes).unwrap();
        for query in &["l", "lov", "loved", "love lov", "lvoed", "zzz"] {
            assert_eq!(summary(&from_proto, query), summary(&from_bytes, query));
        }
    }

    #[test]
    fn invalid_bytes() {
        let mut bytes = Vec::new();
        index_data().encode(&mut bytes).unwrap();
        for len in &[1, 2, 10, bytes.len() / 2, bytes.len() - 1] {
            assert!(VersearchIndex::from_index_bytes(&bytes[..*len]).is_err());
        }
        assert!(VersearchIndex::from_index_bytes(&[0xff; 16]).is_err());
    }

    #[test]
    fn intersect_matches_naive() {
        let mut seed = 1u64;
        let mut random = move |n: u64| {
            seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
            (seed >> 33) % n
        };
        for _ in 0..500 {
            let mut lists: Vec<Vec<VerseId>> = (0..2)
                .map(|_| {
                    let len = random(64);
                    let max = random(200) + 1;
                    (0..len).map(|_| random(max) as VerseId).collect()
                })
                .collect();
            for list in lists.iter_mut() {
                list.sort_unstable();
                list.dedup();
            }
            let (a, b) = if lists[0].len() <= lists[1].len() {
                (&lists[0], &lists[1])
            } else {
                (&lists[1], &lists[0])
            };
            let mut found = Vec::new();
            intersect(a, b, |i, j| found.push((i, j)));
            let mut expected = Vec::new();
            for (i, x) in a.iter().enumerate() {
                if let Ok(j) = b.binary_search(x) {
                    expected.push((i, j));
                }
            }
            assert_eq!(found, expected);
        }
    }
}
