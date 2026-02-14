use std::sync::OnceLock;
use time::{OffsetDateTime, format_description};

pub fn datetime(format: &'static str) -> String {
    static FORMAT_CACHE: OnceLock<Vec<format_description::FormatItem<'static>>> = OnceLock::new();

    let now = OffsetDateTime::now_local().expect("[DATETIME] ERROR: Failed to get local time.");

    let format_desc = FORMAT_CACHE.get_or_init(|| {
        format_description::parse(format).expect("[DATETIME] ERROR: Failed to parse format string.")
    });

    now.format(&format_desc)
        .expect("[DATETIME] ERROR: An error occured when formatting the string.")
}
