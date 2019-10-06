use actix_web::{middleware, web, App, HttpResponse, HttpServer};
use bgram::{BGramIndex, JsonVerse};
use lazy_static::lazy_static;
use log::info;
use serde::Deserialize;
use std::fs;
use std::io::BufReader;
use std::time::Instant;

#[derive(Deserialize, Debug)]
struct Config {
    translation_dir: String,
}

#[derive(Deserialize)]
struct SearchQuery {
    q: String,
}

lazy_static! {
    static ref GRAM_INDEX: BGramIndex = {
        let mut gi = BGramIndex::new();
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
                    gi.insert_verse(&translation, verse);
                }
                info!(
                    "Indexed {} verses in {}ms",
                    verses.len(),
                    now.elapsed().as_millis()
                );
            }
        }
        gi
    };
}

fn index(info: web::Query<SearchQuery>) -> HttpResponse {
    info!(r#"Searching for """{}""""#, info.q);
    let now = Instant::now();
    let res = GRAM_INDEX.search(&info.q);
    // format!("{} results in {}ms", res.len(), now.elapsed().as_millis())
    HttpResponse::Ok()
        .header("X-Response-Time", now.elapsed().as_millis() as u64)
        .json(res)
}

fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "bgram=info,actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(web::resource("/").to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
}
