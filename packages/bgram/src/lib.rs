use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

#[derive(Clone)]
struct JsonVerse {
    book: String,
    chapter: u8,
    verse: u8,
    text: String,
}

#[derive(Clone, Debug)]
struct TranslationResult {
    score: f64,
    highlights: Vec<(usize, usize)>,
}

struct BGramIndexGramEntry {
    // Map translations to highlights for this bgram
    highlights: HashMap<String, Vec<(usize, usize)>>,
}

struct BGramIndexVerseEntry {
    // Map translations to gram sets
    grams: HashMap<String, HashSet<BGram>>,
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct BGram(char, char, char);
#[derive(PartialEq, Eq, Clone, Debug)]
struct IndexedBGram(usize, BGram);
#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct VerseKey(String, u8, u8);

struct BGramIndex {
    gram_index: HashMap<BGram, HashMap<VerseKey, BGramIndexGramEntry>>,
    verse_index: HashMap<VerseKey, BGramIndexVerseEntry>,
}

fn make_bgrams(text: &String) -> Vec<IndexedBGram> {
    text.to_uppercase()
        .chars()
        // Ensure all characters retain original index
        .enumerate()
        // Reject non-alphabetic characters and create 3-grams
        .filter(|(_, c)| c.is_alphabetic())
        .tuple_windows()
        // Only retain index per-gram, not per-character
        .map(|((i, a), (_, b), (_, c))| IndexedBGram(i, BGram(a, b, c)))
        .collect()
}

fn make_gram_set(bgrams: &Vec<IndexedBGram>) -> HashSet<BGram> {
    let mut set: HashSet<BGram> = HashSet::new();
    for IndexedBGram(_, gram) in bgrams {
        set.insert(gram.clone());
    }
    set
}

fn jaccard_index<T: Eq + Hash>(first: &HashSet<T>, second: &HashSet<T>) -> f64 {
    if first.len() == 0 && second.len() == 0 {
        1.0
    } else {
        let numerator = first.intersection(second).count() as f64;
        let denominator = first.union(second).count() as f64;
        numerator / denominator
    }
}

impl BGramIndex {
    pub fn new() -> BGramIndex {
        BGramIndex {
            gram_index: HashMap::new(),
            verse_index: HashMap::new(),
        }
    }

    pub fn insert_verse(&mut self, translation: &String, verse: JsonVerse) {
        let grams = make_bgrams(&verse.text);
        let verse_key = VerseKey(verse.book, verse.chapter, verse.verse);

        self.verse_index
            .entry(verse_key.clone())
            .or_insert(BGramIndexVerseEntry {
                grams: HashMap::new(),
            })
            .grams
            .insert(translation.clone(), make_gram_set(&grams));

        for IndexedBGram(i, gram) in &grams {
            let entry = self
                .gram_index
                .entry(gram.clone())
                .or_insert(HashMap::new())
                .entry(verse_key.clone())
                .or_insert(BGramIndexGramEntry {
                    highlights: HashMap::new(),
                });
            entry
                .highlights
                .entry(translation.clone())
                .or_insert(vec![])
                .push((*i, i + 2));
        }
    }

    pub fn search(&self, text: &String) -> Vec<(VerseKey, HashMap<String, TranslationResult>)> {
        let search_grams = make_gram_set(&make_bgrams(text));
        // Map verse keys to mappings of translations to jaccard indices
        let mut verse_ranks: HashMap<VerseKey, HashMap<String, TranslationResult>> = HashMap::new();
        for gram in &search_grams {
            if let Some(gram_entries) = self.gram_index.get(gram) {
                // Loop over index entries for the current search gram
                for (verse_key, entry) in gram_entries {
                    // If we haven't added this matching verse to the ranked results, do so
                    if !verse_ranks.contains_key(&verse_key) {
                        let mut translation_scores: HashMap<String, TranslationResult> =
                            HashMap::new();
                        if let Some(verse_entries) = self.verse_index.get(verse_key) {
                            for (translation, verse_grams) in &verse_entries.grams {
                                let score = jaccard_index(&verse_grams, &search_grams);
                                translation_scores.insert(
                                    translation.clone(),
                                    TranslationResult {
                                        score,
                                        highlights: vec![],
                                    },
                                );
                            }
                        }
                        verse_ranks.insert(verse_key.clone(), translation_scores);
                    }
                    // Add highlights for this matching gram to the output, make sure this is done for every gram
                    for (translation, highlights) in &entry.highlights {
                        if let Some(ranked_verse) = verse_ranks.get_mut(&verse_key) {
                            if let Some(ranked_translation) = ranked_verse.get_mut(translation) {
                                for hl in highlights {
                                    ranked_translation.highlights.push(hl.clone());
                                }
                            }
                        }
                    }
                }
            }
        }
        let mut results: Vec<(VerseKey, HashMap<String, TranslationResult>)> = verse_ranks
            .iter()
            .map(|(key, results)| (key.clone(), results.clone()))
            .collect();
        results.sort_by(|(_, a_res), (_, b_res)| {
            let a_sum: f64 = a_res.values().map(|v| v.score).sum();
            let b_sum: f64 = b_res.values().map(|v| v.score).sum();
            // Descending sort
            b_sum.partial_cmp(&a_sum).unwrap()
        });
        results
            .iter()
            .take(50)
            .map(|(key, scores)| (key.clone(), scores.clone()))
            .collect()
    }
}

// #[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_bgrams() {
        let s = String::from("Jesus wept.");

