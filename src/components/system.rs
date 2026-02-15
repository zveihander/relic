use libc;
use std::ffi::CStr;
use std::process::Command;

// Just a little helper to get C Strings without allocating Rust strings
unsafe fn get_c_str(ptr: *const libc::c_char) -> Option<&'static str> {
    if ptr.is_null() {
        return None;
    }
    CStr::from_ptr(ptr).to_str().ok()
}

#[cfg(any(feature = "hostname", feature = "userathost"))]
fn get_hostname_static(buf: &mut [libc::c_char]) -> &str {
    unsafe {
        if libc::gethostname(buf.as_mut_ptr(), buf.len()) == 0 {
            CStr::from_ptr(buf.as_ptr()).to_str().unwrap_or("Unknown")
        } else {
            "Unknown"
        }
    }
}

#[cfg(any(feature = "user", feature = "userathost"))]
fn get_user_static() -> &'static str {
    unsafe { get_c_str(libc::getlogin()).unwrap_or("Unknown") }
}

#[cfg(feature = "hostname")]
pub fn hostname(_: &str) -> String {
    let mut buf = [0 as libc::c_char; 255];
    get_hostname_static(&mut buf).to_string()
}

#[cfg(feature = "user")]
pub fn username(_: &str) -> String {
    get_user_static().to_string()
}

#[cfg(feature = "userathost")]
pub fn userathost(_: &str) -> String {
    let mut buf = [0 as libc::c_char; 255];
    format!("{}@{}", get_user_static(), get_hostname_static(&mut buf))
}

#[cfg(feature = "kernel")]
pub fn kernel(_: &str) -> String {
    let mut buf = [0u8; 64];
    if let Ok(mut f) = std::fs::File::open("/proc/sys/kernel/osrelease") {
        use std::io::Read;
        if let Ok(n) = f.read(&mut buf) {
            return std::str::from_utf8(&buf[..n])
                .unwrap_or("unknown")
                .trim()
                .to_string();
        }
    }
    "unknown".to_string()
}

#[cfg(feature = "kernel")]
pub fn kernel(_: &str) -> String {
    std::fs::read_to_string("/proc/sys/kernel/osrelease")
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|_| "unknown".to_string())
}

// TODO: Fix XBPS.
// TODO: Implement update checks for other package managers.
#[cfg(feature = "updates")]
pub fn updates(manager: &str) -> String {
    let count = match manager {
        "xbps" => {
            Command::new("xbps-install") // NOTE: This currently checks the offline cache for package updates. I need to find a way to sync the repos and check for updates.
                .args(["-nuM"])
                .output()
                .map(|o| {
                    o.stdout
                        .split(|&b| b == b'\n')
                        .filter(|l| !l.is_empty())
                        .count()
                })
                .unwrap_or(0)
        }
        _ => 0,
    };
    count.to_string()
}
