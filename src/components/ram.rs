use std::fs::File;
use std::io::Read;

fn util_read_sysfs() -> (u64, u64, u64) {
    let mut buf = [0u8; 1024];
    let mut total = 0;
    let mut available = 0;
    let mut free = 0;
    let mut found_count = 0;

    if let Ok(mut file) = File::open("/proc/meminfo")
        && let Ok(n) = file.read(&mut buf)
    {
        let s = std::str::from_utf8(&buf[..n]).unwrap_or("");
        for line in s.lines() {
            let mut parts = line.split_whitespace();
            let Some(key) = parts.next() else { continue };

            let target = match key {
                "MemTotal:" => &mut total,
                "MemAvailable:" => &mut available,
                "MemFree:" => &mut free,
                _ => continue,
            };

            if let Some(val_str) = parts.next()
                && let Ok(val) = val_str.parse::<u64>()
            {
                *target = val;
                found_count += 1;
            }

            if found_count == 3 {
                break;
            }
        }
    }
    (total, available, free)
}

#[cfg(feature = "ram_total")]
pub fn ram_total(_: &str) -> String {
    let (total, _, _) = util_read_sysfs();
    format!("{:.1}", total as f32 / 1024.0 / 1024.0)
}

#[cfg(feature = "ram_free")]
pub fn ram_free(_: &str) -> String {
    let (_, _, free) = util_read_sysfs();
    format!("{}", free / 1024)
}

#[cfg(feature = "ram_used")]
pub fn ram_used(_: &str) -> String {
    let (total, available, _) = util_read_sysfs();
    let used = total.saturating_sub(available);
    format!("{:.1}", used as f32 / 1024.0 / 1024.0)
}

#[cfg(feature = "ram_perc")]
pub fn ram_perc(_: &str) -> String {
    let (total, available, _) = util_read_sysfs();
    if total == 0 {
        return "0".to_string();
    }

    let used = total.saturating_sub(available);
    let perc = (used * 100) / total;
    perc.to_string()
}
