use std::path::PathBuf;

use coreutils::config::Config;

#[derive(Debug)]
pub struct LsConfig {
    pub paths: Vec<PathBuf>,
    pub all: bool,
    pub almost_all: bool,
    pub creation_time: bool,
    pub directory_view: bool,
    pub full_time: bool,
    pub group_view: bool,
    pub reverse: bool,
    pub recurse: bool,
    pub size_sort: bool,
    pub modify_sort: bool,
    pub access_sort: bool,
    pub one_per_line: bool,
}

const HELP_TEXT: &str = "Usage: ls [OPTION]... [FILE]...
List information about the FILEs (the current directory by default).

  -a, --all                  do not ignore entries starting with .
  -A, --almost-all           do not list implied . and ..
  -c                         with -t: sort by, and show, creation time.
                               otherwise: show creation time and sort by name;
  -d, --directory            list directories themselves, not their contents
      --full-time            Full ISO Time
  -G, --no-group             don't print group names
  -r, --reverse              reverse order while sorting
  -R, --recursive            list subdirectories recursively
  -S                         sort by file size, largest first
  -t                         sort by modification time, newest first
  -u                         with -t: sort by, and show, access time;
                               otherwise: show access time and sort by name;
  -1                         list one file per line.
      --help     display this help and exit
      --version  output version information and exit

Exit status:
 0  if OK,
 1  if minor problems (e.g., cannot access subdirectory),
 2  if serious trouble (e.g., cannot access command-line argument).
";

impl Config for LsConfig {
    fn bin_name(&self) -> &'static str {
        "ls"
    }
    fn usage(&self) -> &'static str {
        &HELP_TEXT
    }
}

impl LsConfig {
    pub fn new() -> LsConfig {
        LsConfig {
            paths: Vec::new(),
            all: false,
            almost_all: false,
            creation_time: false,
            directory_view: false,
            full_time: false,
            group_view: true,
            reverse: false,
            recurse: false,
            size_sort: false,
            modify_sort: false,
            access_sort: false,
            one_per_line: false,
        }
    }

    pub fn parse(&mut self, args: Vec<String>) -> Result<(), String> {
        let base_config = match Config::parse(self, &args) {
            Ok(config) => config,
            Err(e) => return Err(e),
        };

        let mut paths: Vec<PathBuf> = base_config.parameters.iter().map(PathBuf::from).collect();
        self.paths.append(&mut paths);

        for option in base_config.options {
            match option.as_str() {
                "--all" | "-a" => self.all = true,
                "--almost-all" | "-A" => self.almost_all = true,
                "-c" => self.creation_time = true,
                "--directory" | "-d" => self.directory_view = true,
                "--full-time" => self.full_time = true,
                "--no-group" | "-G" => self.group_view = false,
                "--reverse" | "-r" => self.reverse = true,
                "--recursive" | "-R" => self.recurse = true,
                "-S" => self.size_sort = true,
                "-t" => self.modify_sort = true,
                "-u" => self.access_sort = true,
                "-1" => self.one_per_line = true,
                _ => {
                    return Err(format!(
                        "invalid option: {}\n Try ls --help for more information",
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
    use crate::config::LsConfig;
    use coreutils::string_vec;

    #[test]
    fn help() {
        let mut config = LsConfig::new();
        let parse_result = config.parse(string_vec!["ls.exe", "myfile.txt", "--help"]);
        assert!(parse_result.unwrap_err().contains("Usage:"));
    }

    #[test]
    fn version() {
        let mut config = LsConfig::new();
        let parse_result = config.parse(string_vec!["ls.exe", "myfile.txt", "--version"]);
        assert!(parse_result
            .unwrap_err()
            .contains(env!("CARGO_PKG_VERSION")));
    }

    #[test]
    fn normal_paths() {
        let mut config = LsConfig::new();
        config
            .parse(string_vec!["ls.exe", "myfile.txt", "mydir/"])
            .unwrap();
        assert!(config.paths.len() == 2)
    }

    #[test]
    fn option_all() {
        let mut config = LsConfig::new();
        config
            .parse(string_vec!["ls.exe", "myfile.txt", "--all"])
            .unwrap();
        assert!(config.all);
        config
            .parse(string_vec!["ls.exe", "myfile.txt", "-a"])
            .unwrap();
        assert!(config.all);
    }

    #[test]
    fn option_almost_all() {
        let mut config = LsConfig::new();
        config
            .parse(string_vec!["ls.exe", "myfile.txt", "--almost-all"])
            .unwrap();
        assert!(config.almost_all);
        config
            .parse(string_vec!["ls.exe", "myfile.txt", "-A"])
            .unwrap();
        assert!(config.almost_all);
    }

    #[test]
    fn option_creation_time() {
        let mut config = LsConfig::new();
        config
            .parse(string_vec!["ls.exe", "myfile.txt", "-c"])
            .unwrap();
        assert!(config.creation_time);
    }

    #[test]
    fn option_directory_view() {
        let mut config = LsConfig::new();
        config
            .parse(string_vec!["ls.exe", "myfile.txt", "--directory"])
            .unwrap();
        assert!(config.directory_view);
        config
            .parse(string_vec!["ls.exe", "myfile.txt", "-d"])
            .unwrap();
        assert!(config.directory_view);
    }

    #[test]
    fn option_group_view() {
        let mut config = LsConfig::new();
        config.parse(string_vec!["ls.exe", "myfile.txt"]).unwrap();
        assert!(config.group_view);
        config
            .parse(string_vec!["ls.exe", "myfile.txt", "--no-group"])
            .unwrap();
        assert!(!config.group_view);
        config
            .parse(string_vec!["ls.exe", "myfile.txt", "-G"])
            .unwrap();
        assert!(!config.group_view);
    }

    #[test]
    fn option_reverse() {
        let mut config = LsConfig::new();
        config
            .parse(string_vec!["ls.exe", "myfile.txt", "--reverse"])
            .unwrap();
        assert!(config.reverse);
        config
            .parse(string_vec!["ls.exe", "myfile.txt", "-r"])
            .unwrap();
        assert!(config.reverse);
    }

    #[test]
    fn option_recursive() {
        let mut config = LsConfig::new();
        config
            .parse(string_vec!["ls.exe", "myfile.txt", "--recursive"])
            .unwrap();
        assert!(config.recurse);
        config
            .parse(string_vec!["ls.exe", "myfile.txt", "-R"])
            .unwrap();
        assert!(config.recurse);
    }

    #[test]
    fn option_size_sort() {
        let mut config = LsConfig::new();
        config
            .parse(string_vec!["ls.exe", "myfile.txt", "-S"])
            .unwrap();
        assert!(config.size_sort);
    }

    #[test]
    fn option_modify_sort() {
        let mut config = LsConfig::new();
        config
            .parse(string_vec!["ls.exe", "myfile.txt", "-t"])
            .unwrap();
        assert!(config.modify_sort);
    }

    #[test]
    fn option_access_sort() {
        let mut config = LsConfig::new();
        config
            .parse(string_vec!["ls.exe", "myfile.txt", "-u"])
            .unwrap();
        assert!(config.access_sort);
    }

    #[test]
    fn option_one_per_line() {
        let mut config = LsConfig::new();
        config
            .parse(string_vec!["ls.exe", "myfile.txt", "-1"])
            .unwrap();
        assert!(config.one_per_line);
    }
}
