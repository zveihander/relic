use libc;
use std::ffi::CStr;
use std::process::Command;

#[cfg(feature = "hostname")]
pub fn hostname(_: &str) -> String {
    let mut buf = [0 as libc::c_char; 255];
    unsafe {
        if libc::gethostname(buf.as_mut_ptr(), buf.len()) == 0 {
            CStr::from_ptr(buf.as_ptr()).to_string_lossy().into_owned()
        } else {
            "Unknown".to_string()
        }
    }
}

#[cfg(feature = "user")]
pub fn username(_: &str) -> String {
    unsafe {
        let ptr = libc::getlogin();
        if !ptr.is_null() {
            CStr::from_ptr(ptr).to_string_lossy().into_owned()
        } else {
            std::env::var("USER").unwrap_or_else(|_| "Unknown".to_string())
        }
    }
}

#[cfg(feature = "userathost")]
pub fn userathost(_: &str) -> String {
    let mut buf = [0 as libc::c_char; 255];
    unsafe {
        let ptr = libc::getlogin();
        let username = if !ptr.is_null() {
            CStr::from_ptr(ptr).to_string_lossy().into_owned()
        } else {
            std::env::var("USER").unwrap_or_else(|_| "Unknown".to_string())
        };
        let hostname = if libc::gethostname(buf.as_mut_ptr(), buf.len()) == 0 {
            CStr::from_ptr(buf.as_ptr()).to_string_lossy().into_owned()
        } else {
            "Unknown".to_string()
        };
        format!("{}@{}", username, hostname)
    }
}

#[cfg(feature = "updates")]
pub fn updates(manager: &str) -> String {
    let count = match manager {
        "xbps" => Command::new("xbps-install")
            .args(["-nuM"])
            .output()
            .map(|o| String::from_utf8_lossy(&o.stdout).lines().count())
            .unwrap_or(0),
        _ => 0,
    };

    if count > 0 {
        count.to_string()
    } else {
        "0".to_string()
    }
}
