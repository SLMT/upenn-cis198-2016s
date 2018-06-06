extern crate hyper;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate futures;

use hyper::{Body, StatusCode, Uri, Result};
use hyper::rt::{Future, Stream};
use hyper::client::{Client, HttpConnector};

pub const SERVER_ADDR: &'static str = "127.0.0.1:1980";
pub const BOT_ADDR: &'static str = "127.0.0.1:1981";
pub const HTML_ADDR: &'static str = "http://127.0.0.1:1980";

pub const HTML_DATA: &'static str = "data/index.html";
pub const HTML_HEADER: &'static str = "html/header.html";
pub const HTML_FOOTER: &'static str = "html/footer.html";


#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub user: String,
    pub text: String,
}

impl Message {
    pub fn new(user: String, text: String) -> Message {
        Message {
            text: text,
            user: user,
        }
    }
}

pub struct UserClient {
    username: String,
    server_addr: String,
    client: Client<HttpConnector, Body>,
}

impl UserClient {
    pub fn new(username: String, server_addr: String) -> UserClient {
        UserClient {
            username: username,
            server_addr: server_addr,
            client: hyper::Client::new(),
        }
    }

    // TODO: Implement send_msg

    pub fn get_content(&self) -> Result<(StatusCode, String)> {
        let response = self.client.get(self.server_addr.parse::<Uri>().unwrap()).wait()?;
        let status = response.status();
        let chunks = response.into_body().collect().wait()?;
        let string = chunks.iter().fold(String::new(), |mut buf, chunk| {
            buf.push_str(std::str::from_utf8(chunk).unwrap());
            buf
        });
        Ok((status, string))
    }
}
