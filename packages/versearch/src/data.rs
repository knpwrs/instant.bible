use serde::{Deserialize, Serialize};

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
    pub fn from(book: &str, chapter: u8, verse: u8) -> Option<VerseKey> {
        match Book::from_str(book) {
            Some(book) => Some(VerseKey {
                book,
                chapter,
                verse,
            }),
            _ => None,
        }
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

impl Book {
    pub fn from_str(book: &str) -> Option<Book> {
        use Book::*;
        match book {
            "Genesis" => Some(Genesis),
            "Exodus" => Some(Exodus),
            "Leviticus" => Some(Leviticus),
            "Numbers" => Some(Numbers),
            "Deuteronomy" => Some(Deuteronomy),
            "Joshua" => Some(Joshua),
            "Judges" => Some(Judges),
            "Ruth" => Some(Ruth),
            "1 Samuel" => Some(FirstSamuel),
            "2 Samuel" => Some(SecondSamuel),
            "1 Kings" => Some(FirstKings),
            "2 Kings" => Some(SecondKings),
            "1 Chronicles" => Some(FirstChronicles),
            "2 Chronicles" => Some(SecondChronicles),
            "Ezra" => Some(Ezra),
            "Nehemiah" => Some(Nehemiah),
            "Esther" => Some(Esther),
            "Job" => Some(Job),
            "Psalms" => Some(Psalms),
            "Proverbs" => Some(Proverbs),
            "Ecclesiastes" => Some(Ecclesiastes),
            "Song of Solomon" => Some(SongOfSolomon),
            "Isaiah" => Some(Isaiah),
            "Jeremiah" => Some(Jeremiah),
            "Lamentations" => Some(Lamentations),
            "Ezekiel" => Some(Ezekiel),
            "Daniel" => Some(Daniel),
            "Hosea" => Some(Hosea),
            "Joel" => Some(Joel),
            "Amos" => Some(Amos),
            "Obadiah" => Some(Obadiah),
            "Jonah" => Some(Jonah),
            "Micah" => Some(Micah),
            "Nahum" => Some(Nahum),
            "Habakkuk" => Some(Habakkuk),
            "Zephaniah" => Some(Zephaniah),
            "Haggai" => Some(Haggai),
            "Zechariah" => Some(Zechariah),
            "Malachi" => Some(Malachi),
            "Matthew" => Some(Matthew),
            "Mark" => Some(Mark),
            "Luke" => Some(Luke),
            "John" => Some(John),
            "Acts" => Some(Acts),
            "Romans" => Some(Romans),
            "1 Corinthians" => Some(FirstCorinthians),
            "2 Corinthians" => Some(SecondCorinthians),
            "Galatians" => Some(Galatians),
            "Ephesians" => Some(Ephesians),
            "Philippians" => Some(Philippians),
            "Colossians" => Some(Colossians),
            "1 Thessalonians" => Some(FirstThessalonians),
            "2 Thessalonians" => Some(SecondThessalonians),
            "1 Timothy" => Some(FirstTimothy),
            "2 Timothy" => Some(SecondTimothy),
            "Titus" => Some(Titus),
            "Philemon" => Some(Philemon),
            "Hebrews" => Some(Hebrews),
            "James" => Some(James),
            "1 Peter" => Some(FirstPeter),
            "2 Peter" => Some(SecondPeter),
            "1 John" => Some(FirstJohn),
            "2 John" => Some(SecondJohn),
            "3 John" => Some(ThirdJohn),
            "Jude" => Some(Jude),
            "Revelation" => Some(Revelation),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Book;
    #[test]
    fn test_book_from_str() {
        assert_eq!(Book::from_str("Genesis"), Some(Book::Genesis));
        assert_eq!(Book::from_str("Revelation"), Some(Book::Revelation));
        assert_eq!(Book::from_str("Banana"), None);
    }
}
