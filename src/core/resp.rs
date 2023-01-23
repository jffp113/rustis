use async_std::io;
use async_std::io::BufReader;
use async_std::io::prelude::BufReadExt;
use crate::core::resp::RespAst::{
    SimpleString,
    Error,
    Integer,
    BulkString,
    NullBulkString,
    Arrays
};
use crate::core::utils::string::remove_line_terminator_string;

#[derive(PartialEq,Debug)]
pub enum RespAst {
    SimpleString(String), //TODO can I use &str??
    Error(String, String),
    Integer(isize),
    BulkString(String),
    NullBulkString,
    Arrays(Vec<RespAst>),
}

pub fn serialize(serialize: RespAst) -> String {   
    match serialize {
        SimpleString(str) => format!("+{}\r\n",str),
        Error(kind, error) => format!("-{} {}\r\n", kind, error),
        Integer(int) => format!(":{}\r\n", int),
        BulkString(str) => format!("${}\r\n{}\r\n",str.len(),str),
        NullBulkString => "$-1\n\r".to_string(),
        Arrays(vec) => unroll_vec(vec),
    }
}

fn unroll_vec(vec: Vec<RespAst>) -> String {
    let mut result = format!("*{}\r\n", vec.len());

    for t in vec {
        result += &serialize(t);
    }

    result
}

pub async fn deserialize<R: io::Read + Unpin>(reader: &mut BufReader<R>) -> Option<RespAst> {
    build_ast(reader).await
}

async fn build_ast<R: io::Read + Unpin>(reader: &mut BufReader<R>) -> Option<RespAst> {
    let line = read_next_line(reader).await;
    let ast = build_terminal_branch(&line, reader).await;

    if ast != None {
        return ast;
    }

    build_array_branch(&line, reader).await
}

async fn build_array_branch<R: io::Read + Unpin>(line: &str, reader: &mut BufReader<R>) -> Option<RespAst> {
    let char = line.chars().nth(0)?;
    let branch = match char {
        '*' => {
            let size: usize = line[1..].parse().unwrap();
            let mut vec = Vec::with_capacity(size);
            for _ in 0..size {
                let line = read_next_line(reader).await;
                vec.push(build_terminal_branch(&line, reader).await?);
            }
            Arrays(vec)
        }
        _ => return None
    };

    Some(branch)
}

async fn build_terminal_branch<R: io::Read + Unpin>(line: &str, reader: &mut BufReader<R>) -> Option<RespAst> {
    let char = line.chars().nth(0)?;
    let branch = match char {
        '+' => SimpleString(line[1..].to_string()),
        '-' => {
            let mut args = line.split(" ");
            let kind = args.next()?;
            let error = args.next()?;
            Error(kind[1..].to_string(),error.to_string())
        }
        ':' => Integer(line[1..].parse().unwrap()),
        '$' => {
            let size: isize = line[1..].parse().unwrap();
            if size == -1 {
                NullBulkString
            } else {
                let line = read_next_line(reader).await;
                BulkString(line)
            }
        }
        _ => return None
    };

    Some(branch)
}

async fn read_next_line<R: io::Read + Unpin>(reader: &mut BufReader<R>) -> String {
    let mut line = String::new();
    reader.read_line(&mut line).await.unwrap();
    return remove_line_terminator_string(line);
}


#[cfg(test)]
mod tests {
    use async_std::task;
    use super::{deserialize, RespAst};
    use super::RespAst::{
        Arrays,
        BulkString,
        Error,
        Integer,
        NullBulkString,
        SimpleString
    };

    #[test]
    fn test_deserialize_simple_string() {
        let ast = deserialize_test("+test\r\n");
        assert_eq!(SimpleString("test".to_string()), ast)
    }

    #[test]
    fn test_deserialize_error() {
        let ast = deserialize_test("-err failed\r\n");
        assert_eq!(Error("err".to_string(),"failed".to_string()), ast)
    }

    #[test]
    fn test_deserialize_integer() {
        let ast = deserialize_test(":1\r\n");
        assert_eq!(Integer(1), ast)
    }

    #[test]
    fn test_deserialize_bulk_string() {
        let ast = deserialize_test("$10\r\nhellohello\r\n");
        assert_eq!(BulkString("hellohello".to_string()), ast)
    }

    #[test]
    fn test_deserialize_null_bulk_string() {
        let ast = deserialize_test("$-1\r\n");
        assert_eq!(NullBulkString, ast)
    }

    #[test]
    fn test_deserialize_array() {
        let ast = deserialize_test("*2\r\n+test1\r\n+test2\r\n");
        let mut v = Vec::new();
        v.push(SimpleString("test1".to_string()));
        v.push(SimpleString("test2".to_string()));
        assert_eq!(Arrays(v), ast)
    }

    fn deserialize_test(token: &str) -> RespAst {
        use async_std::task;
        use async_std::io::BufReader;

        let mut buf_reader = BufReader::new(token.as_bytes());
        task::block_on(deserialize(&mut buf_reader)).unwrap()
    }

}