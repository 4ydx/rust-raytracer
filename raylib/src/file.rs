use crate::vec;

use std::fs;
use std::io::Write;
use std::path::Path;

#[derive(Debug)]
pub struct File {
    pub handle: std::fs::File,
}

impl File {
    pub fn new(path: &str, height: i64, width: i64) -> File {
        let path = Path::new(path);
        let mut handle = match fs::File::create(&path) {
            Err(err) => panic!("error {}", err),
            Ok(file) => file,
        };
        match handle.write_all(format!("P3\n{} {}\n255\n", width, height).as_bytes()) {
            Err(err) => panic!("error {}", err),
            Ok(_) => {}
        }
        File { handle: handle }
    }

    pub fn write(&self, v: vec::Vec3) {
        let mut file = &self.handle;

        let line = format!("{}\n", v);
        match file.write_all(line.as_bytes()) {
            Err(err) => panic!("error {}", err),
            Ok(_) => {}
        }
    }
}
