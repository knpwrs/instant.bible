use actix_web::{middleware, web, App, HttpResponse, HttpServer};
use log::info;
use prost::Message;
use serde::Deserialize;
use std::fs;
use std::io::prelude::*;
use std::sync::Arc;
use std::time::Instant;
use versearch::data::TranslationData;
use versearch::VersearchIndex;

#[derive(Deserialize, Debug)]
struct Config {
    #[serde(default = "default_translation_dir")]
    translation_dir: String,
}

fn default_translation_dir() -> String {
    "../text/data".to_string()
}

#[derive(Deserialize)]
struct SearchQuery {
    q: String,
}

fn make_index() -> Arc<VersearchIndex> {
    let mut vi = VersearchIndex::new();
    let config = match envy::from_env::<Config>() {
        Ok(config) => config,
        Err(error) => panic!("{:?}", error),
    };

    info!("Loading translations from {:?}", config.translation_dir);
    for entry in fs::read_dir(config.translation_dir).unwrap() {
        let path = entry.unwrap().path();
        if path.is_file() && path.extension().map(|s| s == "pb").unwrap_or(false) {
            let translation = path
                .file_stem()
                .expect("Could not get file stem")
                .to_string_lossy()
                .to_string();
            // let translation = Translation::from_str(&translation);
            info!("Load translation {:?} from {:?}", translation, path);
            let now = Instant::now();
            let mut file_bytes: Vec<u8> = Vec::new();
            fs::File::open(path)
                .unwrap()
                .read_to_end(&mut file_bytes)
                .unwrap();
            let data = TranslationData::decode(file_bytes).unwrap();
            info!(
                "Read {} verses in {}ms",
                data.verses.len(),
                now.elapsed().as_millis()
            );
            let now = Instant::now();
            for verse in &data.verses {
                vi.insert_verse(verse);
            }
            info!(
                "Indexed {} verses in {}ms",
                data.verses.len(),
                now.elapsed().as_millis()
            );
        }
    }

    Arc::new(vi)
}

fn search(info: web::Query<SearchQuery>, index: web::Data<Arc<VersearchIndex>>) -> HttpResponse {
    info!(r#"Searching for """{}""""#, info.q);
    let now = Instant::now();
    let res = index.search(&info.q);
    let us = now.elapsed().as_micros();
    match res {
        Some(res) => {
            info!(r#"{} results for """{}""" in {}us"#, res.len(), info.q, us);
            HttpResponse::Ok()
                .header("X-Response-Time-us", us as u64)
                .json(res)
        }
        None => {
            info!(r#"No results for """{}""" in {}us"#, info.q, us);
            HttpResponse::NotFound()
                .header("X-Response-Time-us", us as u64)
                .finish()
        }
    }
}

fn download_index_bin(index: web::Data<Arc<VersearchIndex>>) -> HttpResponse {
    match index.index_to_bincode() {
        Ok(bc) => HttpResponse::Ok().body(bc),
        _ => unimplemented!(),
    }
}

fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "versearch=info,actix_web=info");
    env_logger::init();

    let index = make_index();

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .data(index.clone())
            .service(web::resource("/").to(search))
            .service(web::resource("/index.bin").to(download_index_bin))
    })
    .bind("0.0.0.0:8080")?
    .run()
}
