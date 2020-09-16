use engine::proto::engine::decode_index_data;
use engine::VersearchIndex;
use ffi_support::ByteBuffer as FfiBuffer;
use jni::objects::{JObject, JString};
use jni::sys::jbyteArray;
use jni::JNIEnv;
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

unsafe fn bridge_search_internal(query: &str) -> Vec<u8> {
    let res = ENGINE.read().unwrap().search(query);
    let mut buf = Vec::new();
    res.encode(&mut buf).unwrap();

    buf
}

#[no_mangle]
pub unsafe extern "C" fn bridge_search(bytes: *const c_char) -> FfiBuffer {
    let c_str = CStr::from_ptr(bytes);
    let query = match c_str.to_str() {
        Err(_) => "",
        Ok(string) => string,
    };
    let buf = bridge_search_internal(query);
    FfiBuffer::from_vec(buf)
}

#[no_mangle]
pub extern "C" fn bridge_search_free(buf: FfiBuffer) {
    std::mem::forget(buf);
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "system" fn Java_bible_instant_ui_main_MainViewModel_bridgeInit(
    env: JNIEnv,
    _class: JObject,
    arr: jbyteArray,
) {
    let bytes = env.convert_byte_array(arr).unwrap();
    bridge_init(bytes.as_ptr(), bytes.len());
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "system" fn Java_bible_instant_ui_main_MainViewModel_bridgeSearch(
    env: JNIEnv,
    _class: JObject,
    q: JString,
) -> jbyteArray {
    let q_string: String = env.get_string(q).expect("Couldn't get Java string!").into();
    let buf = bridge_search_internal(&q_string);
    env.byte_array_from_slice(&buf).unwrap()
}
