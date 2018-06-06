extern crate hyper;
extern crate bbs;

use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::net::SocketAddr;

use hyper::{Body, Method, Server, StatusCode, Request, Response};
use hyper::rt::Future;
use hyper::service::service_fn_ok;

use bbs::Message;
use bbs::{SERVER_ADDR, BOT_ADDR, HTML_DATA, HTML_HEADER, HTML_FOOTER};

// Returns val from Ok(val) or sets the response to return an InternalServerError.
macro_rules! try_or_server_err {
    ($expr:expr, $res:expr) => (match $expr {
        Ok(val) => val,
        Err(err) => {
            println!("{:?}", err);
            *($res).status_mut() = StatusCode::InternalServerError;
            return;
        }
    })
}

fn req_handler(req: Request<Body>) -> Response<Body> {
    let mut resp = Response::builder();

    match *req.method() {
        Method::GET => {
            // Read the files [HTML_HEADER, HTML_DATA, HTML_FOOTER] into buf.
            // If HTML_DATA doesn't exist, it should be skipped without failure.
            // Use `try_or_server_err!(expression, res)` instead of `try!(expression)` in
            // order to return an internal server error.
            let mut buf = String::new();
            // TODO

            // And return buf as the response.
            resp.status(StatusCode::OK).body(Body::from("Hello")).expect("send response error")
        },
        Method::POST => {
            // Read the message out of the `req` into a buffer, handle it, and respond with Ok.
            // TODO
            resp.status(StatusCode::NOT_FOUND).body(Body::from("No Post")).expect("send response error")
        },
        _ => resp.status(StatusCode::IM_A_TEAPOT).body(Body::from("It's a teapot.")).expect("send response error"),
    }
}

fn main() {
    println!("Listening on {}.", SERVER_ADDR);
    let address = SERVER_ADDR.parse().expect("parse error for server address");
    let server = Server::bind(&address)
        .serve(|| service_fn_ok(req_handler))
        .map_err(|e| eprintln!("server error: {}", e));

    // Run this server
    hyper::rt::run(server);
}
