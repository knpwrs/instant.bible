pub mod data {
    include!(concat!(env!("OUT_DIR"), "/instantbible.data.rs"));

    impl VerseKey {
        pub fn to_be_bytes(&self) -> Vec<u8> {
            let mut v = Vec::with_capacity(VerseKey::get_byte_size());
            v.extend(&(self.book as u8).to_be_bytes());
            v.extend(&(self.chapter as u8).to_be_bytes());
            v.extend(&(self.verse as u8).to_be_bytes());
            v
        }

        pub const fn get_byte_size() -> usize {
            std::mem::size_of::<u8>() + 3
        }
    }
}

pub mod service {
    include!(concat!(env!("OUT_DIR"), "/instantbible.service.rs"));
}
