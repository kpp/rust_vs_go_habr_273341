extern crate hyper;
extern crate regex;

#[macro_use] extern crate lazy_static;

use std::io::Write;
use regex::{Regex, Captures};

use hyper::Server;
use hyper::server::{Request, Response};
use hyper::net::Fresh;
use hyper::uri::RequestUri::{AbsolutePath};

fn handler(req: Request, res: Response<Fresh>) {
    lazy_static! {
        static ref GREETING_RE: Regex = Regex::new(r"^/greeting/([a-z]+)$").unwrap();
    }

    match req.uri {
        AbsolutePath(ref path) => match (&req.method, &path[..]) {
            (&hyper::Get, "/") => {
                hello(&req, res);
            },
            _ => {
                greet(&req, res, GREETING_RE.captures(path).unwrap());
            }
        },
        _ => {
            not_found(&req, res);
        }
    };
}

fn hello(_: &Request, res: Response<Fresh>) {
    let mut r = res.start().unwrap();
    r.write_all(b"Hello World!").unwrap();
    r.end().unwrap();
}

fn greet(_: &Request, res: Response<Fresh>, cap: Captures) {
    let mut r = res.start().unwrap();
    r.write_all(format!("Hello, {}", cap.at(1).unwrap()).as_bytes()).unwrap();
    r.end().unwrap();
}

fn not_found(_: &Request, mut res: Response<Fresh>) {
    *res.status_mut() = hyper::NotFound;
    let mut r = res.start().unwrap();
    r.write_all(b"Not Found\n").unwrap();
}

fn main() {
    let _ = Server::http("127.0.0.1:3000").unwrap().handle(handler);
}
