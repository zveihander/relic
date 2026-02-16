use mpris::{PlaybackStatus, PlayerFinder};
use std::time::{SystemTime, UNIX_EPOCH};

fn util_scroll_text(text: String, max_len: usize) -> String {
    let char_count = text.chars().count();
    if char_count <= max_len {
        return text;
    }

    let elapsed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as usize;

    let text_with_gap = format!("{}   ", text);
    let chars: Vec<char> = text_with_gap.chars().collect();
    let len = chars.len();

    chars
        .iter()
        .cycle()
        .skip(elapsed % len)
        .take(max_len)
        .collect()
}

fn util_get_mpris_data() -> (String, String) {
    let finder = match PlayerFinder::new() {
        Ok(f) => f,
        Err(_) => return ("Stopped".to_string(), "".to_string()),
    };

    let player = match finder.find_active() {
        Ok(p) => p,
        Err(_) => return ("Stopped".to_string(), "".to_string()),
    };

    let status = match player.get_playback_status() {
        Ok(PlaybackStatus::Playing) => "Playing",
        Ok(PlaybackStatus::Paused) => "Paused",
        _ => "Stopped",
    }
    .to_string();

    let meta = player
        .get_metadata()
        .ok()
        .map(|m| {
            let artist = m
                .artists()
                .map(|a| a.join(", "))
                .unwrap_or_else(|| "Unknown".to_string());
            let title = m.title().unwrap_or("Unknown").to_string();
            format!("{} - {}", artist, title)
        })
        .unwrap_or_default();

    (status, meta)
}

#[cfg(feature = "mpris")]
pub fn mpris(max_len: &str) -> String {
    let (_, meta) = util_get_mpris_data();
    if meta.is_empty() {
        return "No Media".into();
    }

    let limit = max_len.parse::<usize>().unwrap_or(20);
    util_scroll_text(meta, limit)
}

#[cfg(feature = "mpris_icon")]
pub fn mpris_icon(max_len: &str) -> String {
    let (status, meta) = util_get_mpris_data();
    let limit = max_len.parse::<usize>().unwrap_or(20);

    if meta.is_empty() {
        return "󰝛".into();
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
