use std::path::PathBuf;

#[macro_export]
macro_rules! string_vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(temp_vec.push($x);)*
            temp_vec.iter().map(|s| s.to_string()).collect()
        }
    };
}

pub fn get_path(filename: &str) -> PathBuf {
    let mut path = get_dir();
    path.push(filename);
    path
}

pub fn get_dir() -> PathBuf {
    use std::env;
    let mut cwd = env::current_dir().expect("Failed to get working directory");
    cwd.push("test");
    cwd
}

pub const ASCII: &str = "The quick brown fox jumps over the lazy dog\r\n";

pub const MULTI: &str = "this is a text file\r\n\
    with multiple lines\r\n\
    of text to show.\r\n";

pub const BLANK: &str = "this is a file\r\n\
\r\n\
that has\r\n\
\r\n\
blank lines.\r\n";

pub const MULTI_BLANK: &str = "this is a file\r\n\
\r\n\
\r\n\
that has\r\n\
\r\n\
\r\n\
multiple blank lines.\r\n";

pub const TABS: &str = "This\ttext\thas\tsome\ttabs\tin\tit.\r\n";

pub const NON_PRINTABLE: &str = "Lots \t of \0 control \0\0 characters.\r\n";
