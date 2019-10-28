use actix_web::{middleware, web, App, HttpResponse, HttpServer};
use log::info;
use serde::Deserialize;
use std::fs;
use std::io::BufReader;
use std::sync::Arc;
use std::time::Instant;
use versearch::data::JsonVerse;
use versearch::VersearchIndex;

#[derive(Deserialize, Debug)]
struct Config {
    translation_dir: String,
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
        if path.is_file() && path.extension().map(|s| s == "json").unwrap_or(false) {
            let translation = path.file_stem().unwrap().to_string_lossy().to_string();
            info!("Load translation {} from {:?}", translation, path);
            let now = Instant::now();
            let file = fs::File::open(path).unwrap();
            let reader = BufReader::new(file);
            let verses: Vec<JsonVerse> = serde_json::from_reader(reader).unwrap();
            info!(
                "Read {} verses in {}ms",
                verses.len(),
                now.elapsed().as_millis()
            );
            let now = Instant::now();
            for verse in &verses {
                vi.insert_verse(verse);
            }
            info!(
                "Indexed {} verses in {}ms",
                verses.len(),
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
    let ms = now.elapsed().as_millis();
    match res {
        Some(res) => {
            info!(r#"{} results for """{}""" in {}ms"#, res.len(), info.q, ms);
            HttpResponse::Ok()
                .header("X-Response-Time", ms as u64)
                .json(res)
        }
        None => {
            info!(r#"No results for """{}""" in {}ms"#, info.q, ms);
            HttpResponse::NotFound()
                .header("X-Response-Time", ms as u64)
                .finish()
        }
    }
}

fn download_index_json(index: web::Data<Arc<VersearchIndex>>) -> HttpResponse {
    match index.index_to_json() {
        Ok(txt) => HttpResponse::Ok().body(txt),
        _ => unimplemented!(),
    }
}

fn download_index_bc(index: web::Data<Arc<VersearchIndex>>) -> HttpResponse {
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
            .service(web::resource("/index.json").to(download_index_json))
            .service(web::resource("/index.bc").to(download_index_bc))
    })
    .bind("0.0.0.0:8080")?
    .run()
}
