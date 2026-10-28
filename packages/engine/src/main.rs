use anyhow::{Context, Result};
use engine::util::create_index_proto_struct;
use prost::Message;
use std::fs;

fn main() -> Result<()> {
    env_logger::init();
    let data = create_index_proto_struct();
    let mut bytes = Vec::with_capacity(data.encoded_len());
    data.encode(&mut bytes)
        .context("Could not encode index data protobuf")?;
    drop(data);
    fs::write("index.pb", bytes).context("Could not write bytes to file")?;
    Ok(())
}
