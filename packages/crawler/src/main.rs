mod matches;

use anyhow::{Context, Result};
use bytes::buf::BufExt;
use flate2::read::MultiGzDecoder;
use futures::stream::{self, StreamExt};
use matches::get_matches;
use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::io::prelude::*;
use std::sync::Arc;
use std::time::Instant;
use tokio::fs::File;
use tokio::prelude::*;
use tokio::sync::Mutex;
use tokio::task::spawn_blocking;
use warc_parser::records;

type DataMap = BTreeMap<String, BTreeMap<u8, BTreeMap<u8, BTreeSet<u64>>>>;
type MutexMap = Arc<Mutex<DataMap>>;
type CountsMap = BTreeMap<String, BTreeMap<u8, BTreeMap<u8, u32>>>;

static BASE_URL: &str = "https://commoncrawl.s3.amazonaws.com";

/// Hash a given URL into a u64
fn hash_url(url: &str) -> u64 {
    use core::hash::Hasher;
    let mut hasher = metrohash::MetroHash64::new();
    hasher.write(url.as_bytes());
    hasher.finish()
}

/// Given a year and a crawl number, downloads and uncompresses a common crawl
/// WET index
async fn get_index(year: &str, crawl: &str) -> Result<String> {
    log::info!("Getting index {}-{}", year, crawl);

    let url = format!(
        "{}/crawl-data/CC-MAIN-{}-{}/wet.paths.gz",
        BASE_URL, year, crawl,
    );

    log::info!("(H:{}) Downloading {}", hash_url(&url), url);
    let bytes = reqwest::get(&url)
        .await
        .context("Could not download index")?
        .bytes()
        .await
        .context("Could not get bytes from index reponse")?;
    let mut decoder = MultiGzDecoder::new(bytes.reader());
    let mut index = String::new();
    decoder
        .read_to_string(&mut index)
        .context("Could not read index to string")?;

    Ok(index)
}

/// Given the path to a WARC/WET download, downloads, uncompresses, and returns
/// the bytes of the file
async fn get_warc_bytes(path: &str) -> Result<Vec<u8>> {
    let url = format!("{}/{}", BASE_URL, path);
    let hash = hash_url(&url);
    log::info!("(H:{}) Downloading {}", hash, url);

    let bytes = reqwest::get(&url)
        .await
        .context(format!("(H:{}) Could not download WARC file", hash))?
        .bytes()
        .await
        .context(format!(
            "(H:{}) Could not get WARC bytes from response",
            hash
        ))?;
    log::info!("(H:{}) Downloaded {} bytes", hash, bytes.len());
    spawn_blocking(move || {
        log::info!(
            "(H:{}) Decompressing {} bytes on blocking thread",
            hash,
            bytes.len()
        );
        let mut decoder = MultiGzDecoder::new(bytes.reader());
        let mut buf = Vec::new();
        decoder
            .read_to_end(&mut buf)
            .context(format!("(H:{}) Could not decompress WARC bytes", hash))?;
        log::info!("(H:{}) Decompressed to {} bytes ", hash, buf.len());
        Ok(buf)
    })
    .await
    .context("Failure in spawned decompression task")?
}

