use time::{OffsetDateTime, format_description::parse};

pub fn datetime(format: &'static str) -> String {
    let now = OffsetDateTime::now_local().expect("[DATETIME] ERROR: Failed to get local time.");

    let format_desc = parse(format).expect("[DATETIME] ERROR: Failed to parse format string.");

    now.format(&format_desc)
        .expect("[DATETIME] ERROR: An error occured when formatting the string.")
}
