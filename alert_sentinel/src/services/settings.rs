use dioxus::logger::tracing::debug;
use dirs::config_dir;
use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::types::settings::AppConfig;

/// Returns the path to the application's configuration file.
pub fn config_path() -> PathBuf {
    config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("alert_sentinel")
        .join("settings.json")
}

/// Loads the application configuration from the config file.
/// If the file does not exist or fails to load, returns the default configuration.
///
/// # Returns
/// AppConfig loaded from file or default.
pub fn load_config() -> AppConfig {
    let path = config_path();
    if let Ok(data) = fs::read_to_string(path) {
        serde_json::from_str(&data).unwrap_or_default()
    } else {
        debug!("Config file not found or failed to load. Using default settings.");
        AppConfig::default()
    }
}

/// Saves the template image from the source path to the application's template directory.
/// Returns the destination path as a string if successful.
///
/// # Arguments
/// * `src_path` - Source file path of the template image.
/// * `name` - Name to use for the saved template image.
///
/// # Returns
/// Some(String) with the destination path if successful, None otherwise.
pub fn save_template_image(src_path: &str, name: &str) -> Option<String> {
    let src = Path::new(src_path);
    let dest_dir = dirs::data_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("alert_sentinel")
        .join("templates");
    std::fs::create_dir_all(&dest_dir).ok()?;
    let dest_path = dest_dir.join(format!("{name}.png"));
    if std::fs::copy(src, &dest_path).is_ok() {
        Some(dest_path.to_string_lossy().to_string())
    } else {
        None
    }
}

/// Returns the file path to a license file in the application's assets directory.
///
/// # Arguments
/// * `exe_path` - Path to the current executable.
/// * `file_name` - Name of the license file.
///
/// # Returns
/// PathBuf to the license file.
pub fn get_license_file_path(exe_path: &Path, file_name: &str) -> PathBuf {
    let mut path = exe_path.to_path_buf();
    path.pop();
    path.push("assets");
    path.push("licenses");
    path.push(file_name);
    path
}
