use std::env;

mod config;


fn main() {
    let mut config = config::YesConfig::new();
    if let Err(e) = config.parse(env::args().collect()) {
        eprintln!("{}", e);
        return;
    }
    loop {
        println!("{}", config.output_string);
    }
}
