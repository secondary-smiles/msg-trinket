use async_std::fs;

use std::path::PathBuf;

use chrono;
use home::home_dir;

use info_utils::prelude::*;

macro_rules! eval_return {
    ( $e:expr ) => {
        match $e {
            Ok(x) => x,
            Err(_) => return Err(()),
        }
    };
}

fn get_fp() -> PathBuf {
    let mut file_path = home_dir().eval_or(PathBuf::from("./"));
    file_path.push("cserver_message");

    file_path
}

fn get_savep() -> PathBuf {
    let mut file_path = home_dir().eval_or(PathBuf::from("./"));
    file_path.push("past_messages");
    let file_name = chrono::Utc::now().format("cserver-message_%Y-%m-%d_%H-%M-%S-%f");
    file_path.push(file_name.to_string());

    file_path
}

pub async fn get_curl() -> Result<String, ()> {
    let contents = match fs::read_to_string(get_fp()).await {
        Ok(v) => v,
        Err(_) => "hello, world!\nno message has been set yet.\n\
            run\n\
            $ curl msg.trinket.icu -d <your message>\n\
            to be the first!"
            .to_string(),
    };

    Ok(contents)
}

pub async fn get_other() -> Result<String, ()> {
    let contents: String = eval_return!(fs::read_to_string("resources/index.html").await);

    let mut message = eval_return!(get_curl().await);
    message = html_escape::encode_text(&message).to_string();

    let contents = contents.replace("{{MESSAGE}}", &message);

    Ok(contents)
}

pub async fn post_curl(data: &String) -> Result<(), ()> {
    let save_path = get_savep();
    let message_path = get_fp();
    let prefix = save_path.parent().eval();

    eval_return!(fs::create_dir_all(prefix).await);

    // Backup current message
    if message_path.exists() {
        eval_return!(fs::copy(&message_path, save_path).await);
    }

    eval_return!(fs::write(&message_path, data).await);

    Ok(())
}

pub enum ResponseCode {
    Good,
    Bad,
}

pub fn resp_header(code: ResponseCode, data: &String) -> String {
    let code = match code {
        ResponseCode::Good => "200 OK",
        ResponseCode::Bad => "500 Internal Server Error",
    };

    let header = format!("HTTP/1.0 {}\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n", code, data.as_bytes().len() + 2);

    header
}
