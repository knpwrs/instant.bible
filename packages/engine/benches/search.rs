use criterion::{criterion_group, criterion_main, Criterion};
use engine::util::create_index_proto_struct;
use engine::VersearchIndex;

pub fn search_benches(c: &mut Criterion) {
    let data = create_index_proto_struct();
    let idx = VersearchIndex::from_index_data_proto_struct(data);
    for name in &["thou", "thou shalt", "large letters", "I", "a", "b", "c"] {
        c.bench_function(name, |b| b.iter(|| idx.search(name)));
    }
}

criterion_group!(benches, search_benches);
criterion_main!(benches);
