use fst::Automaton;

/// The largest edit distance supported by `Typo`
pub const MAX_DISTANCE: usize = 2;
const BAND: usize = 2 * MAX_DISTANCE + 1;

/// Matches keys within a Levenshtein distance of a byte string.
///
/// Unlike `fst::automaton::Levenshtein` this does not build a DFA up front (which is expensive,
/// and fails for long queries); it computes rows of the edit distance matrix as the FST is
/// traversed instead. Only the diagonal band of each row which can be within `distance` is kept,
/// so the state is small and `Copy`.
///
/// Distances are computed over bytes, which matches `fst::automaton::Levenshtein` (which works
/// over chars) for ASCII queries and keys such as tokens.
pub struct Typo<'a> {
    query: &'a [u8],
    distance: u8,
}

/// `(row, band)` where `band[k]` is the distance between the first `row` bytes of the key and
/// the first `row - distance + k` bytes of the query (saturated at `distance + 1`)
pub type TypoState = (usize, [u8; BAND]);

impl<'a> Typo<'a> {
    pub fn new(query: &'a str, distance: usize) -> Self {
        assert!(distance <= MAX_DISTANCE, "Typo distance too large");
        Self {
            query: query.as_bytes(),
            distance: distance as u8,
        }
    }

    fn width(&self) -> usize {
        2 * self.distance as usize + 1
    }

    /// Column of the edit distance matrix that band index `k` refers to in row `row`
    fn column(&self, row: usize, k: usize) -> Option<usize> {
        (row + k)
            .checked_sub(self.distance as usize)
            .filter(|&j| j <= self.query.len())
    }
}

impl<'a> Automaton for Typo<'a> {
    type State = TypoState;

    fn start(&self) -> TypoState {
        let mut band = [self.distance + 1; BAND];
        for (k, d) in band.iter_mut().enumerate().take(self.width()) {
            if let Some(j) = self.column(0, k) {
                *d = (j as u8).min(self.distance + 1);
            }
        }
        (0, band)
    }

    fn is_match(&self, (row, band): &TypoState) -> bool {
        // Band index of the last column
        (self.query.len() + self.distance as usize)
            .checked_sub(*row)
            .filter(|&k| k < self.width())
            .map_or(false, |k| band[k] <= self.distance)
    }

    fn can_match(&self, (_, band): &TypoState) -> bool {
        band[..self.width()].iter().any(|&d| d <= self.distance)
    }

    fn accept(&self, (row, band): &TypoState, byte: u8) -> TypoState {
        let inf = self.distance + 1;
        let mut next = [inf; BAND];
        for k in 0..self.width() {
            let j = match self.column(row + 1, k) {
                Some(j) => j,
                None => continue,
            };
            let mut d = inf;
            if j > 0 {
                // Match or substitute: D[row][j - 1] + cost
                let cost = (self.query[j - 1] != byte) as u8;
                d = d.min(band[k].saturating_add(cost));
                // Skip a query byte: D[row + 1][j - 1] + 1
                if k > 0 {
                    d = d.min(next[k - 1].saturating_add(1));
                }
            }
            // Skip a key byte: D[row][j] + 1
            if k + 1 < self.width() {
                d = d.min(band[k + 1].saturating_add(1));
            }
            next[k] = d.min(inf);
        }
        (row + 1, next)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fst::{automaton::Levenshtein, IntoStreamer, Set};

    fn search<A: Automaton>(set: &Set<Vec<u8>>, aut: A) -> Vec<String> {
        set.search(aut).into_stream().into_strs().unwrap()
    }

    #[test]
    fn matches_fst_levenshtein() {
        let mut words = vec![
            "",
            "A",
            "AB",
            "ABC",
            "ABCD",
            "BCD",
            "ACBD",
            "SHEPHERD",
            "SHEPHERDS",
            "SHEPHRED",
            "SHEPERD",
            "HEPHERD",
            "SHEPHERDESS",
            "LOVE",
            "LOVED",
            "LOVES",
            "OVE",
            "LVOE",
            "GLOVE",
            "BEGINNING",
            "BEGINNINGS",
            "BEGINING",
            "RIGHTEOUSNESS",
            "RIGHTOUSNESS",
        ];
        words.sort();
        let set = Set::from_iter(words).unwrap();
        for query in &[
            "",
            "A",
            "ABCD",
            "LVOE",
            "LOVE",
            "SHEPHRED",
            "BEGINNIGN",
            "RIGHTOUSNESS",
            "XYZ",
        ] {
            for distance in 0..=MAX_DISTANCE {
                assert_eq!(
                    search(&set, Typo::new(query, distance)),
                    search(&set, Levenshtein::new(query, distance as u32).unwrap()),
                    "query {:?} distance {}",
                    query,
                    distance
                );
            }
        }
    }
}
