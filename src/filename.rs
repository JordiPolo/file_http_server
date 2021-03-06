use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::env;
use std::path::PathBuf;
use mime_guess;

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

    pub fn content_type(&self) -> mime_guess::Mime {
         mime_guess::guess_mime_type(&self.path)
    }

}
