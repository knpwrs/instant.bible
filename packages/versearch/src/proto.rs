pub mod data {
    use serde::{Deserialize, Serialize};
    include!(concat!(env!("OUT_DIR"), "/instantbible.data.rs"));
}

pub mod service {
    include!(concat!(env!("OUT_DIR"), "/instantbible.service.rs"));
}
