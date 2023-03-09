use async_std::io::prelude::*;

// use async_std::prelude::*;
use info_utils::prelude::*;

use crate::parse::http::{parse_headers, Headers};

pub async fn handle_connection<T: Read + Write + Unpin>(stream: &mut T) {
    let mut stream = stream;

    let header = read_header(&mut stream).await;

    let response = format!("HTTP/1.0 200 OK\r\n\r\n{:?}", header);

    for header in header.fields {
        println!("{:?}", header);
    }

    stream.write(response.as_bytes()).await.eval();
    stream.flush().await.eval();
}

async fn read_header<T: Read + Write + Unpin>(stream: &mut T) -> Headers {
    let mut data = vec![];
    let mut buffer = [0; 1024];

    let bytes_read = stream.read(&mut buffer).await.eval();
    data.extend_from_slice(&buffer[..bytes_read]);

    let data = String::from_utf8_lossy(&data);

    let headers = parse_headers(&data.to_string());

    headers
}
