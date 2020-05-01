use std::path::PathBuf;

pub fn get_path(filename: &str) -> PathBuf {
    use std::env;

    let mut cwd = env::current_dir().expect("Failed to get working directory");

    cwd.push("src");
    cwd.push("test_utils");
    cwd.push(filename);
    cwd
}
