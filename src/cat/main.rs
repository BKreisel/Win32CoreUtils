use std::env;
use std::fs;
use std::path::PathBuf;

mod config;

fn main() {
    if let Ok(config) = config::Config::new(env::args()) {
        for file in config.files {
            if let Err(e) = print_file(&file) {
                eprintln!("cat: {}", e.as_str());
            }
        }
    }    
}

macro_rules! file_err {
    ( $p:expr, $e:expr ) => {
        Err(format!("{}: {}", $p.to_string_lossy(), $e))
    };
}

fn print_file(path: &PathBuf) -> Result<(), String> {
    let attrs = match fs::metadata(path) {
        Ok(attrs) => attrs,
        Err(e) => return file_err!(path, e)
    };

    if attrs.is_dir() {
        return file_err!(path, "Is a directory");
    }

    let contents = match fs::read_to_string(path) {
        Ok(contents) => contents,
        Err(e) => return file_err!(path, e)
    };

    print!("{}", contents.as_str());

    Ok(())
}