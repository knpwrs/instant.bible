use criterion::{black_box, criterion_group, criterion_main, Criterion};
use versearch::util::tokenize;

pub fn bench_tokenize(c: &mut Criterion) {
    c.bench_function("tokenize hello world", |b| {
        b.iter(|| {
            let tokens = tokenize("Hello, World!");
            black_box(tokens);
        });
    });

    c.bench_function("tokenize thou shalnt foo", |b| {
        b.iter(|| {
            let tokens = tokenize("Thou shaln't foo!");
            black_box(tokens);
        });
    });
}

criterion_group!(benches, bench_tokenize);
criterion_main!(benches);
