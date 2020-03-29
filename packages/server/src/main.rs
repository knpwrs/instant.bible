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

    let index_pb = get_or_create_index_proto_struct();
    let route_index_pb_data = index_pb.clone();
    let index = Arc::new(VersearchIndex::from_index_data_proto_struct(index_pb));

    info!("Starting server...");

    let route_api = warp::path("api");

    let route_proto = route_api
        .and(warp::header::exact_ignore_case(
            "accept",
            "application/protobuf",
        ))
        .and(search::search_filter(Arc::clone(&index)))
        .map(|res: ServiceResponse| response::protobuf(&res));
    let route_json = route_api
        .and(search::search_filter(Arc::clone(&index)))
        .map(|res: ServiceResponse| warp::reply::json(&res));
    let route_index_pb = route_api
        .and(warp::path("index.pb"))
        .map(move || response::protobuf(&route_index_pb_data));

    let route = route_proto.or(route_json).or(route_index_pb);
    warp::serve(route).run(([0, 0, 0, 0], 8081)).await
}
