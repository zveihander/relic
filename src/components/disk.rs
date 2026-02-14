use std::ffi::CString;
use std::mem::MaybeUninit;

fn util_statvfs(path: &str) -> Option<libc::statvfs> {
    let c_path = CString::new(path).ok()?;
    let mut stats = MaybeUninit::<libc::statvfs>::uninit();
    unsafe {
        if libc::statvfs(c_path.as_ptr(), stats.as_mut_ptr()) == 0 {
            Some(stats.assume_init())
        } else {
            None
        }
    }
}

#[cfg(feature = "disk_total")]
pub fn disk_total(path: &str) -> String {
    let path = if path.is_empty() { "/" } else { path };
    match util_statvfs(path) {
        Some(s) => format!(
            "{:.1}",
            (s.f_blocks as f64 * s.f_frsize as f64) / 1024.0 / 1024.0 / 1024.0
        ),
        None => "0.0".to_string(),
    }
}

#[cfg(feature = "disk_free")]
pub fn disk_free(path: &str) -> String {
    let path = if path.is_empty() { "/" } else { path };
    match util_statvfs(path) {
        Some(s) => format!(
            "{:.1}",
            (s.f_bavail as f64 * s.f_frsize as f64) / 1024.0 / 1024.0 / 1024.0
        ),
        None => "0.0".to_string(),
    }
}

#[cfg(feature = "disk_used")]
pub fn disk_used(path: &str) -> String {
    let path = if path.is_empty() { "/" } else { path };
    match util_statvfs(path) {
        Some(s) => {
            let total = s.f_blocks as f64 * s.f_frsize as f64;
            let free = s.f_bfree as f64 * s.f_frsize as f64;
            format!("{:.1}", (total - free) / 1024.0 / 1024.0 / 1024.0)
        }
        None => "0.0".to_string(),
    }
}

#[cfg(feature = "disk_perc")]
pub fn disk_perc(path: &str) -> String {
    let path = if path.is_empty() { "/" } else { path };
    match util_statvfs(path) {
        Some(s) => {
            let total = s.f_blocks;
            let used = total - s.f_bfree;
            if total == 0 {
                return "0".into();
            }
            format!("{}", (used * 100) / total)
        }
        None => "0".to_string(),
    }
}
