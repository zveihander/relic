use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

fn util_scroll_text(text: String, max_len: usize) -> String {
    if text.chars().count() <= max_len {
        return text;
    }

    let elapsed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    let text_with_gap = format!("{}   ", text);
    let chars: Vec<char> = text_with_gap.chars().collect();
    let len = chars.len();
    let offset = (elapsed as usize) % len;

    let mut scrolled = String::new();
    for i in 0..max_len {
        scrolled.push(chars[(offset + i) % len]);
    }
    scrolled
}

fn util_get_mpris_data() -> (String, String) {
    let output = Command::new("playerctl")
        .args(["metadata", "--format", "{{status}}|{{artist}} - {{title}}"])
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_else(|_| "".to_string());

    if output.is_empty() {
        return ("Stopped".to_string(), "".to_string());
    }

    let parts: Vec<&str> = output.splitn(2, '|').collect();
    (
        parts.get(0).unwrap_or(&"Stopped").to_string(),
        parts.get(1).unwrap_or(&"").to_string(),
    )
}

#[cfg(feature = "mpris")]
pub fn mpris(max_len: &str) -> String {
    let (_, meta) = util_get_mpris_data();
    if meta.is_empty() {
        return "No Media".to_string();
    }

    let limit = max_len.parse::<usize>().unwrap_or(20);
    util_scroll_text(meta, limit)
}

#[cfg(feature = "mpris_icon")]
pub fn mpris_icon(max_len: &str) -> String {
    let (status, meta) = util_get_mpris_data();
    let limit = max_len.parse::<usize>().unwrap_or(20);

    if meta.is_empty() {
        return "󰝛".to_string();
    }

    let state_icon = match status.as_str() {
        "Playing" => " ",
        "Paused" => " ",
        _ => "",
    };

    let scrolled_meta = util_scroll_text(meta, limit);

    // Format: [Icon] [Play/Pause] [Artist - Title]
    format!("󰝚  {} {}", state_icon, scrolled_meta)
}
