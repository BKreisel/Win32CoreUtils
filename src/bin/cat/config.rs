use std::path::PathBuf;

#[derive(Debug)]
pub struct Config {
    pub files: Vec<PathBuf>,
    pub number: bool,
    pub number_nonblank: bool,
    pub show_ends: bool,
    pub squeeze_blank: bool,
    pub show_tabs: bool,
    pub show_nonprinting: bool
}

const HELP_TEXT: &str = "Usage: cat [OPTION]... [FILE]...
Concatenate FILE(s) to standard output.
            
    -A, --show-all           equivalent to -vET
    -b, --number-nonblank    number nonempty output lines, overrides -n
    -e                       equivalent to -vE
    -E, --show-ends          display $ at end of each line
    -n, --number             number all output lines
    -s, --squeeze-blank      suppress repeated empty output lines
    -t                       equivalent to -vT
    -T, --show-tabs          display TAB characters as ^I
    -v, --show-nonprinting   use ^ and M- notation, except for LFD and TAB
        --help     display this help and exit
        --version  output version information and exit
";

impl Config {
    pub fn new(args: Vec<String>) -> Result<Config, String> {
        if args.len() == 1 || args.contains(&"--help".to_string()) {
            return Err(HELP_TEXT.to_string());
        }

        if args.contains(&"--version".to_string()) {
            return Err(format!("cat (Win32CoreUtils) v{}", env!("CARGO_PKG_VERSION")));
        }

        let mut number = false;
        let mut number_nonblank = false;
        let mut show_ends = false;
        let mut squeeze_blank = false;
        let mut show_tabs = false;
        let mut show_nonprinting = false;

        let options: Vec<String> = args
            .iter()
            .skip(1)
            .filter(|arg| arg.starts_with("-"))
            .map(|s| String::from(s))
            .collect();
        
        for option in options {
            match option.as_str() {
                "--show-all" =>  {
                    show_nonprinting = true;
                    show_ends = true;
                    show_tabs = true;
                },
                "-A" =>  {
                    show_nonprinting = true;
                    show_ends = true;
                    show_tabs = true;
                },
                "--number-nonblank" => {
                    number_nonblank = true;
                    number = false;
                }
                "-b" => {
                    number_nonblank = true;
                    number = false;
                }
                "-e" => {
                    show_nonprinting = true;
                    show_ends = true;
                }
                "--show-ends" => show_ends = true,
                "-E" => show_ends = true,
                "--number" => if !number_nonblank {number = true},
                "-n" => if !number_nonblank {number = true},
                "--squeeze-blank" => squeeze_blank = true,
                "-s" => squeeze_blank = true,
                "--show-tabs" => show_tabs = true,
                "-T" => show_tabs = true,
                "-t" => {
                    show_nonprinting = true;
                    show_tabs = true;
                }
                "--show-nonprinting" => show_nonprinting = true,
                "-v" => show_nonprinting = true,
                _ => {}
            }
        }

        let files: Vec<PathBuf> = args
            .iter()
            .skip(1)
            .filter(|arg| !arg.starts_with('-'))
            .map(|arg| PathBuf::from(arg))
            .collect();

        Ok(Config {
            files, number, number_nonblank, show_ends,
            squeeze_blank, show_tabs, show_nonprinting
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::config;

    macro_rules! string_vec {
        ( $( $x:expr ),* ) => {
            {
                let mut temp_vec = Vec::new();
                $(temp_vec.push($x);)*
                temp_vec.iter().map(|s| s.to_string()).collect()
            }
        };
    }

    #[test]
    fn no_args() {
        let args = string_vec!["cat.exe"];
        assert!(config::Config::new(args)
                .unwrap_err()
                .contains("Usage:"));
    }

    #[test]
    fn one_file() {
        let args = string_vec!["cat.exe", "myfile.txt"];
        let config = config::Config::new(args).unwrap();
        assert!(config.files.iter().any(|x| x.to_string_lossy() == "myfile.txt"));
        assert!(!config.number);
        assert!(!config.number_nonblank);
        assert!(!config.show_ends);
        assert!(!config.squeeze_blank);
        assert!(!config.show_tabs);
        assert!(!config.show_nonprinting);
    }

    #[test]
    fn multiple_files() {
        let args = string_vec!["cat.exe", "myfile.txt", "other.txt"];
        let files = config::Config::new(args).unwrap().files;

        assert_eq!(files.len(), 2);
        assert!(files.iter().any(|x| x.to_string_lossy() == "myfile.txt"));
        assert!(files.iter().any(|x| x.to_string_lossy() == "other.txt"));

    }

    #[test]
    fn show_all_long() {
        let args = string_vec!["cat.exe", "myfile.txt", "--show-all"];
        let config = config::Config::new(args).unwrap();
        assert!(config.show_nonprinting);
        assert!(config.show_ends);
        assert!(config.show_tabs);
    }

    #[test]
    fn show_all_short() {
        let args = string_vec!["cat.exe", "-A", "myfile.txt"];
        let config = config::Config::new(args).unwrap();
        assert!(config.show_nonprinting);
        assert!(config.show_ends);
        assert!(config.show_tabs);
    }

    #[test]
    fn number_nonblank_long() {
        let args = string_vec!["cat.exe", "myfile.txt", "--number-nonblank"];
        let config = config::Config::new(args).unwrap();
        assert!(config.number_nonblank);
    }
    
    #[test]
    fn number_nonblank_short() {
        let args = string_vec!["cat.exe", "myfile.txt", "-b"];
        let config = config::Config::new(args).unwrap();
        assert!(config.number_nonblank);
    }

    #[test]
    fn number_nonblank_override() {
        let args = string_vec!["cat.exe", "myfile.txt", "-b", "-n"];
        let config = config::Config::new(args).unwrap();
        assert!(config.number_nonblank);
        assert!(!config.number);
    }

    #[test]
    fn number_nonblank_override_long() {
        let args = string_vec!["cat.exe", "-n", "myfile.txt", "--number-nonblank"];
        let config = config::Config::new(args).unwrap();
        assert!(config.number_nonblank);
        assert!(!config.number);
    }


    #[test]
    fn  e_flag() {
        let args = string_vec!["cat.exe", "myfile.txt", "-e"];
        let config = config::Config::new(args).unwrap();
        assert!(config.show_nonprinting);
        assert!(config.show_ends);
    }

    #[test]
    fn show_ends_long() {
        let args = string_vec!["cat.exe", "myfile.txt", "--show-ends"];
        let config = config::Config::new(args).unwrap();
        assert!(config.show_ends);
    }

    #[test]
    fn show_ends_short() {
        let args = string_vec!["cat.exe", "myfile.txt", "-E"];
        let config = config::Config::new(args).unwrap();
        assert!(config.show_ends);
    }

    #[test]
    fn number_long() {
        let args = string_vec!["cat.exe", "myfile.txt", "--number"];
        let config = config::Config::new(args).unwrap();
        assert!(config.number);
    }

    #[test]
    fn number_short() {
        let args = string_vec!["cat.exe", "myfile.txt", "-n"];
        let config = config::Config::new(args).unwrap();
        assert!(config.number);
    }

    #[test]
    fn squeeze_blank_long() {
        let args = string_vec!["cat.exe", "myfile.txt", "--squeeze-blank"];
        let config = config::Config::new(args).unwrap();
        assert!(config.squeeze_blank);
    }

    #[test]
    fn squeeze_blank_short() {
        let args = string_vec!["cat.exe", "myfile.txt", "-s"];
        let config = config::Config::new(args).unwrap();
        assert!(config.squeeze_blank);
    }

    #[test]
    fn t_flag() {
        let args = string_vec!["cat.exe", "myfile.txt", "-t"];
        let config = config::Config::new(args).unwrap();
        assert!(config.show_nonprinting);
        assert!(config.show_tabs);
    }

    #[test]
    fn show_tabs_long() {
        let args = string_vec!["cat.exe", "myfile.txt", "--show-tabs"];
        let config = config::Config::new(args).unwrap();
        assert!(config.show_tabs);
    }

    #[test]
    fn show_tabs_short() {
        let args = string_vec!["cat.exe", "myfile.txt", "-T"];
        let config = config::Config::new(args).unwrap();
        assert!(config.show_tabs);
    }

    #[test]
    fn show_nonprinting_long() {
        let args = string_vec!["cat.exe", "myfile.txt", "--show-nonprinting"];
        let config = config::Config::new(args).unwrap();
        assert!(config.show_nonprinting);
    }

    #[test]
    fn show_nonprinting_short() {
        let args = string_vec!["cat.exe", "myfile.txt", "-v"];
        let config = config::Config::new(args).unwrap();
        assert!(config.show_nonprinting);
    }

    #[test]
    fn help() {
        let args = string_vec!["cat.exe", "myfile.txt", "--help"];
        assert!(config::Config::new(args)
            .unwrap_err()
            .contains("Usage:"));
    }

    #[test]
    fn version() {
        let args = string_vec!["cat.exe", "myfile.txt", "--version"];
        assert!(config::Config::new(args)
                .unwrap_err()
                .contains(env!("CARGO_PKG_VERSION")));
    }
}