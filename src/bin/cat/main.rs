use std::env;
use std::fs::{self, File};
use std::io::Read;
use std::path::PathBuf;

mod config;

macro_rules! file_err {
    ( $p:expr, $e:expr ) => {
        Err(format!("{}: {}", $p.to_string_lossy(), $e))
    };
}

fn main() {
    if let Ok(config) = config::Config::new(env::args()) {
        for file in config.files {
            match parse_file(&file) {
                Ok(contents) => print!("{}", contents.as_str()),
                Err(e) => eprintln!("cat: {}", e.as_str()),
            }
        }
    }
}

pub fn parse_file(path: &PathBuf) -> Result<String, String> {
    let attrs = match fs::metadata(path) {
        Ok(attrs) => attrs,
        Err(e) => return file_err!(path, e),
    };

    if attrs.is_dir() {
        return file_err!(path, "Is a directory");
    }

    let file = match File::open(path) {
        Ok(f) => f,
        Err(e) => return file_err!(path, e),
    };

    let bytes: Vec<u8> = file.bytes().filter_map(|bytes| bytes.ok()).collect();

    return Ok(String::from_utf8_lossy(&bytes).to_string());
}

#[cfg(test)]
mod cat_tests {
    use super::*;
    use coreutils::test_utils;
    #[test]
    fn regular_ascii_file() {
        let path = test_utils::get_path("ascii.txt");
        let contents = match parse_file(&path) {
            Ok(contents) => contents,
            Err(e) => panic!(e),
        };
        assert_eq!(contents, "The quick brown fox jumps over the lazy dog")
    }

    #[test]
    fn binary_file() {
        let path = test_utils::get_path("hello.bin");
        let contents = match parse_file(&path) {
            Ok(contents) => contents,
            Err(e) => panic!(e),
        };
        assert!(contents.contains("Hello"))
    }
}
