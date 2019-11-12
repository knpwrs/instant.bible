fn main() {
    let mut prost = prost_build::Config::new();
    prost.type_attribute(
        ".instantbible.data.VerseKey",
        "#[derive(Deserialize, Serialize, Eq, PartialOrd, Ord)]",
    );
    prost
        .compile_protos(&["../proto/data.proto"], &["../proto"])
        .unwrap();
}
