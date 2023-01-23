use crate::core::commands::echo::{echo,ECHO};
use crate::core::commands::ping::{ping, PING};
use crate::core::resp::RespAst;
use crate::core::resp::RespAst::{Arrays, BulkString, SimpleString};
use crate::core::server::Client;
use crate::core::utils::string::remove_line_terminator;


pub async fn invoke(ast: &RespAst, client: &mut Client) -> Option<()> {
    let command = get_command(&ast)?;
    let command = remove_line_terminator(command);
    println!("Invoking {}", command);
    match command {
        PING => ping(ast,client).await,
        ECHO => echo(ast,client).await,
        _ => {}
    };

    return None
}

fn get_command(resp_ast: &RespAst) -> Option<&str> {
    return get_command_rec(resp_ast,0)
}

fn get_command_rec(resp_ast: &RespAst, depth: usize) -> Option<&str> {
    match resp_ast {
        SimpleString(command) => Some(command),
        BulkString(command) => Some(command),
        Arrays(v) if depth == 0 =>
            Some(get_command_rec(v.first()?,depth + 1)?),
        Arrays(_v) if depth > 0 => None, //TODO I think this is not necessary
        _ => None
    }
}

