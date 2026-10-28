//! Borrowed views of `IndexData`, either from a decoded protobuf struct or straight from the
//! encoded bytes. Reading straight from the encoded bytes avoids materializing every
//! `ReverseIndexEntry` (millions of small allocations) only to throw them away again.
use crate::proto::engine::{IndexData, ReverseIndexEntry};
use anyhow::{anyhow, bail, Context, Result};

pub enum RawEntry<'a> {
    Proto(&'a ReverseIndexEntry),
    Wire(&'a [u8]),
}

impl<'a> RawEntry<'a> {
    /// Reads the entry, returning its FST map bytes and filling `counts` and `highlights`
    pub fn read(
        &self,
        counts: &mut Vec<&'a [u8]>,
        highlights: &mut Vec<&'a [u8]>,
    ) -> Result<&'a [u8]> {
        counts.clear();
        highlights.clear();
        match self {
            RawEntry::Proto(entry) => {
                counts.extend(entry.counts_map_data.iter().map(|b| b.as_slice()));
                highlights.extend(entry.highlights_map_data.iter().map(|b| b.as_slice()));
                Ok(&entry.map_bytes)
            }
            RawEntry::Wire(bytes) => {
                let mut map: &[u8] = &[];
                for field in Fields(bytes) {
                    match field? {
                        (1, value) => map = value,
                        (2, value) => counts.push(value),
                        (3, value) => highlights.push(value),
                        _ => {}
                    }
                }
                Ok(map)
            }
        }
    }
}

pub struct RawIndex<'a> {
    pub fst: &'a [u8],
    pub entries: Vec<RawEntry<'a>>,
    pub highlight_words: Vec<&'a str>,
    pub translation_verses: &'a [u8],
    pub translation_verses_strings: Vec<&'a str>,
    pub popularity: &'a [u8],
}

impl<'a> RawIndex<'a> {
    pub fn from_proto(data: &'a IndexData) -> Self {
        Self {
            fst: &data.fst,
            entries: data
                .reverse_index_entries
                .iter()
                .map(RawEntry::Proto)
                .collect(),
            highlight_words: data.highlight_words.iter().map(|s| s.as_str()).collect(),
            translation_verses: &data.translation_verses,
            translation_verses_strings: data
                .translation_verses_strings
                .iter()
                .map(|s| s.as_str())
                .collect(),
            popularity: &data.popularity,
        }
    }

    /// Reads an encoded `IndexData` message without copying any of its contents
    pub fn from_bytes(bytes: &'a [u8]) -> Result<Self> {
        let mut raw = Self {
            fst: &[],
            entries: Vec::new(),
            highlight_words: Vec::new(),
            translation_verses: &[],
            translation_verses_strings: Vec::new(),
            popularity: &[],
        };
        let to_str = |b| std::str::from_utf8(b).context("Invalid UTF-8 in index string");
        for field in Fields(bytes) {
            match field? {
                (1, value) => raw.fst = value,
                (2, value) => raw.entries.push(RawEntry::Wire(value)),
                (3, value) => raw.highlight_words.push(to_str(value)?),
                (4, value) => raw.translation_verses = value,
                (5, value) => raw.translation_verses_strings.push(to_str(value)?),
                (6, value) => raw.popularity = value,
                _ => {}
            }
        }
        Ok(raw)
    }
}

/// Iterates the length-delimited fields of an encoded protobuf message as
/// `(field number, value)`. Fields of other wire types are skipped.
struct Fields<'a>(&'a [u8]);

impl<'a> Fields<'a> {
    fn varint(&mut self) -> Result<u64> {
        let mut value = 0u64;
        for shift in (0..64).step_by(7) {
            let (&byte, rest) = self
                .0
                .split_first()
                .ok_or_else(|| anyhow!("Truncated varint in index data"))?;
            self.0 = rest;
            value |= u64::from(byte & 0x7f) << shift;
            if byte & 0x80 == 0 {
                return Ok(value);
            }
        }
        bail!("Invalid varint in index data")
    }

    fn take(&mut self, len: u64) -> Result<&'a [u8]> {
        if len > self.0.len() as u64 {
            bail!("Truncated field in index data");
        }
        let (value, rest) = self.0.split_at(len as usize);
        self.0 = rest;
        Ok(value)
    }

    fn next_field(&mut self) -> Result<Option<(u32, &'a [u8])>> {
        while !self.0.is_empty() {
            let key = self.varint()?;
            match key & 0x7 {
                0 => {
                    self.varint()?;
                }
                1 => {
                    self.take(8)?;
                }
                2 => {
                    let len = self.varint()?;
                    return Ok(Some(((key >> 3) as u32, self.take(len)?)));
                }
                5 => {
                    self.take(4)?;
                }
                wire_type => bail!("Unsupported wire type {} in index data", wire_type),
            }
        }
        Ok(None)
    }
}

impl<'a> Iterator for Fields<'a> {
    type Item = Result<(u32, &'a [u8])>;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.next_field();
        if next.is_err() {
            // Stop iterating after an error
            self.0 = &[];
        }
        next.transpose()
    }
}
