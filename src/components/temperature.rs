use std::fs;

#[cfg(feature = "temperature_c")]
pub fn temperature_c(sensor_path: &str) -> String {
    let path = if sensor_path.starts_with('/') {
        sensor_path.to_string()
    } else {
        format!("/sys/class/thermal/{}", sensor_path)
    };

    match fs::read_to_string(&path) {
        Ok(content) => {
            let raw_temp = content.trim().parse::<f32>().unwrap_or(0.0);
            format!("{:.0}°C", raw_temp / 1000.0)
        }
        Err(_) => "N/A".to_string(),
    }
}

#[cfg(feature = "temperature_f")]
pub fn temperature_f(sensor_path: &str) -> String {
    let path = if sensor_path.starts_with('/') {
        sensor_path.to_string()
    } else {
        format!("/sys/class/thermal/{}", sensor_path)
    };

    match fs::read_to_string(&path) {
        Ok(content) => {
            let raw_temp = content.trim().parse::<f32>().unwrap_or(0.0);
            let celsius = raw_temp / 1000.0;
            let fahrenheit = (celsius * 9.0 / 5.0) + 32.0;
            format!("{:.0}°F", fahrenheit)
        }
        Err(_) => "N/A".to_string(),
    }
}
