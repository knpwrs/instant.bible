use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new(
        r"(?ix) # Case insensitive and whitespace insignificant
          # Match multi-chapter books
          (
            (?P<book>
            # Law
              Genesis|
              Exodus|
              Leviticus|
              Numbers|
              Deuteronomy|
            # Old Testament Narrative
              Joshua|
              Judges|
              Ruth|
              (?:First|1)\sSamuel|
              (?:Second|2)\sSamuel|
              (?:First|1)\sKings|
              (?:Second|2)\sKings|
              (?:First|1)\sChronicles|
              (?:Second|2)\sChronicles|
              Ezra|
              Nehemiah|
              Esther|
            # Wisdom Literature
              Job|
              Psalms?| # Match Psalm or Psalms
              Proverbs|
              Ecclesiastes|
              Song of Solomon|
              Isaiah|
              Jeremiah|
              Lamentations|
              Ezekiel|
              Daniel|
            # Minor Prophets|
              Hosea|
              Joel|
              Amos|
              Jonah|
              Micah|
              Nahum|
              Habakkuk|
              Zephaniah|
              Haggai|
              Zechariah|
              Malachi|
            # New Testament Narrative
              Matthew|
              Mark|
              Luke|
              John|
              Acts|
            # Pauline Epistles
              Romans|
              (?:First|1)\sCorinthians|
              (?:Second|2)\sCorinthians|
              Galatians|
              Ephesians|
              Philippians|
              Colossians|
              (?:First|1)\sThessalonians|
              (?:Second|2)\sThessalonians|
              (?:First|1)\sTimothy|
              (?:Second|2)\sTimothy|
              Titus|
              Philemon|
            # General Epistles
              Hebrews|
              James|
              (?:First|1)\sPeter|
              (?:Second|2)\sPeter|
              (?:First|1)\sJohn|
            # Apocalyptic Epistle
              Revelation
            )
            \s?
            (?P<chapter>\d+):(?P<verse>\d+)
          )
          # Match single-chapter books
          |(
            (?P<book_single>
            # Minor Prophets
              Obadiah|
            # Pauline Epistles
              Philemon|
            # General Epistles
              (?:Second|2)\sJohn|
              (?:Third|3)\sJohn|
              Jude
            )
            \s?
            (?:1:)?(?P<verse_single>\d+) # Optionally match `1:` followed by verse
          )"
    )
    .unwrap();
}

/// Represents a found scripture reference
#[cfg_attr(test, derive(Debug, Eq, PartialEq))]
pub struct Match {
    pub book: String,
    pub chapter: u8,
    pub verse: u8,
}

/// Given an input string, finds all occurrences of what look like scripture
/// references and returns a vector of those
pub fn get_matches(input: &str) -> Vec<Match> {
    RE.captures_iter(input)
        .map(|cap| {
            let book = cap
                .name("book")
                .map_or_else(|| cap.name("book_single").unwrap().as_str(), |m| m.as_str())
                .to_uppercase()
                .replace("FIRST", "1")
                .replace("SECOND", "2")
                .replace("THIRD", "3");
            let chapter = cap
                .name("chapter")
                .map_or(1, |m| m.as_str().parse::<u8>().unwrap());
            let verse = cap.name("verse").map_or_else(
                || {
                    cap.name("verse_single")
                        .unwrap()
                        .as_str()
                        .parse::<u8>()
                        .unwrap()
                },
                |m| m.as_str().parse::<u8>().unwrap(),
            );

            Match {
                // I actually cite as 'Psalm' with no trailing 's', but in the interest
                // of data consitency with a `book` field this makes sense
                book: if book == "PSALM" {
                    "PSALMS".to_string()
                } else {
                    book
                },
                chapter,
                verse,
            }
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn regex_matches_book_chapter_verse() {
        let re_match = RE.captures("Genesis 1:1").unwrap();
        assert_eq!(&re_match["book"], "Genesis");
        assert_eq!(&re_match["chapter"], "1");
        assert_eq!(&re_match["verse"], "1");
    }

    #[test]
    fn regex_matches_book_verse_single() {
        let re_match = RE.captures("Jude 1:1").unwrap();
        assert_eq!(&re_match["book_single"], "Jude");
        assert_eq!(&re_match["verse_single"], "1");
    }

    #[test]
    fn regex_matches_book_verse_single_optional() {
        let re_match = RE.captures("Obadiah 1:1").unwrap();
        assert_eq!(&re_match["book_single"], "Obadiah");
        assert_eq!(&re_match["verse_single"], "1");
    }

    #[test]
    fn test_get_matches() {
        let matches = get_matches(
            r"
            Hello and blah
            and Genesis 1:1 and Exodus 1 but...\r\n John 3:16 also and stuff and Jude 3 with Obadiah 1:2 blerp Psalms 12 errrr\n\n
            Psalm 119:2 blah Psalms 12:2
            First Samuel 1:2 bloop Second John 3 blip blip Third John 4
        ",
        );
        println!("{:?}", matches);
        assert_eq!(
            matches,
            vec![
                Match {
                    book: "GENESIS".to_string(),
                    chapter: 1,
                    verse: 1,
                },
                Match {
                    book: "JOHN".to_string(),
                    chapter: 3,
                    verse: 16,
                },
                Match {
                    book: "JUDE".to_string(),
                    chapter: 1,
                    verse: 3
                },
                Match {
                    book: "OBADIAH".to_string(),
                    chapter: 1,
                    verse: 2
                },
                Match {
                    book: "PSALMS".to_string(),
                    chapter: 119,
                    verse: 2
                },
                Match {
                    book: "PSALMS".to_string(),
                    chapter: 12,
                    verse: 2
                },
                Match {
                    book: "1 SAMUEL".to_string(),
                    chapter: 1,
                    verse: 2
                },
                Match {
                    book: "2 JOHN".to_string(),
                    chapter: 1,
                    verse: 3
                },
                Match {
                    book: "3 JOHN".to_string(),
                    chapter: 1,
                    verse: 4
                },
            ]
        );
    }
}
