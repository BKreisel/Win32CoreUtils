use coreutils::config::Config;

#[derive(Debug)]
pub struct YesConfig {
    pub output_string: String,
}

const HELP_TEXT: &str = "Usage: yes [STRING]...
    or:  yes OPTION
Repeatedly output a line with all specified STRING(s), or 'y'.

    --help     display this help and exit
    --version  output version information and exit
";

impl Config for YesConfig {
    fn bin_name(&self) -> &'static str {
        "yes"
    }
    fn usage(&self) -> &'static str {
        &HELP_TEXT
    }
}

impl YesConfig {
    pub fn new() -> YesConfig {
        YesConfig {
            output_string: String::new(),
        }
    }

    pub fn parse(&mut self, args: Vec<String>) -> Result<(), String> {
        let base_config = match Config::parse(self, &args) {
            Ok(config) => config,
            Err(e) => return Err(e),
        };

        if base_config.parameters.len() == 0 {
            self.output_string.push('y');
            return Ok(());
        }
        
        let mut first_word: bool = false;
        for param in base_config.parameters {
            match first_word {
                true => self.output_string.push(' '),
                false =>first_word = true,
            }
            self.output_string.push_str(param.as_str())
        }
        Ok(())
    }
}