        // Process text into indexed 3-grams
        let v = make_bgrams(&s);

        assert_eq!(
            v,
            vec![
                IndexedBGram(0, BGram('J', 'E', 'S')),
                IndexedBGram(1, BGram('E', 'S', 'U')),
                IndexedBGram(2, BGram('S', 'U', 'S')),
                IndexedBGram(3, BGram('U', 'S', 'W')),
                IndexedBGram(4, BGram('S', 'W', 'E')),
                IndexedBGram(6, BGram('W', 'E', 'P')),
                IndexedBGram(7, BGram('E', 'P', 'T'))
            ]
        );
    }

    #[test]
    fn test_jaccard_index() {
        let empty_set: HashSet<i32> = HashSet::new();
        let empty_set_2: HashSet<i32> = HashSet::new();
        assert_eq!(jaccard_index(&empty_set, &empty_set_2), 1.0);
        let set_a: HashSet<i32> = vec![1, 2, 3, 4].into_iter().collect();
        let set_b: HashSet<i32> = vec![3, 4, 5, 6].into_iter().collect();
        let set_c: HashSet<i32> = vec![4, 5].into_iter().collect();
        assert_eq!(jaccard_index(&set_a, &set_b), 2.0 / 6.0);
        assert_eq!(jaccard_index(&set_b, &set_c), 0.5);
        assert_eq!(jaccard_index(&empty_set, &set_a), 0.0);
    }

    #[test]
    fn insert_and_search() {
        let mut index = BGramIndex::new();
        let v1 = JsonVerse {
            book: String::from("John"),
            chapter: 11,
            verse: 35,
            text: String::from("Jesus wept."),
        };
        let v2_net = JsonVerse {
            book: String::from("Galatians"),
            chapter: 6,
            verse: 11,
            text: String::from("See what big letters I make as I write to you with my own hand!"),
        };
        let v2_kjv = JsonVerse {
            text: String::from(
                "Ye see how large a letter I have written unto you with mine own hand.",
            ),
            ..v2_net.clone()
        };
        let v3_net = JsonVerse {
            book: String::from("1 Corinthians"),
            chapter: 16,
            verse: 21,
            text: String::from("I, Paul, send this greeting with my own hand."),
        };
        let v3_kjv = JsonVerse {
            text: String::from("The salutation of me Paul with mine own hand."),
            ..v3_net.clone()
        };
        let v4_net = JsonVerse {
            book: String::from("Colossions"),
            chapter: 4,
            verse: 18,
            text: String::from("I, Paul, write this greeting by my own hand. Remember my chains. Grace be with you."),
        };
        let v4_kjv = JsonVerse {
            text: String::from("The salutation by the hand of me Paul. Remember my bonds. Grace be with you. Amen."),
            ..v4_net.clone()
        };
        let v5_net = JsonVerse {
            book: String::from("2 Thessalonians"),
            chapter: 3,
            verse: 17,
            text: String::from("I, Paul, write this greeting with my own hand, which is how I write in every letter."),
        };
        let v5_kjv = JsonVerse {
            text: String::from("The salutation of Paul with mine own hand, which is the token in every epistle: so I write."),
            ..v5_net.clone()
        };
        let v6_net = JsonVerse {
            book: String::from("Philemon"),
            chapter: 1,
            verse: 19,
            text: String::from("I, Paul, have written this letter with my own hand: I will repay it. I could also mention that you owe me your very self."),
        };
        let v6_kjv = JsonVerse {
            text: String::from("I Paul have written it with mine own hand, I will repay it: albeit I do not say to thee how thou owest unto me even thine own self besides."),
            ..v6_net.clone()
        };
        index.insert_verse(&String::from("NET"), v1.clone());
        index.insert_verse(&String::from("KJV"), v1.clone());
        index.insert_verse(&String::from("NET"), v2_net);
        index.insert_verse(&String::from("KJV"), v2_kjv);
        index.insert_verse(&String::from("NET"), v3_net);
        index.insert_verse(&String::from("KJV"), v3_kjv);
        index.insert_verse(&String::from("NET"), v4_net);
        index.insert_verse(&String::from("KJV"), v4_kjv);
        index.insert_verse(&String::from("NET"), v5_net);
        index.insert_verse(&String::from("KJV"), v5_kjv);
        index.insert_verse(&String::from("NET"), v6_net);
        index.insert_verse(&String::from("KJV"), v6_kjv);
        let res = index.search(&String::from("Jes wep"));
        assert_eq!(res.len(), 1);
        let res = index.search(&String::from("Paul"));
        assert_eq!(res.len(), 4);
        let res = index.search(&String::from("Letters"));
        assert_eq!(res.len(), 3);
        let res = index.search(&String::from("Own Hand"));
        assert_eq!(res.len(), 5);
        let res = index.search(&String::from("Own Hand Greetings"));
        assert_eq!(res.len(), 5);
    }
}
