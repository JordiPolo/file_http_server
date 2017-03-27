extern crate hyper;
extern crate regex;

use hyper::server::{Server, Request, Response};
use hyper::uri::RequestUri;
use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::env;

use regex::Regex;

fn to_full_filename(filename: &str) -> String {
    let mut path = env::current_dir().unwrap();
    path.push(filename);
    path.into_os_string().into_string().unwrap()
}

fn parse_file_name(uri: &RequestUri) -> Option<String> {
    let extracts_filename = Regex::new(r"([\w\-\.]+[^#?\s]+).*?").unwrap();
    extracts_filename.captures(&uri.to_string()).map(|captures| {
        let full_filename = to_full_filename(&captures[1]);
        println!("Attempting {:?}", full_filename);
        full_filename
    })
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
            match parse_file_name(&req.uri) {
                Some(filename) => {
                    match read_file(&filename) {
                        Ok(file) => res.send(file.as_bytes()).unwrap(),
                        Err(error) => res.send(error.to_string().as_bytes()).unwrap(),
                    }
                },
                None => res.send("For security reasons directories are not listed. Add a filename to the URL.".as_bytes()).unwrap(),
            }

        }
        _ => {
            println!("Not supported");
            // TODO: how to modify status?
            //*res.status_mut() = StatusCode::MethodNotAllowed
            res.send("Operation not supported".as_bytes()).unwrap();
        }
    }
}

fn main() {
    Server::http("127.0.0.1:8888").unwrap().handle(handle_request).unwrap();
}
