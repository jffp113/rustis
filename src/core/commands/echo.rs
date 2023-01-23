use crate::core::server::Client;
use crate::core::resp::{RespAst, serialize};
use crate::core::resp::RespAst::{Arrays, BulkString, Error, SimpleString};
use crate::core::utils::string::remove_line_terminator;

pub const ECHO: &'static str = "echo";

pub async fn echo(request: &RespAst, client: &mut Client) {
    let echo_content = extract_echo_argument(request);

    match echo_content {
        None => client.send_error("ECHO", "Invalid echo command").await,
        Some(echo_content) => client.send_simple_string(echo_content).await,
    };
}


fn extract_echo_argument(request: &RespAst) -> Option<&str> {
    match request {
        Arrays(v) if v.len() == 2 => extract_string_from(&v[1]),
        _ => None,
    }
}

fn extract_string_from(request: &RespAst) -> Option<&str> {
    let s = match request {
        SimpleString(s) => Some(s),
        BulkString(s) => Some(s),
        _ => None
    }?;

    let a = remove_line_terminator(s);

    return Some(a)
}

