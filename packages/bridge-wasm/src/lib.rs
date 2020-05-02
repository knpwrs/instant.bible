use engine::proto::engine::decode_index_data;
use engine::VersearchIndex;
use lazy_static::lazy_static;
use mut_static::MutStatic;
use prost::Message;
use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

lazy_static! {
    static ref ENGINE: MutStatic<VersearchIndex> = MutStatic::new();
}

#[wasm_bindgen]
pub extern "C" fn init(data: &[u8]) {
    let proto = decode_index_data(data).unwrap();
    ENGINE
        .set(VersearchIndex::from_index_data_proto_struct(proto))
        .unwrap();
}

#[wasm_bindgen]
pub extern "C" fn search(query: &str) -> Vec<u8> {
    let res = ENGINE.read().unwrap().search(query);
    let mut buf = Vec::new();
    res.encode(&mut buf).unwrap();
    buf
}
