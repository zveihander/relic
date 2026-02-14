use std::fs::File;
use std::io::Read;

fn util_read_sysfs() -> (u64, u64, u64, u64, u64) {
    let mut buf = [0u8; 1024];
    let mut total = 0;
    let mut avail = 0;
    let mut free = 0;
    let mut s_total = 0;
    let mut s_free = 0;
    let mut found_count = 0;

    if let Ok(mut file) = File::open("/proc/meminfo")
        && let Ok(n) = file.read(&mut buf)
    {
        let s = std::str::from_utf8(&buf[..n]).unwrap_or("");
        for line in s.lines() {
            let mut parts = line.split_whitespace();
            let Some(key) = parts.next() else { continue };

            let target = match key {
                "MemTotal:" => {
                    found_count += 1;
                    &mut total
                }
                "MemAvailable:" => {
                    found_count += 1;
                    &mut avail
                }
                "MemFree:" => {
                    found_count += 1;
                    &mut free
                }
                "SwapTotal:" => {
                    found_count += 1;
                    &mut s_total
                }
                "SwapFree:" => {
                    found_count += 1;
                    &mut s_free
                }
                _ => continue,
            };

            if let Some(val_str) = parts.next()
                && let Ok(val) = val_str.parse::<u64>()
            {
                *target = val;
            }

            if found_count == 5 {
                break;
            }
        }
    }
    (total, avail, free, s_total, s_free)
}

#[cfg(feature = "ram_total")]
pub fn ram_total(_: &str) -> String {
    let (total, _, _, _, _) = util_read_sysfs();
    format!("{:.1}", total as f32 / 1024.0 / 1024.0)
}

#[cfg(feature = "ram_free")]
pub fn ram_free(_: &str) -> String {
    let (_, _, free, _, _) = util_read_sysfs();
    format!("{}", free / 1024)
}

#[cfg(feature = "ram_used")]
pub fn ram_used(_: &str) -> String {
    let (total, available, _, _, _) = util_read_sysfs();
    let used = total.saturating_sub(available);
    format!("{:.1}", used as f32 / 1024.0 / 1024.0)
}

#[cfg(feature = "ram_perc")]
pub fn ram_perc(_: &str) -> String {
    let (total, available, _, _, _) = util_read_sysfs();
    if total == 0 {
        return "0".to_string();
    }

    let used = total.saturating_sub(available);
    let perc = (used * 100) / total;
    perc.to_string()
}

#[cfg(feature = "swap_total")]
pub fn swap_total(_: &str) -> String {
    let (_, _, _, total, _) = util_read_sysfs();
    format!("{:.1}", total as f32 / 1048576.0)
}

#[cfg(feature = "swap_free")]
pub fn swap_free(_: &str) -> String {
    let (_, _, _, _, free) = util_read_sysfs();
    format!("{:.1}", free as f32 / 1048576.0)
}

#[cfg(feature = "swap_used")]
pub fn swap_used(_: &str) -> String {
    let (_, _, _, total, free) = util_read_sysfs();
    let used = total.saturating_sub(free);
    format!("{:.1}", used as f32 / 1048576.0)
}

#[cfg(feature = "swap_perc")]
pub fn swap_perc(_: &str) -> String {
    let (_, _, _, total, free) = util_read_sysfs();
    if total == 0 {
        return "0".into();
    }

    let used = total.saturating_sub(free);
    let perc = (used * 100) / total;
    perc.to_string()
}
