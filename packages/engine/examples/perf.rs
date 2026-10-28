//! Benchmark / verification harness for the search engine.
//!
//! Usage (run from the `packages` directory, `index.pb` and `$TRANSLATION_DIR` must exist):
//!
//!   cargo run --release -p engine --example perf -- load
//!   cargo run --release -p engine --example perf -- search
//!   cargo run --release -p engine --example perf -- dump > results.txt
//!
//! Set `PERF_LOAD_PROTO=1` to load through the decoded protobuf struct instead of the raw bytes.
//!
//! A counting global allocator tracks live and peak heap usage so memory numbers are exact and
//! platform independent.
use criterion::black_box;
use engine::proto::data::decode_translation_data;
use engine::proto::engine::decode_index_data;
use engine::VersearchIndex;
use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};
use std::time::{Duration, Instant};

struct Counting;

static LIVE: AtomicUsize = AtomicUsize::new(0);
static PEAK: AtomicUsize = AtomicUsize::new(0);
static ALLOCS: AtomicUsize = AtomicUsize::new(0);

unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let p = System.alloc(layout);
        if !p.is_null() {
            let live = LIVE.fetch_add(layout.size(), Relaxed) + layout.size();
            PEAK.fetch_max(live, Relaxed);
            ALLOCS.fetch_add(1, Relaxed);
        }
        p
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout);
        LIVE.fetch_sub(layout.size(), Relaxed);
    }
    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        let p = System.realloc(ptr, layout, new_size);
        if !p.is_null() {
            if new_size > layout.size() {
                let live =
                    LIVE.fetch_add(new_size - layout.size(), Relaxed) + new_size - layout.size();
                PEAK.fetch_max(live, Relaxed);
            } else {
                LIVE.fetch_sub(layout.size() - new_size, Relaxed);
            }
            ALLOCS.fetch_add(1, Relaxed);
        }
        p
    }
}

#[global_allocator]
static GLOBAL: Counting = Counting;

fn mb(b: usize) -> f64 {
    b as f64 / (1024.0 * 1024.0)
}

fn load_index() -> VersearchIndex {
    let bytes = std::fs::read("index.pb").expect("index.pb missing (run from packages/)");
    if std::env::var("PERF_LOAD_PROTO").map_or(false, |v| v == "1") {
        let proto = decode_index_data(&bytes).unwrap();
        drop(bytes);
        VersearchIndex::from_index_data_proto_struct(proto)
    } else {
        VersearchIndex::from_index_bytes(&bytes).unwrap()
    }
}

/// Tiny deterministic PRNG so every build sees the same query set
struct Lcg(u64);
impl Lcg {
    fn next(&mut self) -> u64 {
        self.0 = self
            .0
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        self.0 >> 33
    }
}

