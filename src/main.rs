extern crate hyper;
extern crate futures;
extern crate regex;

use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::env;
use std::path::PathBuf;
use regex::Regex;
use futures::future::FutureResult;

use hyper::server::{Http, Service, Request, Response};
use hyper::{Get, StatusCode};
use hyper::header::ContentLength;
use hyper::header::ContentType;
use hyper::mime;
use hyper::mime::{Mime, TopLevel, SubLevel};


fn to_full_filename(filename: &str) -> PathBuf {
    let mut path = env::current_dir().unwrap();
    path.push(filename);
    path
}

fn parse_file_name(uri: &str) -> Option<PathBuf> {
    let extracts_filename = Regex::new(r"([\w\-\.]+[^#?\s]+).*?").unwrap();
    extracts_filename.captures(uri).map(|captures| {
        let full_filename = to_full_filename(&captures[1]);
        println!("Attempting {:?}", full_filename);
        full_filename
    })
}

fn filename_content_type(filename: &PathBuf) -> hyper::header::ContentType {
    match filename.extension() {
        Some(extension) => {
            match extension.to_str().unwrap() {
                "html" => ContentType::html(),
                "css"  => ContentType(Mime(TopLevel::Text, SubLevel::Css, vec![])),
                "js"   => ContentType(Mime(TopLevel::Application, SubLevel::Javascript, vec![])),
                "jpg" | "jpeg" => ContentType::jpeg(),
                "png" => ContentType::png(),
                 _ => ContentType::plaintext()
            }
        },
        None => ContentType::plaintext()

    }
}


fn read_file(filename: &PathBuf) -> Result<Vec<u8>, io::Error> {
    let mut f = File::open(filename)?;
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer)?;
    Ok(buffer)
}


struct FileServer;

static INDEX: &'static [u8] = b"For security reasons directories are not listed. Add a filename to the URL.";
static UNIMPLEMENTED: &'static [u8] = b"Operation not implemented";

fn serve_file(url: &str) -> Response {
    let filename = parse_file_name(url).unwrap();
    match read_file(&filename) {
        Ok(data) => {
            let length = data.len();
            Response::new()
                .with_body(data)
                .with_header(ContentLength(length as u64))
                .with_header(filename_content_type(&filename))
        },
        Err(error) => {
            println!("{:?}", error);
            Response::new().with_status(StatusCode::NotFound)
        }

    }
}


impl Service for FileServer {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = FutureResult<Response, hyper::Error>;

    fn call(&self, req: Request) -> Self::Future {
        futures::future::ok(match (req.method(), req.path()) {
            (&Get, "/") => {
                Response::new()
                    .with_header(ContentLength(INDEX.len() as u64))
                    .with_body(INDEX)
            },
            (&Get, url) => {
                serve_file(url)
            },
            (operation, url) => {
                println!("Operation {:?} on {:?} not implemented", &operation, &url);
                Response::new()
                    .with_header(ContentLength(UNIMPLEMENTED.len() as u64))
                    .with_body(UNIMPLEMENTED)
            },
        })
    }
}



fn main() {
    let addr = "127.0.0.1:8888".parse().unwrap();

    let server = Http::new().bind(&addr, || Ok(FileServer)).unwrap();
    println!("Listening on http://{} with 1 thread.", server.local_addr().unwrap());
    server.run().unwrap();
}
