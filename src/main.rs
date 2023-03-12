use async_std::io::prelude::*;
use async_std::net;
use async_std::net::TcpListener;
use futures::stream::StreamExt;

use connection::handle::handle_connection;
use rate::ratelimiter::RateLimiter;

use info_utils::prelude::*;

mod connection;
mod parse;
mod rate;

#[async_std::main]
async fn main() {
    let port = 4040;
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).await.eval();

    let ratelimiter: RateLimiter = RateLimiter::default();
    
    log!("listening on port {}", port);

    listener
        .incoming()
        .for_each_concurrent(/*limit*/ None, |stream| async {
            let mut ratelimiter = ratelimiter.clone();
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

            handle_connection(&mut stream, &mut ratelimiter).await;
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
