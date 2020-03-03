use anyhow::{Context, Result};
use bytes::BytesMut;
use engine::util::create_index_proto_struct;
use prost::Message;
use std::fs;

fn main() -> Result<()> {
    env_logger::init();
    let data = create_index_proto_struct();
    let mut buf = BytesMut::new();
    let bytes = data
        .encode(&mut buf)
        .map(|_| buf.to_vec())
        .context("Could not encode index data protobuf")?;
    fs::write("index.pb", bytes).context("Could not write bytes to file")?;
    Ok(())
}
