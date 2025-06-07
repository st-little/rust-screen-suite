use chrono::{DateTime, Utc};
use chrono_tz::Tz;

/// Formats the detection time in the specified timezone as a string.
///
/// # Arguments
/// * `dt` - The detection time as a DateTime<Utc>.
/// * `tz` - The timezone to convert the detection time to.
///
/// # Returns
/// A string representing the formatted detection time in "YYYY/MM/DD HH:MM:SS" format.
pub fn format_detection_time(dt: &DateTime<Utc>, tz: &Tz) -> String {
    dt.with_timezone(tz).format("%Y/%m/%d %H:%M:%S").to_string()
}
