use std::fs::File;

use std::io::Read;

fn util_get_temp(sensor_path: &str) -> Option<f32> {
    let mut buf = [0u8; 32];

    let mut path_buf = [0u8; 128];

    let path = if sensor_path.starts_with('/') {
        sensor_path
    } else {
        let prefix = b"/sys/class/thermal/";

        path_buf[..prefix.len()].copy_from_slice(prefix);

        path_buf[prefix.len()..prefix.len() + sensor_path.len()]
            .copy_from_slice(sensor_path.as_bytes());

        std::str::from_utf8(&path_buf[..prefix.len() + sensor_path.len()]).unwrap_or("")
    };

    let mut file = File::open(path).ok()?;

    let n = file.read(&mut buf).ok()?;

    std::str::from_utf8(&buf[..n])
        .ok()?
        .trim()
        .parse::<f32>()
        .ok()
}

#[cfg(feature = "temperature_c")]

pub fn temperature_c(sensor_path: &str) -> String {
    match util_get_temp(sensor_path) {
        Some(t) => format!("{:.0}°C", t / 1000.0),

        None => "N/A".into(),
    }
}

#[cfg(feature = "temperature_f")]

pub fn temperature_f(sensor_path: &str) -> String {
    match util_get_temp(sensor_path) {
        Some(t) => format!("{:.0}°F", (t / 1000.0) * 1.8 + 32.0),

        None => "N/A".into(),
    }
}
