use crate::types::ClientGameState;
use ::futures::{stream, Stream};
use ::futures::StreamExt;
use ::actix_web::{
  Error,
  web::Bytes,
};

pub fn to_stream(event: ClientGameState) ->
  impl Stream<Item = Result<Bytes, Error>> {
  stream::once(async {event})
    .map(|value| Ok::<_, Error>(to_bytes(&value)))
}

fn to_bytes(event: &ClientGameState) -> Bytes {
  Bytes::from(format!(
    "event: message\ndata: {}\n\n",
     serde_json::to_string(event).unwrap()
  ))
}
