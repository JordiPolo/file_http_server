extern crate hyper;
extern crate futures;


// extern crate futures_cpupool;
use futures::future::FutureResult;
// use futures_cpupool::CpuPool;

use hyper::server::{Http, Service, Request, Response};
use hyper::{Get, StatusCode};
use hyper::header::{ContentLength, ContentType};
use std::time::Instant;


//pub mod filename;


use hyper::mime::{Mime, TopLevel, SubLevel};

use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::env;
use std::path::PathBuf;


struct Filename {
    path: PathBuf,
}

impl Filename {
    fn from_path(path: &str) -> Self {
        let mut fullpath = env::current_dir().unwrap();
        fullpath.push(&path[1..]);
                println!("Full {:?}", fullpath);

        Self {path: fullpath}
    }

    fn read_data(&self) -> Result<Vec<u8>, io::Error> {
        println!("Attemping {:?}", &self.path);
        let mut f = File::open(&self.path)?;
        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer)?;
        Ok(buffer)
    }

    fn content_type(&self) -> hyper::header::ContentType {
        let extension = self.path.extension().map(|extension| extension.to_str().unwrap());

        match extension {
            Some(extension) => {
                match extension {
                    "html" => ContentType::html(),
                    "css" => ContentType(Mime(TopLevel::Text, SubLevel::Css, vec![])),
                    "js" => ContentType(Mime(TopLevel::Application, SubLevel::Javascript, vec![])),
                    "jpg" | "jpeg" => ContentType::jpeg(),
                    "png" => ContentType::png(),
                    _ => ContentType::plaintext(),
                }
            }
            None => ContentType::plaintext(),

        }
    }

}







static INDEX: &'static [u8] = b"For security reasons directories are not listed. Add a filename to the URL.";
static UNIMPLEMENTED: &'static [u8] = b"Operation not implemented";




fn print_elapsed<U, F>(f: F, text: &str) -> U where F: Fn() -> U {
    let start = Instant::now();
    let result = f();
    let elapsed = start.elapsed();
    println!("{}: {} ms", text, (elapsed.as_secs() * 1_000) + (elapsed.subsec_nanos() / 1_000_000) as u64);
    result
}



fn serve_file(path: &str) -> Response {
    let file = print_elapsed(|| Filename::from_path(path), "Elapsed read");

    match file.read_data() {
        Ok(data) => {
            let length = data.len();
            Response::new()
                .with_body(data)
                .with_header(ContentLength(length as u64))
                .with_header(file.content_type())
        }
        Err(error) => {
            println!("{:?}", error);
            Response::new().with_status(StatusCode::NotFound)
        }

    }
}

struct FileServer; //{
 //   thread_pool: CpuPool,
//}

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
            }
            (&Get, path) => {
                print_elapsed(|| serve_file(path), "Served in")
            },
            (operation, path) => {
                println!("Operation {:?} on {:?} not implemented", &operation, &path);
                Response::new()
                    .with_header(ContentLength(UNIMPLEMENTED.len() as u64))
                    .with_body(UNIMPLEMENTED)
            }
        })
    }
}



fn main() {
    let addr = "127.0.0.1:8888".parse().unwrap();
    //let thread_pool = CpuPool::new(10);

    let server = Http::new()
        .bind(&addr,
              || Ok(FileServer)) //{ thread_pool: thread_pool.clone() }))
        .expect("Port 8888 already in use. Kill other running instances of this program.");

    println!("Listening on http://{} with 1 thread.", server.local_addr().unwrap());
    server.run().unwrap();
}
