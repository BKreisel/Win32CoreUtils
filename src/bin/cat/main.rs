use std::env;
use std::fs::{self, File};
use std::io::Read;
use std::path::PathBuf;
use std::process;

mod config;
mod format;

macro_rules! file_err {
    ( $p:expr, $e:expr ) => {
        Err(format!("{}: {}", $p.to_string_lossy(), $e))
    };
}

fn main() {
    let config = match config::Config::new(env::args().collect()) {
        Ok(config) => config,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };

    for file in config.files {
        let mut contents = match parse_file(&file) {
            Ok(contents) => contents,
            Err(e) => {
                eprintln!("cat: {}", e.as_str());
                process::exit(1);
            }
        };

        if config.number_nonblank {
            contents = format::number_nonblank_lines(contents);
        }
        if config.number {
            contents = format::number_lines(contents);
        }
        if config.show_ends {
            contents = format::show_ends(contents);
        }
        if config.squeeze_blank {
            contents = format::squeeze_blank(contents);
        }
        if config.show_tabs {
            contents = format::show_tabs(contents);
        }
        if config.show_nonprinting {
            contents = format::show_nonprinting(contents);
        }

        print!("{}", contents);
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

    Ok(String::from_utf8_lossy(&bytes).to_string())
}

#[cfg(test)]
mod tests {
    use coreutils::test_utils;

    #[test]
    fn regular_ascii_file() {
        let path = test_utils::get_path("ascii.txt");
        let contents = match crate::parse_file(&path) {
            Ok(contents) => contents,
            Err(e) => panic!(e),
        };
        assert_eq!(contents, test_utils::ASCII)
    }

    #[test]
    fn binary_file() {
        let path = test_utils::get_path("hello.bin");
        let contents = match crate::parse_file(&path) {
            Ok(contents) => contents,
            Err(e) => panic!(e),
        };
        assert!(contents.contains("Hello"))
    }
}
