use std::process::Command;

fn get_wpctl_info(id: &str) -> (f32, bool) {
    let target = if id.is_empty() {
        "@DEFAULT_AUDIO_SINK@"
    } else {
        id
    };

    let output = Command::new("wpctl").args(["get-volume", target]).output();

    if let Ok(out) = output {
        let s = String::from_utf8_lossy(&out.stdout);
        let muted = s.contains("[MUTED]");
        let vol_str = s.split_whitespace().nth(1).unwrap_or("0.0");

        let vol = vol_str.parse::<f32>().unwrap_or(0.0) * 100.0;
        return (vol, muted);
    }
    (0.0, true)
}

#[cfg(feature = "pipewire")]
pub fn pipewire(id: &str) -> String {
    let (vol, muted) = get_wpctl_info(id);
    if muted {
        "MUTED".to_string()
    } else {
        format!("{:.0}%", vol)
    }
}

#[cfg(feature = "pipewire_icon")]
pub fn pipewire_icon(id: &str) -> String {
    let (vol, muted) = get_wpctl_info(id);

    if muted {
        return "󰝟 MUTED".to_string();
    }

    let icon = match vol as u32 {
        0 => "󰝟",
        1..=30 => "󰕿",
        31..=60 => "󰖀",
        _ => "󰕾",
    };

    format!("{} {:.0}%", icon, vol)
}
