use std::env;
use std::fs::{self, DirEntry};
use std::io;
use std::path::PathBuf;
use std::process;

use std::ffi::{OsStr, OsString};
use std::iter::once;
use std::mem;
use std::os::windows::ffi::OsStrExt;
use std::os::windows::prelude::*;
use std::ptr::null_mut;

extern crate winapi;
use winapi::shared::minwindef;
use winapi::shared::ntdef::NULL;
use winapi::shared::winerror;
use winapi::um::accctrl;
use winapi::um::aclapi;
use winapi::um::fileapi;
use winapi::um::handleapi;
use winapi::um::winbase;
use winapi::um::winnt;
mod config;
mod format;

const NORMAL: i32 = 0;
const NON_FATAL: i32 = 1;
const FATAL: i32 = 2;

fn main() {
    let mut exit_code = NORMAL;

    let mut config = config::LsConfig::new();
    if let Err(e) = config.parse(env::args().collect()) {
        eprintln!("{}", e);
        process::exit(FATAL);
    }

    for path in config.paths {
        let entries = match dir(&path) {
            Ok(entries) => entries,
            Err(e) => {
                eprintln!("{}", e);
                exit_code = NON_FATAL;
                continue;
            }
        };

        if config.one_per_line {
            print!("{}", format::one_per_line(entries));
        } else {
            print!("{}", format::long_list(entries));
        }

        process::exit(exit_code);
    }
}

fn dir(path: &PathBuf) -> io::Result<Vec<DirEntry>> {
    let mut entries: Vec<DirEntry> = Vec::new();
    for item in fs::read_dir(path)? {
        if let Ok(entry) = item {
            entries.push(entry);
        } else {
            eprintln!("{}", item.unwrap_err());
        }
    }
    Ok(entries)
}

fn owner(entry: &DirEntry) -> String {
    let wide_path: Vec<u16> = OsStr::new(&entry.path())
        .encode_wide()
        .chain(once(0))
        .collect();
    let h_file = unsafe {
        fileapi::CreateFileW(
            wide_path.as_ptr(),
            winnt::GENERIC_READ,
            winnt::FILE_SHARE_READ,
            null_mut(),
            fileapi::OPEN_EXISTING,
            winnt::FILE_ATTRIBUTE_NORMAL | winbase::FILE_FLAG_BACKUP_SEMANTICS,
            NULL,
        )
    };

    if h_file == handleapi::INVALID_HANDLE_VALUE {
        return format!("{:16} {:16}", "unknown", "unknown");
    }

    let mut p_sid_owner: winnt::PSID = null_mut();
    let mut p_descriptor: winnt::PSECURITY_DESCRIPTOR = null_mut();

    let dw_code = unsafe {
        aclapi::GetSecurityInfo(
            h_file,
            accctrl::SE_FILE_OBJECT,
            winnt::OWNER_SECURITY_INFORMATION,
            &mut p_sid_owner,
            null_mut(),
            null_mut(),
            null_mut(),
            &mut p_descriptor,
        )
    };

    if dw_code != winerror::ERROR_SUCCESS {
        return format!("{:16} {:16}", "unknown", "unknown");
    }

    let mut account_size: minwindef::DWORD = 50;
    let mut domain_size: minwindef::DWORD = 50;

    let mut account_name: Vec<u16> = unsafe { vec![mem::zeroed(); 50] };
    let mut domain_name: Vec<u16> = unsafe { vec![mem::zeroed(); 50] };

    let mut pe_use: winnt::SID_NAME_USE = winnt::SidTypeUnknown;

    let b_success = unsafe {
        winbase::LookupAccountSidW(
            null_mut(),
            p_sid_owner,
            account_name.as_mut_ptr(),
            &mut account_size,
            domain_name.as_mut_ptr(),
            &mut domain_size,
            &mut pe_use,
        )
    };

    if b_success == 0 {
        return format!("{:16} {:16}", "unknown", "unknown");
    }

    let account_size = account_name.iter().take_while(|&&c| c != 0).count();
    let domain_size = domain_name.iter().take_while(|&&c| c != 0).count();

    let account_name = OsString::from_wide(&account_name[..account_size]);
    let domain_name = OsString::from_wide(&domain_name[..domain_size]);
    format!(
        "{:16} {:16}",
        domain_name.to_string_lossy(),
        account_name.to_string_lossy()
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use coreutils::test_utils;
    use std::path::PathBuf;

    #[test]
    fn normal_dir() {
        let path = PathBuf::from(test_utils::get_dir());
        let entries: Vec<DirEntry> = crate::dir(&path).unwrap();
        let names: Vec<String> = entries
            .iter()
            .map(|x| x.path().file_name().unwrap().to_string_lossy().into_owned())
            .collect();

        let expected: Vec<&str> = vec![
            "ascii.txt",
            "hello.bin",
            "hidden",
            "link",
            "mklink",
            "normal",
            "notouch.txt",
        ];
        assert_eq!(names, expected)
    }
}
