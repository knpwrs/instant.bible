use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::rc::Rc;
use versearch::util::InterIter;

pub fn intersection(c: &mut Criterion) {
    let numbers1: Rc<Vec<u32>> = Rc::new((1..50_000).collect());
    let numbers2: Rc<Vec<u32>> = Rc::new(numbers1.iter().skip(1000).copied().collect());
    let numbers3: Rc<Vec<u32>> = Rc::new(numbers1.iter().take(40_000).copied().collect());
    let numbers4: Rc<Vec<u32>> = Rc::new(numbers1.iter().step_by(2).copied().collect());
    let numbers5: Rc<Vec<u32>> = Rc::new(numbers1.iter().step_by(3).copied().collect());
    let numbers6: Rc<Vec<u32>> =
        Rc::new(numbers1.iter().skip(10_000).take(20_000).copied().collect());

    c.bench_function("three", |b| {
        let numbers1 = numbers1.clone();
        let numbers2 = numbers2.clone();
        let numbers3 = numbers3.clone();
        b.iter(|| {
            let intersection: Vec<u32> =
                InterIter::new(vec![numbers1.iter(), numbers2.iter(), numbers3.iter()])
                    .copied()
                    .collect();
            black_box(intersection);
        });
    });

    c.bench_function("four", |b| {
        let numbers1 = numbers1.clone();
        let numbers2 = numbers2.clone();
        let numbers3 = numbers3.clone();
        let numbers4 = numbers4.clone();
        b.iter(|| {
            let intersection: Vec<u32> = InterIter::new(vec![
                numbers1.iter(),
                numbers2.iter(),
                numbers3.iter(),
                numbers4.iter(),
            ])
            .copied()
            .collect();
            black_box(intersection);
        });
    });

    c.bench_function("five", |b| {
        let numbers1 = numbers1.clone();
        let numbers2 = numbers2.clone();
        let numbers3 = numbers3.clone();
        let numbers4 = numbers4.clone();
        let numbers5 = numbers5.clone();
        b.iter(|| {
            let intersection: Vec<u32> = InterIter::new(vec![
                numbers1.iter(),
                numbers2.iter(),
                numbers3.iter(),
                numbers4.iter(),
                numbers5.iter(),
            ])
            .copied()
            .collect();
            black_box(intersection);
        });
    });

    c.bench_function("six", |b| {
        let numbers1 = numbers1.clone();
        let numbers2 = numbers2.clone();
        let numbers3 = numbers3.clone();
        let numbers4 = numbers4.clone();
        let numbers5 = numbers5.clone();
        let numbers6 = numbers6.clone();
        b.iter(|| {
            let intersection: Vec<u32> = InterIter::new(vec![
                numbers1.iter(),
                numbers2.iter(),
                numbers3.iter(),
                numbers4.iter(),
                numbers5.iter(),
                numbers6.iter(),
            ])
            .copied()
            .collect();
            black_box(intersection);
        });
    });

    c.bench_function("twelve", |b| {
        let numbers1 = numbers1.clone();
        let numbers2 = numbers2.clone();
        let numbers3 = numbers3.clone();
        let numbers4 = numbers4.clone();
        let numbers5 = numbers5.clone();
        let numbers6 = numbers6.clone();
        b.iter(|| {
            let intersection: Vec<u32> = InterIter::new(vec![
                numbers1.iter(),
                numbers2.iter(),
                numbers3.iter(),
                numbers4.iter(),
                numbers5.iter(),
                numbers6.iter(),
                numbers1.iter(),
                numbers2.iter(),
                numbers3.iter(),
                numbers4.iter(),
                numbers5.iter(),
                numbers6.iter(),
            ])
            .copied()
            .collect();
            black_box(intersection);
        });
    });

    c.bench_function("twenty four", |b| {
        let numbers1 = numbers1.clone();
        let numbers2 = numbers2.clone();
        let numbers3 = numbers3.clone();
        let numbers4 = numbers4.clone();
        let numbers5 = numbers5.clone();
        let numbers6 = numbers6.clone();
        b.iter(|| {
            let intersection: Vec<u32> = InterIter::new(vec![
                numbers1.iter(),
                numbers2.iter(),
                numbers3.iter(),
                numbers4.iter(),
                numbers5.iter(),
                numbers6.iter(),
                numbers1.iter(),
                numbers2.iter(),
                numbers3.iter(),
                numbers4.iter(),
                numbers5.iter(),
                numbers6.iter(),
                numbers1.iter(),
                numbers2.iter(),
                numbers3.iter(),
                numbers4.iter(),
                numbers5.iter(),
                numbers6.iter(),
                numbers1.iter(),
                numbers2.iter(),
                numbers3.iter(),
                numbers4.iter(),
                numbers5.iter(),
                numbers6.iter(),
            ])
            .copied()
            .collect();
            black_box(intersection);
        });
    });
}

criterion_group!(benches, intersection);
criterion_main!(benches);
