use std::process::Command;

fn get_wpctl_info(id: &str) -> (f32, bool) {
    let target = if id.is_empty() {
        "@DEFAULT_AUDIO_SINK@"
    } else {
        id
    };

    let output = Command::new("wpctl").args(["get-volume", target]).output();

    if let Ok(out) = output {
        let muted = out.stdout.windows(7).any(|w| w == b"[MUTED]");

        let s = std::str::from_utf8(&out.stdout).unwrap_or("");
        let mut parts = s.split_whitespace();
        let vol = parts
            .nth(1)
            .and_then(|v| v.parse::<f32>().ok())
            .unwrap_or(0.0)
            * 100.0;

        return (vol, muted);
    }
    (0.0, true)
}

#[cfg(feature = "pipewire")]
pub fn pipewire(id: &str) -> String {
    let (vol, muted) = get_wpctl_info(id);
    if muted {
        "MUTED".into()
    } else {
        format!("{:.0}%", vol)
    }
}

#[cfg(feature = "pipewire_icon")]
pub fn pipewire_icon(id: &str) -> String {
    let (vol, muted) = get_wpctl_info(id);

    if muted {
        return "󰝟 MUTED".into();
    }

    let icon = match vol as u32 {
        0 => "󰝟",
        1..=30 => "󰕿",
        31..=60 => "󰖀",
        _ => "󰕾",
    };

    format!("{} {:.0}%", icon, vol)
}
