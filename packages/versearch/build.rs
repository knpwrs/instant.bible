fn main() {
    let mut prost = prost_build::Config::new();
    prost.type_attribute(
        ".instantbible.data.VerseKey",
        "#[derive(Deserialize, Serialize, Eq, PartialOrd, Ord, Copy)]",
    );
    prost
        .compile_protos(
            &["../proto/data.proto", "../proto/service.proto"],
            &["../proto"],
        )
        .unwrap();
}
