use std::collections::HashMap;

use info_utils::prelude::*;

#[derive(Debug)]
pub enum Method {
    Get,
    Post,
}

#[derive(Debug)]
pub struct Headers {
    pub method: Method,
    pub fields: HashMap<String, String>,
}

impl Default for Headers {
    fn default() -> Headers {
        Headers {
            method: Method::Get,
            fields: HashMap::new(),
        }
    }
}

#[derive(Default, Debug)]
pub struct Req {
    pub headers: Headers,
    pub data: Option<String>,
}

pub fn parse_headers(data: String) -> Headers {
    let method_line = data.lines().nth(0).eval();

    let mut headers: Headers = Headers::default();

    match method_line.split(" ").nth(0).eval() {
        "GET" => headers.method = Method::Get,
        "POST" => headers.method = Method::Post,
        _ => error!("method not supported"),
    };

    for line in data.lines() {
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
