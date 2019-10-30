use super::error::DataInputError;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Deserialize)]
pub struct JsonVerse {
    pub book: String,
    pub chapter: u8,
    pub verse: u8,
    pub text: String,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct VerseKey {
    pub book: Book,
    pub chapter: u8,
    pub verse: u8,
}

impl VerseKey {
    pub fn from(book: &str, chapter: u8, verse: u8) -> Result<VerseKey, DataInputError> {
        Ok(VerseKey {
            book: Book::from_str(book)?,
            chapter,
            verse,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub enum Book {
    Genesis,
    Exodus,
    Leviticus,
    Numbers,
    Deuteronomy,
    Joshua,
    Judges,
    Ruth,
    FirstSamuel,
    SecondSamuel,
    FirstKings,
    SecondKings,
    FirstChronicles,
    SecondChronicles,
    Ezra,
    Nehemiah,
    Esther,
    Job,
    Psalms,
    Proverbs,
    Ecclesiastes,
    SongOfSolomon,
    Isaiah,
    Jeremiah,
    Lamentations,
    Ezekiel,
    Daniel,
    Hosea,
    Joel,
    Amos,
    Obadiah,
    Jonah,
    Micah,
    Nahum,
    Habakkuk,
    Zephaniah,
    Haggai,
    Zechariah,
    Malachi,
    Matthew,
    Mark,
    Luke,
    John,
    Acts,
    Romans,
    FirstCorinthians,
    SecondCorinthians,
    Galatians,
    Ephesians,
    Philippians,
    Colossians,
    FirstThessalonians,
    SecondThessalonians,
    FirstTimothy,
    SecondTimothy,
    Titus,
    Philemon,
    Hebrews,
    James,
    FirstPeter,
    SecondPeter,
    FirstJohn,
    SecondJohn,
    ThirdJohn,
    Jude,
    Revelation,
}

impl FromStr for Book {
    type Err = DataInputError;

    fn from_str(book: &str) -> Result<Book, Self::Err> {
        use Book::*;
        match book {
            "Genesis" => Ok(Genesis),
            "Exodus" => Ok(Exodus),
            "Leviticus" => Ok(Leviticus),
            "Numbers" => Ok(Numbers),
            "Deuteronomy" => Ok(Deuteronomy),
            "Joshua" => Ok(Joshua),
            "Judges" => Ok(Judges),
            "Ruth" => Ok(Ruth),
            "1 Samuel" => Ok(FirstSamuel),
            "2 Samuel" => Ok(SecondSamuel),
            "1 Kings" => Ok(FirstKings),
            "2 Kings" => Ok(SecondKings),
            "1 Chronicles" => Ok(FirstChronicles),
            "2 Chronicles" => Ok(SecondChronicles),
            "Ezra" => Ok(Ezra),
            "Nehemiah" => Ok(Nehemiah),
            "Esther" => Ok(Esther),
            "Job" => Ok(Job),
            "Psalms" => Ok(Psalms),
            "Proverbs" => Ok(Proverbs),
            "Ecclesiastes" => Ok(Ecclesiastes),
            "Song of Solomon" => Ok(SongOfSolomon),
            "Isaiah" => Ok(Isaiah),
            "Jeremiah" => Ok(Jeremiah),
            "Lamentations" => Ok(Lamentations),
            "Ezekiel" => Ok(Ezekiel),
            "Daniel" => Ok(Daniel),
            "Hosea" => Ok(Hosea),
            "Joel" => Ok(Joel),
            "Amos" => Ok(Amos),
            "Obadiah" => Ok(Obadiah),
            "Jonah" => Ok(Jonah),
            "Micah" => Ok(Micah),
            "Nahum" => Ok(Nahum),
            "Habakkuk" => Ok(Habakkuk),
            "Zephaniah" => Ok(Zephaniah),
            "Haggai" => Ok(Haggai),
            "Zechariah" => Ok(Zechariah),
            "Malachi" => Ok(Malachi),
            "Matthew" => Ok(Matthew),
            "Mark" => Ok(Mark),
            "Luke" => Ok(Luke),
            "John" => Ok(John),
            "Acts" => Ok(Acts),
            "Romans" => Ok(Romans),
            "1 Corinthians" => Ok(FirstCorinthians),
            "2 Corinthians" => Ok(SecondCorinthians),
            "Galatians" => Ok(Galatians),
            "Ephesians" => Ok(Ephesians),
            "Philippians" => Ok(Philippians),
            "Colossians" => Ok(Colossians),
            "1 Thessalonians" => Ok(FirstThessalonians),
            "2 Thessalonians" => Ok(SecondThessalonians),
            "1 Timothy" => Ok(FirstTimothy),
            "2 Timothy" => Ok(SecondTimothy),
            "Titus" => Ok(Titus),
            "Philemon" => Ok(Philemon),
            "Hebrews" => Ok(Hebrews),
            "James" => Ok(James),
            "1 Peter" => Ok(FirstPeter),
            "2 Peter" => Ok(SecondPeter),
            "1 John" => Ok(FirstJohn),
            "2 John" => Ok(SecondJohn),
            "3 John" => Ok(ThirdJohn),
            "Jude" => Ok(Jude),
            "Revelation" => Ok(Revelation),
            _ => Err(DataInputError::new()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub enum Translation {
    KJV,
    NET,
}

impl FromStr for Translation {
    type Err = DataInputError;

    fn from_str(translation: &str) -> Result<Translation, Self::Err> {
        use Translation::*;
        match translation {
            "KJV" => Ok(KJV),
            "NET" => Ok(NET),
            _ => Err(DataInputError::new()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Book, FromStr, Translation};

    #[test]
    fn test_book_from_str() {
        assert_eq!(Book::from_str("Genesis"), Ok(Book::Genesis));
        assert_eq!(Book::from_str("Revelation"), Ok(Book::Revelation));
        assert!(Book::from_str("Banana").is_err());
    }

    #[test]
    fn test_translation_from_str() {
        assert_eq!(Translation::from_str("KJV"), Ok(Translation::KJV));
        assert_eq!(Translation::from_str("NET"), Ok(Translation::NET));
    }
}
