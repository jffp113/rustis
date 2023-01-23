use async_std::io::{BufReader, BufWriter, WriteExt};
use async_std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use async_std::io;
use crate::core::commands::commands::invoke;
use crate::core::resp::{deserialize, serialize};
use crate::core::resp::RespAst::{Error, SimpleString};

pub struct Server {
    tcp_listener: TcpListener,
    //local_addr: &'a str,
    //handler: Option<JoinHandle<()>>,
    shutdown: Arc<AtomicBool>
}

impl Server {
    pub async fn new(address: &str) -> Server {
        let tcp_listener = TcpListener::bind(address).await.unwrap();

        return Server {
            tcp_listener,
            //local_addr: address,
            //handler: None,
            shutdown: Arc::new(AtomicBool::new(false))
        }
    }

/*    pub fn start(&mut self) {
        let handle = thread::spawn(|| {

        });
        self.handler = Some(handle);
    }*/

    pub async fn start_blocking(&self) {
        use async_std::task;

        loop {
            let (stream, _) = self.tcp_listener.accept().await.unwrap();

            if self.shutdown.load(Ordering::Relaxed) {
                return;
            }

            task::spawn( async {
                println!("Spawned");
                handle_connection(stream).await;
            });
        }
    }

    /*pub fn close(mut self) { TODO implemennt
        self.shutdown.store(true, Ordering::Relaxed);

        match self.handler {
            Some(handle) => {
                let _ = TcpStream::connect(self.local_addr);
                let _ = handle.join();
            },
            None => {}
        };
    }*/
}


async fn handle_connection(stream: TcpStream) {
    println!("got new connection");

    let mut client = Client::new(stream);

    loop {
        match deserialize(&mut client.buf_reader).await {
            None => break,
            Some(resp_ast) => {
                invoke(&resp_ast, &mut client).await;
            }
        };
    }
}

/*fn handle_connection_error(err: &dyn Error) {
    println!("error")
}*/

pub struct Client {
    buf_writer: BufWriter<TcpStream>,
    buf_reader: BufReader<TcpStream>
}

impl Client{
    pub fn new(stream: TcpStream) -> Self {
        Client{
            buf_reader: BufReader::new(stream.clone()),
            buf_writer: BufWriter::new(stream),
        }
    }

    pub async fn write(&mut self, buf: &[u8]) -> io::Result<()> {
        self.buf_writer.write(buf).await?;
        self.buf_writer.flush().await?;
        Ok(())
    }

    pub async fn send_error(&mut self, kind: &str, error: &str) {
        let error = Error(kind.to_string(),error.to_string());
        let error_s = serialize(error);
        let _ = self.write(error_s.as_bytes()).await;
    }

    pub async fn send_simple_string(&mut self, content: &str) {
        let response = serialize(SimpleString(content.to_string()));
        let _ = self.write(response.as_bytes()).await;
    }
}