use criterion::{criterion_group, criterion_main, Criterion};
use engine::util::tokenize;

static ESTHER_8_9: &str = "Then were the king's scribes called at that time in the third month, that is, the month Sivan, on the three and twentieth day thereof; and it was written according to all that Mordecai commanded unto the Jews, and to the lieutenants, and the deputies and rulers of the provinces which are from India unto Ethiopia, an hundred twenty and seven provinces, unto every province according to the writing thereof, and unto every people after their language, and to the Jews according to their writing, and according to their language.";

pub fn tokenize_bench(c: &mut Criterion) {
    c.bench_function("Esther 8:9", |b| b.iter(|| tokenize(ESTHER_8_9)));
}

criterion_group!(benches, tokenize_bench);
criterion_main!(benches);
