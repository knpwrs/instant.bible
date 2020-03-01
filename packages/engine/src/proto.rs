pub mod data {
    use anyhow::{Context, Result};
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
