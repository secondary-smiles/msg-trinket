use async_std::{io::prelude::*, net::TcpStream};

// use async_std::prelude::*;
use info_utils::prelude::*;

use crate::connection::response;
use crate::parse::http::{parse_headers, Method, Req};

pub async fn handle_connection(stream: &mut TcpStream) {
    let mut stream = stream;

    let req = read_req(&mut stream).await;

    log!("{:?}", req.data);

    let message;

    match req.headers.method {
        Method::Get => {
            if req
                .headers
                .fields
                .get("User-Agent")
                .eval_or(&"".to_string())
                .contains("curl")
            {
                message = response::get_curl().await.eval();
            } else {
                message = response::get_curl().await.eval();
            }
        }

        Method::Post => match response::post_curl(&req.data.eval_or_default()).await {
            Ok(_) => message = "uploaded.".to_string(),
            Err(_) => {
                send(
                    &mut stream,
                    format!(
                        "{}{}\r\n",
                        response::resp_header(response::ResponseCode::Bad),
                        "error uploading message."
                    ),
                )
                .await;
                return;
            }
        },
    }

    send(
        &mut stream,
        format!(
            "{}{}\r\n",
            response::resp_header(response::ResponseCode::Good),
            message
        ),
    )
    .await;
}

async fn send(stream: &mut TcpStream, message: String) {
    stream.write(message.as_bytes()).await.eval();
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
