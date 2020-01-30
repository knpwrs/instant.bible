use bytes::BytesMut;
use prost::EncodeError;
use prost::Message;
use warp::http::{
  header::{HeaderValue, CONTENT_TYPE},
  StatusCode,
};
use warp::reply::{Reply, Response};

pub struct ProtobufResponse {
  bytes: Result<Vec<u8>, EncodeError>,
}

impl Reply for ProtobufResponse {
  #[inline]
  fn into_response(self) -> Response {
    match self.bytes {
      Ok(buf) => {
        let mut res = Response::new(buf.into());
        res.headers_mut().insert(
          CONTENT_TYPE,
          HeaderValue::from_static("application/protobuf"),
        );
        res
      }
      Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
  }
}

pub fn protobuf<T: Message>(value: &T) -> ProtobufResponse {
  let mut buf = BytesMut::new();
  ProtobufResponse {
    bytes: value.encode(&mut buf).map(|_| buf.to_vec()),
  }
}
