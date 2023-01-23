use std::string::ToString;
use crate::core::server::Client;
use crate::core::resp::{RespAst, serialize};
use crate::core::resp::RespAst::SimpleString;

pub const PING: &'static str = "ping";
const PONG: &'static str = "PONG";

pub async fn ping(_request: &RespAst, client: &mut Client) {
    let response = serialize(SimpleString(PONG.to_string()));
    let _ = client.write(response.as_bytes()).await;
}