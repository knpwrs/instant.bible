mod response;
mod search;

use engine::proto::service::Response as ServiceResponse;
use engine::util::get_or_create_index_proto_struct;
use engine::VersearchIndex;
use log::info;
use std::sync::Arc;
use warp::Filter;

#[tokio::main]
async fn main() {
    env_logger::init();

    let index = Arc::new(VersearchIndex::from_index_data_proto_struct(
        get_or_create_index_proto_struct(),
    ));

    info!("Starting server...");

    let route_proto = warp::header::exact_ignore_case("accept", "application/protobuf")
        .and(search::search_filter(Arc::clone(&index)))
        .map(|res: ServiceResponse| response::protobuf(&res));
    let route_json = search::search_filter(Arc::clone(&index))
        .map(|res: ServiceResponse| warp::reply::json(&res));

    let route = route_proto
        .or(route_json)
        .with(warp::cors().allow_any_origin());
    warp::serve(route).run(([0, 0, 0, 0], 8081)).await
}
