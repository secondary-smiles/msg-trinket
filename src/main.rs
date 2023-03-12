use async_std::io::prelude::*;
use async_std::net;
use async_std::net::TcpListener;
use futures::stream::StreamExt;

use connection::handle::handle_connection;
use info_utils::prelude::*;

mod connection;
mod parse;

#[async_std::main]
async fn main() {
    let port = 80;
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).await.eval();

    log!("listening on port {}", port);

    listener
        .incoming()
        .for_each_concurrent(/*limit*/ None, |stream| async move {
            let mut stream = match stream {
                Ok(v) => v,
                Err(e) => {
                    warn!("couldn't connect to stream: {}", e);
                    return;
                }
            };

            let connector = match stream.peer_addr() {
                Ok(v) => v.to_string(),
                Err(_) => "<unknown>".to_string(),
            };

            log!("connection from: {}", connector);

            handle_connection(&mut stream).await;
            match stream.flush().await {
                Ok(_) => {}
                Err(e) => {
                    warn!("couldn't flush stream: {}", e)
                }
            }
            match stream.shutdown(net::Shutdown::Both) {
                Ok(_) => {}
                Err(e) => {
                    warn!("couldn't shutdown stream: {}", e)
                }
            };
            log!("closed connection from {}", connector);
        })
        .await;
}
