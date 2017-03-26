extern crate hyper;

use hyper::server::{Server, Request, Response};
use hyper::status::StatusCode;
use hyper::uri::RequestUri;
use std::io;
use std::io::prelude::*;
use std::fs::File;

//const ALLOWED_FILE_TYPES: [&str] = ["html", "js", "css"];

fn parse_file_name(uri: &RequestUri) -> Option<String> {
    let filename = uri.to_string();
    let clean_filename = filename.split("?").nth(0).unwrap();
    let mut fullname = ".".to_string();
    fullname.push_str(clean_filename);
    println!("Opening {}", fullname);
    Some(fullname)
}

fn read_file(filename: &str) -> Result<String, io::Error> {
    let mut f = File::open(filename)?;
    let mut buffer = String::new();
    f.read_to_string(&mut buffer)?;
    Ok(buffer)
}


fn handle_request(req: Request, res: Response) {
    match req.method {
        hyper::Get => {
            match parse_file_name(&req.uri).map(|file_name| read_file(&file_name)).unwrap() {
                Ok(file) => res.send(file.as_bytes()).unwrap(),
                Err(error) => res.send(error.to_string().as_bytes()).unwrap(),
            }
        },
        _ => {
            println!("Not supported"); //*res.status_mut() = StatusCode::MethodNotAllowed
            res.send("Operation not supported".as_bytes()).unwrap();
        }
    }
}

fn main() {
    Server::http("0.0.0.0:8888").unwrap().handle(handle_request).unwrap();
}
