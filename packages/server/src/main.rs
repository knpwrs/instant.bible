use actix_protobuf::ProtoBufResponseBuilder;
use actix_web::{
    http, middleware, web, App, HttpRequest, HttpResponse, HttpServer, Result as ActixResult,
};
use engine::util::get_index;
use engine::VersearchIndex;
use log::info;
use serde::Deserialize;
use std::sync::Arc;

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
    let res = index.search(&info.q);
    let mut http_res = HttpResponse::Ok();
    if accepts_protbuf(req) {
        http_res.protobuf(res)
    } else {
        Ok(http_res.json(res))
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
    })
    .bind("0.0.0.0:8080")?
    .run()
}
