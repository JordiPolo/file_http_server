use hyper::mime::{Mime, TopLevel, SubLevel};
use hyper::header::ContentType;
use hyper;

use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::env;
use std::path::PathBuf;

pub struct Filename {
    path: PathBuf,
}

impl Filename {
   pub fn from_path(path: &str) -> Self {
        let mut fullpath = env::current_dir().unwrap();
        fullpath.push(&path[1..]);
        Self { path: fullpath }
    }

    pub fn read_data(&self) -> Result<Vec<u8>, io::Error> {
        println!("Attemping {:?}", &self.path);
        let mut f = File::open(&self.path)?;
        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer)?;
        Ok(buffer)
    }

    pub fn content_type(&self) -> hyper::header::ContentType {
        match self.path.extension().map(|extension| extension.to_str().unwrap()) {
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
