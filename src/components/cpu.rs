use std::fs::File;
use std::io::Read;
use std::sync::atomic::{AtomicU64, Ordering};

static PREV_IDLE: AtomicU64 = AtomicU64::new(0);
static PREV_TOTAL: AtomicU64 = AtomicU64::new(0);

#[cfg(feature = "cpu_perc")]
pub fn cpu_perc(_: &str) -> String {
    let mut buffer = [0u8; 1024];

    let mut file = match File::open("/proc/stat") {
        Ok(f) => f,
        Err(_) => return "N/A".to_string(),
    };

    if file.read(&mut buffer).is_err() {
        return "N/A".to_string();
    }

    let contents = std::str::from_utf8(&buffer).unwrap_or("");
    let first_line = contents.lines().next().unwrap_or("");

    let parts = first_line
        .split_whitespace()
        .skip(1)
        .filter_map(|s| s.parse::<u64>().ok());

    let mut total: u64 = 0;
    let mut idle: u64 = 0;

    for (idx, val) in parts.enumerate() {
        total += val;
        if idx == 3 || idx == 4 {
            idle += val;
        }
    }

    let prev_idle = PREV_IDLE.swap(idle, Ordering::Relaxed);
    let prev_total = PREV_TOTAL.swap(total, Ordering::Relaxed);

    let idle_delta = idle.saturating_sub(prev_idle);
    let total_delta = total.saturating_sub(prev_total);

    if total_delta == 0 {
        return "0".to_string();
    }

    let usage = 100.0 * (1.0 - (idle_delta as f64 / total_delta as f64));
    format!("{:.0}", usage)
}
