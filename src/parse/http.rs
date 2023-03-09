use std::collections::HashMap;

use info_utils::prelude::*;

#[derive(Debug)]
pub enum Method {
    Get,
    Post,
}

#[derive(Debug)]
pub struct Headers {
    method: Method,
    fields: HashMap<String, String>,
}

impl Default for Headers {
    fn default() -> Headers {
        Headers {
            method: Method::Get,
            fields: HashMap::new(),
        }
    }
}

pub fn parse_headers(headers: &String) -> Headers {
    let parts = headers.split("\r\n\r\n").collect::<Vec<&str>>();

    let raw_header = parts[0];

    let method_line = raw_header.lines().nth(0).eval();

    let mut headers: Headers = Headers::default();

    match method_line.split(" ").nth(0).eval() {
        "GET" => headers.method = Method::Get,
        "POST" => headers.method = Method::Post,
        _ => error!("method not supported"),
    };

    for line in raw_header.lines() {
        match line.split_once(":") {
            Some((key, value)) => {
                headers
                    .fields
                    .insert(key.trim().to_string(), value.trim().to_string());
            }
            None => continue,
        }
    }

    headers
}
