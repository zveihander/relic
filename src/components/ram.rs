use std::fs::File;
use std::io::Read;
use std::sync::Mutex;
use std::time::{Duration, Instant};

const KIB_TO_GIB: f32 = 1024.0 * 1024.0;

static CACHE: Mutex<Option<(u64, u64, u64, u64, u64, Instant)>> = Mutex::new(None);

fn util_read_sysfs() -> (u64, u64, u64, u64, u64) {
    let mut cache = CACHE.lock().unwrap();

    // If the CACHE is less than 500ms old, reuse it
    if let Some((t, a, f, st, sf, last_update)) = *cache
        && last_update.elapsed() < Duration::from_millis(500)
    {
        return (t, a, f, st, sf);
    }

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
        for line in buf[..n].split(|&b| b == b'\n') {
            let mut parts = line.split(|&b| b == b' ').filter(|p| !p.is_empty());
            let Some(key) = parts.next() else { continue };

            let target = match key {
                b"MemTotal:" => {
                    found_count += 1;
                    &mut total
                }
                b"MemAvailable:" => {
                    found_count += 1;
                    &mut avail
                }
                b"MemFree:" => {
                    found_count += 1;
                    &mut free
                }
                b"SwapTotal:" => {
                    found_count += 1;
                    &mut s_total
                }
                b"SwapFree:" => {
                    found_count += 1;
                    &mut s_free
                }
                _ => continue,
            };

            if let Some(val_bytes) = parts.next() {
                let mut res = 0u64;
                for &b in val_bytes {
                    if b.is_ascii_digit() {
                        res = res * 10 + (b - b'0') as u64;
                    }
                }
                *target = res;
            }

            if found_count == 5 {
                break;
            }
        }
    }
    *cache = Some((total, avail, free, s_total, s_free, Instant::now()));
    (total, avail, free, s_total, s_free)
}

#[cfg(feature = "ram_total")]
pub fn ram_total(_: &str) -> String {
    format!("{:.1}", util_read_sysfs().0 as f32 / KIB_TO_GIB)
}

#[cfg(feature = "ram_free")]
pub fn ram_free(_: &str) -> String {
    format!("{}", util_read_sysfs().2 / 1024)
}

#[cfg(feature = "ram_used")]
pub fn ram_used(_: &str) -> String {
    let (t, a, _, _, _) = util_read_sysfs();
    format!("{:.1}", t.saturating_sub(a) as f32 / KIB_TO_GIB)
}

#[cfg(feature = "ram_perc")]
pub fn ram_perc(_: &str) -> String {
    let (t, a, _, _, _) = util_read_sysfs();
    if t == 0 {
        return "0".into();
    }
    format!("{}", (t.saturating_sub(a) * 100) / t)
}

#[cfg(feature = "swap_total")]
pub fn swap_total(_: &str) -> String {
    format!("{:.1}", util_read_sysfs().3 as f32 / KIB_TO_GIB)
}

#[cfg(feature = "swap_free")]
pub fn swap_free(_: &str) -> String {
    format!("{:.1}", util_read_sysfs().4 as f32 / KIB_TO_GIB)
}

#[cfg(feature = "swap_used")]
pub fn swap_used(_: &str) -> String {
    let (_, _, _, t, f) = util_read_sysfs();
    format!("{:.1}", t.saturating_sub(f) as f32 / KIB_TO_GIB)
}

#[cfg(feature = "swap_perc")]
pub fn swap_perc(_: &str) -> String {
    let (_, _, _, t, f) = util_read_sysfs();
    if t == 0 {
        return "0".into();
    }
    format!("{}", (t.saturating_sub(f) * 100) / t)
}
