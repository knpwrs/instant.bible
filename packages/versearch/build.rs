fn main() {
    let mut prost = prost_build::Config::new();
    prost.type_attribute(
        ".instantbible.data.VerseKey",
        "#[derive(serde::Serialize, Eq, PartialOrd, Ord, Copy)]",
    );
    prost.type_attribute(
        ".instantbible.service.Response",
        "#[derive(serde::Serialize)]",
    );
    prost
        .compile_protos(
            &["../proto/data.proto", "../proto/service.proto"],
            &["../proto"],
        )
        .unwrap();
}
