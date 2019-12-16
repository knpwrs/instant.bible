pub mod data {
    include!(concat!(env!("OUT_DIR"), "/instantbible.data.rs"));
}

pub mod service {
    use std::cmp::Ordering;

    include!(concat!(env!("OUT_DIR"), "/instantbible.service.rs"));

    impl PartialOrd for response::VerseResult {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            self.total_score.partial_cmp(&other.total_score)
        }
    }

    impl Ord for response::VerseResult {
        fn cmp(&self, other: &Self) -> Ordering {
            self.partial_cmp(other).unwrap()
        }
    }

    impl Eq for response::VerseResult {}
}
