use async_std::{io::prelude::*, net::TcpStream};

use info_utils::prelude::*;

use crate::connection::response;
use crate::parse::http::{parse_headers, Method, Req};

pub async fn handle_connection(stream: &mut TcpStream) {
    let mut stream = stream;

    let req = match read_req(&mut stream).await {
        Ok(v) => v,
        Err(_) => {
            send_error(&mut stream).await;
            return;
        }
    };

    log!("{:#?}", req);

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
                message = match response::get_curl().await {
                    Ok(v) => {
                        log!("curl GET");
                        v
                    }
                    Err(_) => {
                        send_error(&mut stream).await;
                        return;
                    }
                };
            } else {
                message = match response::get_other().await {
                    Ok(v) => {
                        log!("other GET");
                        v
                    }
                    Err(_) => {
                        send_error(&mut stream).await;
                        return;
                    }
                };
            }
        }

        Method::Post => match response::post_curl(&req.data.eval_or_default()).await {
            Ok(_) => {
                log!("successfull POST");
                message = "uploaded your message successfully.".to_string()
            }
            Err(_) => {
                send_error(&mut stream).await;
                return;
            }
        },
    }

    send(
        &mut stream,
        format!(
            "{}{}\r\n",
            response::resp_header(response::ResponseCode::Good, &message),
            message
        ),
    )
    .await;
}

async fn send_error(stream: &mut TcpStream) {
    let mut stream = stream;
    warn!("caught server error");
    let error = "server error. try again later";
    send(
        &mut stream,
        format!(
            "{}{}\r\n",
            response::resp_header(response::ResponseCode::Bad, &error.to_string()),
            error
        ),
    )
    .await;
}

async fn send(stream: &mut TcpStream, message: String) {
    stream.write(message.as_bytes()).await.eval_or_default();
    stream.flush().await.eval_or_default();
}

async fn read_req(stream: &mut TcpStream) -> Result<Req, std::io::Error> {
    let stream = stream;
    let mut data = vec![];
    let mut buffer = [0; 8192];

    let bytes_read = stream.read(&mut buffer).await?;
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

    Ok(req)
}
