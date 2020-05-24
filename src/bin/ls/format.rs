use std::ffi::OsStr;
use std::fs::DirEntry;

extern crate chrono;
use chrono::offset::Utc;
use chrono::DateTime;

fn filename(item: &DirEntry) -> String {
    item.path()
        .file_name()
        .unwrap_or(OsStr::new("?"))
        .to_string_lossy()
        .into_owned()
}

fn permissions(item: &DirEntry) -> String {
    let mut output = String::new();
    if let Ok(file_type) = item.file_type() {
        if file_type.is_dir() {
            output.push('d')
        } else if file_type.is_symlink() {
            output.push('l')
        } else {
            output.push('-')
        }
    } else {
        output.push('?')
    }

    if let Ok(meta) = item.metadata() {
        if meta.permissions().readonly() {
            output.push_str("r-xr-xr-x");
        } else {
            output.push_str("rwxrwxrwx");
        }
    } else {
        output.push_str("???????");
    }
    output
}

fn size(item: &DirEntry) -> String {
    const POSTFIXES: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    if let Ok(meta) = item.metadata() {
        if meta.file_type().is_dir() {
            String::from("<DIR>")
        } else if meta.file_type().is_symlink() {
            String::from("<DIR>")
        } else {
            let mut size = meta.len() as f64;
            let mut idx = 0;
            while size >= 1024.0 {
                idx += 1;
                size /= 1024.0;
            }
            format!("{:.2} {}", size, POSTFIXES[idx])
        }
    } else {
        String::from("?")
    }
}

fn modified(item: &DirEntry) -> String {
    let mut output = String::from("?");
    if let Ok(meta) = item.metadata() {
        if let Ok(modified) = meta.modified() {
            let datetime: DateTime<Utc> = modified.into();
            output = format!("{}", datetime.format("%m/%d/%Y %R"))
        }
    }
    output
}

pub fn long_list(entries: Vec<DirEntry>) -> String {
    let mut output = String::new();
    for entry in entries {
        output.push_str(
            format!(
                "{} {} {:9} {} {}\n",
                permissions(&entry),
                crate::owner(&entry),
                size(&entry),
                modified(&entry),
                filename(&entry),
            )
            .as_str(),
        );
    }
    output
}

pub fn one_per_line(entries: Vec<DirEntry>) -> String {
    let mut output = String::new();
    for entry in entries {
        output.push_str(format!("{}\n", filename(&entry)).as_str());
    }
    output
}
