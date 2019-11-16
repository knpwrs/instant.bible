use actix_protobuf::ProtoBufResponseBuilder;
use actix_web::{
    http, middleware, web, App, HttpRequest, HttpResponse, HttpServer, Result as ActixResult,
};
use log::info;
use serde::Deserialize;
use std::sync::Arc;
use std::time::Instant;
use versearch::proto::service::Response as ServiceResponse;
use versearch::util::get_index;
use versearch::VersearchIndex;

#[derive(Deserialize)]
struct SearchQuery {
    q: String,
}

fn accepts_protbuf(req: HttpRequest) -> bool {
    match req.headers().get(http::header::ACCEPT) {
        Some(header) => match header.to_str() {
            Ok(str) => str.contains("application/protobuf"),
            _ => false,
        },
        _ => false,
    }
}

fn search(
    req: HttpRequest,
    info: web::Query<SearchQuery>,
    index: web::Data<Arc<VersearchIndex>>,
) -> ActixResult<HttpResponse> {
    info!(r#"Searching for """{}""""#, info.q);
    let now = Instant::now();
    let res = index.search(&info.q);
    let us = now.elapsed().as_micros();
    match res {
        Some(res) => {
            info!(r#"{} results for """{}""" in {}us"#, res.len(), info.q, us);
            let mut http_res = HttpResponse::Ok();
            let http_res = http_res.header("X-Response-Time-us", us as u64);
            if accepts_protbuf(req) {
                http_res.protobuf(ServiceResponse { results: res })
            } else {
                Ok(http_res.json(res))
            }
        }
        None => {
            info!(r#"No results for """{}""" in {}us"#, info.q, us);
            Ok(HttpResponse::NotFound()
                .header("X-Response-Time-us", us as u64)
                .finish())
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

    let index = Arc::new(get_index());

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
