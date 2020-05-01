use std::env;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Config {
    pub files: Vec<PathBuf>,
}

impl Config {
    pub fn new(args: env::Args) -> Result<Config, ()> {
        if args.len() == 1 {
            Config::help();
            return Err(());
        }

        let files: Vec<PathBuf> = args
            .skip(1)
            .filter(|arg| !arg.starts_with('-'))
            .map(|arg| PathBuf::from(arg))
            .collect();

        Ok(Config { files })
    }

    pub fn help() {
        let usage = "Usage: cat [OPTION] ... [FILE] ...
Concatenate FILE(s) to standard output.
";
        println!("{}", usage);
    }
}
