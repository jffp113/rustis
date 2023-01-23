use std::thread;
use signal_hook::consts::SIGTERM;
use signal_hook::iterator::Signals;
use crate::core::server::Server;

mod core;

#[async_std::main]
async fn main() {
    let server = Server::new("0.0.0.0:6379").await;

    let mut signals = Signals::new(&[SIGTERM]).unwrap();
    let handler = signals.handle();

    thread::spawn(move || {
        //todo catch ctrl + c signal
        for _ in signals.forever() {
            let _ = handler.clone();
            println!("CTRL");
        }
    });


    server.start_blocking().await;

}