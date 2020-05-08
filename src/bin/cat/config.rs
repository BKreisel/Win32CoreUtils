use std::path::PathBuf;

use coreutils::config::Config;

#[derive(Debug)]
pub struct CatConfig {
    pub files: Vec<PathBuf>,
    pub number: bool,
    pub number_nonblank: bool,
    pub show_ends: bool,
    pub squeeze_blank: bool,
    pub show_tabs: bool,
    pub show_nonprinting: bool,
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

impl Config for CatConfig {
    fn bin_name(&self) -> &'static str {
        "cat"
    }
    fn usage(&self) -> &'static str {
        &HELP_TEXT
    }
}

impl CatConfig {
    pub fn new() -> CatConfig {
        CatConfig {
            files: Vec::new(),
            number: false,
            number_nonblank: false,
            show_ends: false,
            squeeze_blank: false,
            show_tabs: false,
            show_nonprinting: false,
        }
    }

    pub fn parse(&mut self, args: Vec<String>) -> Result<(), String> {
        let base_config = match Config::parse(self, &args) {
            Ok(config) => config,
            Err(e) => return Err(e),
        };

        let mut files: Vec<PathBuf> = base_config.parameters.iter().map(PathBuf::from).collect();
        self.files.append(&mut files);

        if self.files.is_empty() {
            return Err(self.usage().to_string());
        }

        for option in base_config.options {
            match option.as_str() {
                "--show-all" | "-A" => {
                    self.show_nonprinting = true;
                    self.show_ends = true;
                    self.show_tabs = true;
                }
                "--number-nonblank" | "-b" => {
                    self.number_nonblank = true;
                    self.number = false;
                }
                "-e" => {
                    self.show_nonprinting = true;
                    self.show_ends = true;
                }
                "--show-ends" | "-E" => self.show_ends = true,
                "--number" | "-n" => {
                    if !self.number_nonblank {
                        self.number = true
                    }
                }
                "--squeeze-blank" | "-s" => self.squeeze_blank = true,
                "--show-tabs" | "-T" => self.show_tabs = true,
                "-t" => {
                    self.show_nonprinting = true;
                    self.show_tabs = true;
                }
                "--show-nonprinting" | "-v" => self.show_nonprinting = true,
                _ => {
                    return Err(format!(
                        "invalid option: {}\n Try cat --help for more information",
                        option
                    ))
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::config::CatConfig;
    use coreutils::string_vec;

    #[test]
    fn no_args() {
        let args = string_vec!["cat.exe"];
        assert!(CatConfig::new().parse(args).unwrap_err().contains("Usage:"));
    }

    #[test]
    fn one_file() {
        let mut config = CatConfig::new();
        config.parse(string_vec!["cat.exe", "myfile.txt"]).unwrap();

        assert!(config
            .files
            .iter()
            .any(|x| x.to_string_lossy() == "myfile.txt"));
        assert!(!config.number);
        assert!(!config.number_nonblank);
        assert!(!config.show_ends);
        assert!(!config.squeeze_blank);
        assert!(!config.show_tabs);
        assert!(!config.show_nonprinting);
    }

    #[test]
    fn multiple_files() {
        let mut config = CatConfig::new();
        config
            .parse(string_vec!["cat.exe", "myfile.txt", "other.txt"])
            .unwrap();
        assert_eq!(config.files.len(), 2);
        assert!(config
            .files
            .iter()
            .any(|x| x.to_string_lossy() == "myfile.txt"));
        assert!(config
            .files
            .iter()
            .any(|x| x.to_string_lossy() == "other.txt"));
    }

    #[test]
    fn show_all() {
        let mut config = CatConfig::new();
        config
            .parse(string_vec!["cat.exe", "myfile.txt", "--show-all"])
            .unwrap();
        assert!(config.show_nonprinting);
        assert!(config.show_ends);
        assert!(config.show_tabs);

        config
            .parse(string_vec!["cat.exe", "-A", "myfile.txt"])
            .unwrap();
        assert!(config.show_nonprinting);
        assert!(config.show_ends);
        assert!(config.show_tabs);
    }

    #[test]
    fn number_nonblank() {
        let mut config = CatConfig::new();
        config
            .parse(string_vec!["cat.exe", "myfile.txt", "--number-nonblank"])
            .unwrap();
        assert!(config.number_nonblank);

        config
            .parse(string_vec!["cat.exe", "myfile.txt", "-b"])
            .unwrap();
        assert!(config.number_nonblank);
    }

    #[test]
    fn number_nonblank_override() {
        let mut config = CatConfig::new();
        config
            .parse(string_vec!["cat.exe", "myfile.txt", "-b", "-n"])
            .unwrap();
        assert!(config.number_nonblank);
        assert!(!config.number);

        config
            .parse(string_vec![
                "cat.exe",
                "-n",
                "myfile.txt",
                "--number-nonblank"
            ])
            .unwrap();
        assert!(config.number_nonblank);
        assert!(!config.number);
    }

    #[test]
    fn e_flag() {
        let mut config = CatConfig::new();
        config
            .parse(string_vec!["cat.exe", "myfile.txt", "-e"])
            .unwrap();
        assert!(config.show_nonprinting);
        assert!(config.show_ends);
    }

    #[test]
    fn show_ends() {
        let mut config = CatConfig::new();
        config
            .parse(string_vec!["cat.exe", "myfile.txt", "--show-ends"])
            .unwrap();
        assert!(config.show_ends);

        config
            .parse(string_vec!["cat.exe", "myfile.txt", "-E"])
            .unwrap();
        assert!(config.show_ends);
    }

    #[test]
    fn number() {
        let mut config = CatConfig::new();
        config
            .parse(string_vec!["cat.exe", "myfile.txt", "--number"])
            .unwrap();
        assert!(config.number);

        config
            .parse(string_vec!["cat.exe", "myfile.txt", "-n"])
            .unwrap();
        assert!(config.number);
    }

    #[test]
    fn squeeze_blank() {
        let mut config = CatConfig::new();
        config
            .parse(string_vec!["cat.exe", "myfile.txt", "--squeeze-blank"])
            .unwrap();
        assert!(config.squeeze_blank);

        config
            .parse(string_vec!["cat.exe", "myfile.txt", "-s"])
            .unwrap();
        assert!(config.squeeze_blank);
    }

    #[test]
    fn t_flag() {
        let mut config = CatConfig::new();
        config
            .parse(string_vec!["cat.exe", "myfile.txt", "-t"])
            .unwrap();
        assert!(config.show_nonprinting);
        assert!(config.show_tabs);
    }

    #[test]
    fn show_tabs() {
        let mut config = CatConfig::new();
        config
            .parse(string_vec!["cat.exe", "myfile.txt", "--show-tabs"])
            .unwrap();
        assert!(config.show_tabs);

        config
            .parse(string_vec!["cat.exe", "myfile.txt", "-T"])
            .unwrap();
        assert!(config.show_tabs);
    }

    #[test]
    fn show_nonprinting() {
        let mut config = CatConfig::new();
        config
            .parse(string_vec!["cat.exe", "myfile.txt", "--show-nonprinting"])
            .unwrap();
        assert!(config.show_nonprinting);

        config
            .parse(string_vec!["cat.exe", "myfile.txt", "-v"])
            .unwrap();
        assert!(config.show_nonprinting);
    }

    #[test]
    fn help() {
        let mut config = CatConfig::new();
        let parse_result = config.parse(string_vec!["cat.exe", "myfile.txt", "--help"]);
        assert!(parse_result.unwrap_err().contains("Usage:"));
    }

    #[test]
    fn version() {
        let mut config = CatConfig::new();
        let parse_result = config.parse(string_vec!["cat.exe", "myfile.txt", "--version"]);
        assert!(parse_result
            .unwrap_err()
            .contains(env!("CARGO_PKG_VERSION")));
    }
}
