use std::path::PathBuf;

pub fn get_path(filename: &str) -> PathBuf {
    use std::env;

    let mut cwd = env::current_dir().expect("Failed to get working directory");

    cwd.push("src");
    cwd.push("test_utils");
    cwd.push(filename);
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