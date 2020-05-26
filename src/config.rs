pub struct BaseConfig {
    pub parameters: Vec<String>,
    pub options: Vec<String>,
}

pub trait Config {
    fn bin_name(&self) -> &'static str;
    fn usage(&self) -> &'static str;

    fn parse(&self, args: &[String]) -> Result<BaseConfig, String> {
        if let Some(err) = self.check_help(&args) {
            return Err(err);
        }

        if let Some(err) = self.check_version(&args) {
            return Err(err);
        }

        Ok(BaseConfig {
            parameters: self.gather_parameters(&args),
            options: self.gather_options(&args),
        })
    }

    fn gather_options(&self, args: &[String]) -> Vec<String> {
        args.iter()
            .skip(1)
            .filter(|arg| arg.starts_with('-'))
            .map(String::from)
            .collect()
    }

    fn gather_parameters(&self, args: &[String]) -> Vec<String> {
        args.iter()
            .skip(1)
            .filter(|arg| !arg.starts_with('-'))
            .map(String::from)
            .collect()
    }

    fn check_help(&self, args: &[String]) -> Option<String> {
        if args.contains(&"--help".to_string()) {
            return Some(self.usage().to_string());
        }
        None
    }

    fn check_version(&self, args: &[String]) -> Option<String> {
        if args.contains(&"--version".to_string()) {
            return Some(format!(
                "{} (Win32CoreUtils) v{}",
                self.bin_name(),
                env!("CARGO_PKG_VERSION")
            ));
        }
        None
    }
}
