use async_std::{io::prelude::*, net::TcpStream};

// use async_std::prelude::*;
use info_utils::prelude::*;

use crate::parse::http::{parse_headers, Method, Req};

pub async fn handle_connection(stream: &mut TcpStream) {
    let mut stream = stream;

    let req = read_req(&mut stream).await;

    log!("{:?}", req.data);

    let message;

    if req
        .headers
        .fields
        .get("User-Agent")
        .eval_or(&"".to_string())
        .contains("curl")
    {
        message = "ur curl";
    } else {
        message = "ur not curl";
    }

    let response = format!("HTTP/1.0 200 OK\r\n\r\n{}\n", message);

    stream.write(response.as_bytes()).await.eval();
    stream.flush().await.eval();
}

async fn read_req(stream: &mut TcpStream) -> Req {
    let stream = stream;
    let mut data = vec![];
    let mut buffer = [0; 1024];

    let bytes_read = stream.read(&mut buffer).await.eval();
    data.extend_from_slice(&buffer[..bytes_read]);

    let data = String::from_utf8_lossy(&data);

    let parts = data.split("\r\n\r\n").collect::<Vec<&str>>();

    let headers = parse_headers(parts[0].to_string());

    let mut req: Req = Req::default();

    match headers.method {
        Method::Post => {
            req.data = Some(parts[1].to_string());
        }
        _ => {}
    }

    req.headers = headers;

    req
}
