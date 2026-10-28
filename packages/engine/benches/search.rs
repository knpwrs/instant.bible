use criterion::{criterion_group, criterion_main, Criterion};
use engine::VersearchIndex;

pub fn search_benches(c: &mut Criterion) {
    let idx = VersearchIndex::get_or_create();
    for name in &["thou", "thou shalt", "large letters", "I", "a", "b", "c"] {
        c.bench_function(name, |b| b.iter(|| idx.search(name)));
    }
}

criterion_group!(benches, search_benches);
criterion_main!(benches);
