use std::time::Duration;

use async_std::io::prelude::*;
use async_std::net::TcpStream;
use async_std::task;

// use async_std::prelude::*;
use info_utils::prelude::*;

pub async fn handle_connection(stream: &mut TcpStream) {
    log!("{:?}", stream);
    task::sleep(Duration::from_secs(2)).await;

    let mut data = vec![];
    let mut buffer = [0; 4096];

    let bytes_read = stream.read(&mut buffer).await.eval();

    log!("read {} bytes", bytes_read);

    data.extend_from_slice(&buffer[..bytes_read]);

    let data = String::from_utf8_lossy(&data);

    log!("{}", data);

    let response = format!("HTTP/1.0 200 OK\r\n\r\n{}", data);

    stream.write(response.as_bytes()).await.eval();
}
