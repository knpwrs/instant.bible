/// An immutable list of strings stored in a single contiguous buffer. Compared to a
/// `Vec<String>` this avoids one heap allocation (plus its header and allocator slack) per
/// string.
#[derive(Default)]
pub struct StringTable {
    buf: String,
    /// `offsets[i]..offsets[i + 1]` is the byte range of string `i`
    offsets: Vec<u32>,
}

impl StringTable {
    pub fn with_capacity(strings: usize, bytes: usize) -> Self {
        let mut offsets = Vec::with_capacity(strings + 1);
        offsets.push(0);
        Self {
            buf: String::with_capacity(bytes),
            offsets,
        }
    }

    pub fn push(&mut self, s: &str) {
        self.buf.push_str(s);
        self.offsets.push(self.buf.len() as u32);
    }

    pub fn get(&self, idx: usize) -> &str {
        &self.buf[self.offsets[idx] as usize..self.offsets[idx + 1] as usize]
    }

    pub fn len(&self) -> usize {
        self.offsets.len().saturating_sub(1)
    }

    pub fn shrink_to_fit(&mut self) {
        self.buf.shrink_to_fit();
        self.offsets.shrink_to_fit();
    }

    /// Returns the index of the first string for which `pred` is false, assuming the table is
    /// partitioned by `pred` (see `slice::partition_point`)
    pub fn partition_point<P: Fn(&str) -> bool>(&self, pred: P) -> usize {
        let (mut lo, mut hi) = (0, self.len());
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if pred(self.get(mid)) {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        lo
    }
}

impl<'a> std::iter::FromIterator<&'a str> for StringTable {
    fn from_iter<I: IntoIterator<Item = &'a str>>(iter: I) -> Self {
        let mut table = Self::with_capacity(0, 0);
        for s in iter {
            table.push(s);
        }
        table
    }
}