/// Given a line out of a common crawl index, calls the download function, parses
/// the WARC, and processes the result
async fn process_index_line(i: usize, line: &str, map: MutexMap) -> Result<()> {
    let hash = hash_url(line);
    log::info!("(L:{}) (H:{}) processing line {}", i, hash, line);
    let warc_bytes = match get_warc_bytes(line).await {
        Ok(bytes) => bytes,
        Err(_) => {
            // TODO: Use a better abstracted retry system
            // Wait 10 seconds and try again
            log::error!(
                "(H:{}) Error getting WARC bytes for line. Waiting 10 seconds and trying again.",
                hash
            );
            tokio::time::delay_for(std::time::Duration::from_secs(10)).await;
            get_warc_bytes(line).await.context(format!(
                "(H:{}) Error retrying to get WARC bytes. Giving up.",
                hash
            ))?
        }
    };
    let parsed = records(&warc_bytes);
    match parsed {
        Err(_) => log::error!("(H:{}) Error parsing WARC data!", hash),
        Ok((_i, records)) => {
            log::info!("(H:{}) Processing {} WARC records", hash, records.len());
            log::info!(
                "(H:{}) Converting records to strings and searching with regex on blocking threads",
                hash
            );
            for warc_parser::Record { content, headers } in records {
                let matches = spawn_blocking(move || {
                    let content =
                        String::from_utf8(content).context("Could not parse UTF-8 from WARC")?;
                    get_matches(&content)
                })
                .await
                .context(format!(
                    "(H:{}) Could not get matches from blocking thread",
                    hash
                ))?;
                match matches {
                    Ok(matches) => {
                        if !matches.is_empty() {
                            let url = &headers["WARC-Target-URI"];
                            let url_hash = hash_url(url);
                            log::info!("(H:{}) {} matches at {}", url_hash, matches.len(), url);
                            let map = &mut *map.lock().await;
                            for mat in matches {
                                map.entry(mat.book)
                                    .or_insert_with(BTreeMap::new)
                                    .entry(mat.chapter)
                                    .or_insert_with(BTreeMap::new)
                                    .entry(mat.verse)
                                    .or_insert_with(BTreeSet::new)
                                    .insert(url_hash);
                            }
                        }
                    }
                    Err(err) => log::error!("Error prcessing index line! {:?}", err),
                }
            }
        }
    }
    Ok(())
}

#[cfg(debug_assertions)]
fn get_concurrency() -> usize {
    1
}

#[cfg(not(debug_assertions))]
fn get_concurrency() -> usize {
    num_cpus::get()
}

/// Given a CommonCrawl index, downloads and processes all entries
async fn process_index(index: &str) -> MutexMap {
    let map: DataMap = BTreeMap::new();
    let guarded_map: MutexMap = Arc::new(Mutex::new(map));

    stream::iter(index.lines().enumerate())
        .for_each_concurrent(get_concurrency(), |(i, line)| {
            let map = Arc::clone(&guarded_map);
            async move {
                let hash = hash_url(line);
                match process_index_line(i + 1, line, map).await {
                    Ok(_) => log::info!("(H:{}) Done processing line!", hash),
                    Err(e) => log::error!("(H:{}) Error processing line! {:?}", hash, e),
                }
            }
        })
        .await;

    guarded_map
}

/// Converts a data map to a counts map for serialization
fn make_counts_map(data_map: &DataMap) -> CountsMap {
    let mut counts_map: CountsMap = BTreeMap::new();
    for (book, chapters) in data_map {
        for (chapter, verses) in chapters {
            for (verse, set) in verses {
                counts_map
                    .entry(book.clone())
                    .or_insert_with(BTreeMap::new)
                    .entry(*chapter)
                    .or_insert_with(BTreeMap::new)
                    .insert(*verse, set.len() as u32);
            }
        }
    }
    counts_map
}

#[tokio::main]
async fn main() -> Result<()> {
    let start = Instant::now();
    std::env::set_var("RUST_LOG", "crawler=info");
    env_logger::init();
    let args: Vec<_> = env::args().skip(1).collect();
    let year = &args[0];
    let crawl = &args[1];
    let index = get_index(year, crawl)
        .await
        .context("Could not get index")?;
    let mutex_map = process_index(&index).await;
    let map = &*mutex_map.lock().await;
    let counts = make_counts_map(map);
    let json = serde_json::to_string(&counts).context("Could not convert counts to JSON")?;
    let mut file = File::create(format!("{}-{}.json", year, crawl))
        .await
        .context("Could not create JSON file on disk")?;
    file.write_all(json.as_bytes())
        .await
        .context("Could not write JSON to disk")?;
    log::info!("Done in {}s", start.elapsed().as_secs_f64());

    Ok(())
}
