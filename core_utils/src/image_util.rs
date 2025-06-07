use base64::Engine as _;
use base64::engine::general_purpose::STANDARD as BASE64_STANDARD;
use std::fs;

/// Converts an image file to a data URL (base64-encoded PNG).
///
/// # Arguments
/// * `path` - Path to the image file.
///
/// # Returns
/// Some(String) containing the data URL if successful, None otherwise.
pub fn image_file_to_data_url(path: &str) -> Option<String> {
    let data = fs::read(path).ok()?;
    let encoded = BASE64_STANDARD.encode(&data);
    Some(format!("data:image/png;base64,{}", encoded))
}