/// Builds a realistic query workload: every keystroke prefix while typing the start of random
/// verses (that is how instant.bible is used), plus typo'd words and a set of fixed queries.
fn queries() -> Vec<String> {
    let mut qs: Vec<String> = [
        "thou",
        "thou shalt",
        "large letters",
        "I",
        "a",
        "b",
        "c",
        "for god so loved",
        "jesus wept",
        "in the beginning",
        "love is patient",
        "the lord is my shepherd",
        "be strong and courageous",
        "faith hope love",
        "lamb of god",
        "it's",
        "its",
        "lvoe",
        "shephred",
        "beginnign",
        "rightousness",
        "I am",
        "the",
        "and the",
        "xyzzy",
        "",
        "   ",
        "!!!",
        "a b c d e f g h i j k l m n o p q r s t u v w x y z",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();

    let dir = std::env::var("TRANSLATION_DIR").expect("TRANSLATION_DIR not set");
    let kjv = std::fs::read(format!("{}/KJV.pb", dir)).unwrap();
    let verses = decode_translation_data(&kjv).unwrap().verses;
    let mut rng = Lcg(0x5eed);
    for _ in 0..60 {
        let v = &verses[rng.next() as usize % verses.len()];
        let words: Vec<&str> = v.text.split_whitespace().take(6).collect();
        let phrase = words.join(" ");
        // Every keystroke prefix
        for (i, _) in phrase.char_indices().skip(1) {
            qs.push(phrase[..i].to_string());
        }
        qs.push(phrase.clone());
        // A typo'd version: swap two adjacent letters in the longest word
        if let Some(w) = words.iter().max_by_key(|w| w.len()) {
            if w.len() >= 4 && w.is_ascii() {
                let mut b = w.as_bytes().to_vec();
                let i = 1 + rng.next() as usize % (b.len() - 2);
                b.swap(i, i + 1);
                qs.push(phrase.replace(w, std::str::from_utf8(&b).unwrap()));
            }
        }
    }
    qs
}

fn cmd_load() {
    let start = Instant::now();
    let index = load_index();
    let elapsed = start.elapsed();
    let peak = PEAK.load(Relaxed);
    let live = LIVE.load(Relaxed);
    println!("load_time_ms        {:.1}", elapsed.as_secs_f64() * 1000.0);
    println!("heap_live_after_mb  {:.2}", mb(live));
    println!("heap_peak_load_mb   {:.2}", mb(peak));
    println!("allocations_total   {}", ALLOCS.load(Relaxed));
    drop(index);
}

fn time_query(index: &VersearchIndex, q: &str, min: Duration) -> (f64, u32) {
    // Warm up
    black_box(index.search(q));
    let mut iters = 0u32;
    let start = Instant::now();
    while start.elapsed() < min || iters < 3 {
        black_box(index.search(black_box(q)));
        iters += 1;
    }
    (start.elapsed().as_secs_f64() * 1e6 / iters as f64, iters)
}

fn cmd_search() {
    let index = load_index();
    let qs = queries();
    let base_live = LIVE.load(Relaxed);
    PEAK.store(base_live, Relaxed);

    println!("-- fixed queries (mean us/query) --");
    for q in &[
        "thou",
        "thou shalt",
        "large letters",
        "I",
        "a",
        "b",
        "c",
        "for god so loved",
        "shephred",
        "the",
    ] {
        let (us, _) = time_query(&index, q, Duration::from_millis(300));
        println!("{:<20} {:>10.1}", format!("{:?}", q), us);
    }

    // Whole workload, several passes
    let mut lat: Vec<f64> = Vec::with_capacity(qs.len());
    for q in &qs {
        let (us, _) = time_query(&index, q, Duration::from_millis(5));
        lat.push(us);
    }
    let total: f64 = lat.iter().sum();
    lat.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let pct = |p: f64| lat[((lat.len() as f64 - 1.0) * p) as usize];
    println!(
        "-- workload: {} queries (typing prefixes + typos) --",
        qs.len()
    );
    println!("mean_us             {:.1}", total / lat.len() as f64);
    println!("p50_us              {:.1}", pct(0.5));
    println!("p90_us              {:.1}", pct(0.9));
    println!("p99_us              {:.1}", pct(0.99));
    println!("max_us              {:.1}", pct(1.0));
    println!(
        "search_heap_peak_mb {:.2}",
        mb(PEAK.load(Relaxed).saturating_sub(base_live))
    );
}

fn cmd_slowest() {
    let index = load_index();
    let mut lat: Vec<(f64, String)> = queries()
        .into_iter()
        .map(|q| (time_query(&index, &q, Duration::from_millis(5)).0, q))
        .collect();
    lat.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
    for (us, q) in lat.iter().take(25) {
        println!("{:>10.1} {:?}", us, q);
    }
}

fn cmd_dump() {
    let index = load_index();
    for q in queries() {
        let res = index.search(&q);
        println!("Q {:?} n={}", q, res.results.len());
        for r in res.results {
            let mut hl = r.highlights.clone();
            hl.sort();
            hl.dedup();
            println!(
                "  {:?} top={} pop={} rank={:?} hl={:?} text={:?}",
                r.key.unwrap(),
                r.top_translation,
                r.popularity,
                r.rankings
                    .iter()
                    .map(|x| (x.query_words, x.exact, x.typos))
                    .collect::<Vec<_>>(),
                hl,
                r.text
            );
        }
    }
}

fn main() {
    match std::env::args().nth(1).as_deref() {
        Some("load") => cmd_load(),
        Some("search") => cmd_search(),
        Some("dump") => cmd_dump(),
        Some("slowest") => cmd_slowest(),
        _ => eprintln!("usage: perf load|search|dump|slowest"),
    }
}
