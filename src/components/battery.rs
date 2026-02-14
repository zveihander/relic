use std::fs::File;
use std::io::Read;

fn util_read_sysfs(device: &str, file: &str, buf: &mut [u8]) -> Option<usize> {
    let path = format!("/sys/class/power_supply/{}/{}", device, file);

    let mut f = File::open(path).ok()?;
    let n = f.read(buf).ok()?;
    Some(n)
}

#[cfg(feature = "battery_perc")]
pub fn battery_perc(device: &str) -> String {
    let mut buf = [0u8; 16];
    if let Some(n) = util_read_sysfs(device, "capacity", &mut buf) {
        let s = std::str::from_utf8(&buf[..n]).unwrap_or("0").trim();
        return s.to_string();
    }
    "0".to_string()
}

#[cfg(feature = "battery_state")]
pub fn battery_state(device: &str) -> String {
    let mut buf = [0u8; 32];
    if let Some(n) = util_read_sysfs(device, "status", &mut buf) {
        let s = std::str::from_utf8(&buf[..n]).unwrap_or("Unknown").trim();
        return s.to_string();
    }
    "Unknown".to_string()
}

#[cfg(feature = "battery_custom")]
pub fn battery_custom(device: &str) -> String {
    let mut buf = [0u8; 64];

    let perc = if let Some(n) = util_read_sysfs(device, "capacity", &mut buf) {
        std::str::from_utf8(&buf[..n])
            .unwrap_or("0")
            .trim()
            .parse::<u8>()
            .unwrap_or(0)
    } else {
        0
    };

    let mut status_buf = [0u8; 32];
    let is_charging = if let Some(n) = util_read_sysfs(device, "status", &mut status_buf) {
        std::str::from_utf8(&status_buf[..n])
            .unwrap_or("")
            .starts_with('C')
    } else {
        false
    };

    let icon = if is_charging {
        "󱐋"
    } else {
        match perc {
            90..=100 => "󱊣",
            40..=89 => "󱊢",
            10..=39 => "󱊡",
            _ => "󰂎",
        }
    };

    format!("{} {}%", icon, perc)
}
