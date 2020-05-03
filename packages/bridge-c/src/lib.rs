use engine::proto::engine::decode_index_data;
use engine::VersearchIndex;
use ffi_support::ByteBuffer as FfiBuffer;
use lazy_static::lazy_static;
use mut_static::MutStatic;
use prost::Message;
use std::ffi::CStr;
use std::os::raw::c_char;

lazy_static! {
    static ref ENGINE: MutStatic<VersearchIndex> = MutStatic::new();
}

#[no_mangle]
pub unsafe extern "C" fn bridge_init(raw_data: *const u8, len: usize) {
    let data = std::slice::from_raw_parts(raw_data, len);
    let proto = decode_index_data(data).unwrap();
    ENGINE
        .set(VersearchIndex::from_index_data_proto_struct(proto))
        .unwrap();
}

#[no_mangle]
pub unsafe extern "C" fn bridge_search(bytes: *const c_char) -> FfiBuffer {
    let c_str = CStr::from_ptr(bytes);
    let query = match c_str.to_str() {
        Err(_) => "",
        Ok(string) => string,
    };
    let res = ENGINE.read().unwrap().search(query);
    let mut buf = Vec::new();
    res.encode(&mut buf).unwrap();
    FfiBuffer::from_vec(buf)
}

#[no_mangle]
pub extern "C" fn bridge_search_free(buf: FfiBuffer) {
    std::mem::forget(buf);
}
