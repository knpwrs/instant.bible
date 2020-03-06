pub mod data {
    use anyhow::{anyhow, Context, Result};
    use prost::Message;

    include!(concat!(env!("OUT_DIR"), "/instantbible.data.rs"));

    impl VerseKey {
        pub const fn get_byte_size() -> usize {
            std::mem::size_of::<u8>() * 3
        }

        pub fn to_be_bytes(&self) -> Vec<u8> {
            let mut v = Vec::with_capacity(VerseKey::get_byte_size());
            v.extend(&(self.book as u8).to_be_bytes());
            v.extend(&(self.chapter as u8).to_be_bytes());
            v.extend(&(self.verse as u8).to_be_bytes());
            v
        }

        pub fn from_be_bytes(bytes: &[u8]) -> Self {
            let book = u8::from_be_bytes([bytes[0]]) as i32;
            let chapter = u8::from_be_bytes([bytes[1]]) as u32;
            let verse = u8::from_be_bytes([bytes[2]]) as u32;
            Self {
                book,
                chapter,
                verse,
            }
        }
    }

    impl Book {
        pub fn from_string(name: &str) -> Result<Self> {
            match name {
                "GENESIS" => Ok(Self::Genesis),
                "EXODUS" => Ok(Self::Exodus),
                "LEVITICUS" => Ok(Self::Leviticus),
                "NUMBERS" => Ok(Self::Numbers),
                "DEUTERONOMY" => Ok(Self::Deuteronomy),
                "JOSHUA" => Ok(Self::Joshua),
                "JUDGES" => Ok(Self::Judges),
                "RUTH" => Ok(Self::Ruth),
                "FIRST SAMUEL" => Ok(Self::FirstSamuel),
                "SECOND SAMUEL" => Ok(Self::SecondSamuel),
                "FIRST KINGS" => Ok(Self::FirstKings),
                "SECOND KINGS" => Ok(Self::SecondKings),
                "FIRST CHRONICLES" => Ok(Self::FirstChronicles),
                "SECOND CHRONICLES" => Ok(Self::SecondChronicles),
                "EZRA" => Ok(Self::Ezra),
                "NEHEMIAH" => Ok(Self::Nehemiah),
                "ESTHER" => Ok(Self::Esther),
                "JOB" => Ok(Self::Job),
                "PSALMS" => Ok(Self::Psalms),
                "PROVERBS" => Ok(Self::Proverbs),
                "ECCLESIASTES" => Ok(Self::Ecclesiastes),
                "SONG OF SOLOMON" => Ok(Self::SongOfSolomon),
                "ISAIAH" => Ok(Self::Isaiah),
                "JEREMIAH" => Ok(Self::Jeremiah),
                "LAMENTATIONS" => Ok(Self::Lamentations),
                "EZEKIEL" => Ok(Self::Ezekiel),
                "DANIEL" => Ok(Self::Daniel),
                "HOSEA" => Ok(Self::Hosea),
                "JOEL" => Ok(Self::Joel),
                "AMOS" => Ok(Self::Amos),
                "OBADIAH" => Ok(Self::Obadiah),
                "JONAH" => Ok(Self::Jonah),
                "MICAH" => Ok(Self::Micah),
                "NAHUM" => Ok(Self::Nahum),
                "HABAKKUK" => Ok(Self::Habakkuk),
                "ZEPHANIAH" => Ok(Self::Zephaniah),
                "HAGGAI" => Ok(Self::Haggai),
                "ZECHARIAH" => Ok(Self::Zechariah),
                "MALACHI" => Ok(Self::Malachi),
                "MATTHEW" => Ok(Self::Matthew),
                "MARK" => Ok(Self::Mark),
                "LUKE" => Ok(Self::Luke),
                "JOHN" => Ok(Self::John),
                "ACTS" => Ok(Self::Acts),
                "ROMANS" => Ok(Self::Romans),
                "FIRST CORINTHIANS" => Ok(Self::FirstCorinthians),
                "SECOND CORINTHIANS" => Ok(Self::SecondCorinthians),
                "GALATIANS" => Ok(Self::Galatians),
                "EPHESIANS" => Ok(Self::Ephesians),
                "PHILIPPIANS" => Ok(Self::Philippians),
                "COLOSSIANS" => Ok(Self::Colossians),
                "FIRST THESSALONIANS" => Ok(Self::FirstThessalonians),
                "SECOND THESSALONIANS" => Ok(Self::SecondThessalonians),
                "FIRST TIMOTHY" => Ok(Self::FirstTimothy),
                "SECOND TIMOTHY" => Ok(Self::SecondTimothy),
                "TITUS" => Ok(Self::Titus),
                "PHILEMON" => Ok(Self::Philemon),
                "HEBREWS" => Ok(Self::Hebrews),
                "JAMES" => Ok(Self::James),
                "FIRST PETER" => Ok(Self::FirstPeter),
                "SECOND PETER" => Ok(Self::SecondPeter),
                "FIRST JOHN" => Ok(Self::FirstJohn),
                "SECOND JOHN" => Ok(Self::SecondJohn),
                "THIRD JOHN" => Ok(Self::ThirdJohn),
                "JUDE" => Ok(Self::Jude),
                "REVELATION" => Ok(Self::Revelation),
                _ => Err(anyhow!("Invalid string for book")),
            }
        }
    }

    pub fn decode_translation_data(bytes: &[u8]) -> Result<TranslationData> {
        TranslationData::decode(bytes).context("Decoding Translation Data")
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn to_and_from_bytes() {
            let key = VerseKey {
                book: 4,
                chapter: 5,
                verse: 6,
            };
            let bytes = key.to_be_bytes();
            let decoded = VerseKey::from_be_bytes(&bytes);
            assert_eq!(decoded.book, 4);
            assert_eq!(decoded.chapter, 5);
            assert_eq!(decoded.verse, 6);
        }
    }
}

pub mod service {
    include!(concat!(env!("OUT_DIR"), "/instantbible.service.rs"));
}

pub mod engine {
    use anyhow::{Context, Result};
    use prost::Message;

    include!(concat!(env!("OUT_DIR"), "/instantbible.engine.rs"));

    pub fn decode_index_data(bytes: &[u8]) -> Result<IndexData> {
        IndexData::decode(bytes).context("Decoding Translation Data")
    }
}
