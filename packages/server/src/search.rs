use engine::proto::service::Response as ServiceResponse;
use engine::VersearchIndex;
use log::info;
use std::sync::Arc;
use warp::{Filter, Rejection};

#[derive(serde::Deserialize)]
struct Query {
  q: String,
}

pub fn search_filter(
  index: Arc<VersearchIndex>,
) -> impl Filter<Extract = (ServiceResponse,), Error = Rejection> + Clone {
  warp::filters::query::query::<Query>().map(move |Query { q }| {
    info!(r#"Searching for """{}""""#, q);
    index.search(&q)
  })
}
