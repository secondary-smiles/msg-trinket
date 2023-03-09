use async_std::io::prelude::*;
use async_std::net;
use async_std::net::TcpListener;
use futures::stream::StreamExt;

use connection::handle::handle_connection;
use info_utils::prelude::*;

mod connection;

#[async_std::main]
async fn main() {
    let listener = TcpListener::bind("0.0.0.0:4040").await.eval();

    log!("listening on port 4040");

    listener
        .incoming()
        .for_each_concurrent(/*limit*/ None, |stream| async move {
            let mut stream = stream.eval();
            handle_connection(&mut stream).await;
            stream.flush().await.eval();
            stream.shutdown(net::Shutdown::Both).eval();
        })
        .await;
}
