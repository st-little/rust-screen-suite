use dirs::data_dir;
use std::fs;
use std::io::{self};
use std::path::Path;
use std::path::PathBuf;

/// Returns the path to a data file in the application's data directory.
///
/// # Arguments
/// * `app_name` - Name of the application directory.
/// * `file_name` - Name of the file.
///
/// # Returns
/// PathBuf to the data file.
pub fn data_file_path(app_name: &str, file_name: &str) -> PathBuf {
    data_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(app_name)
        .join(file_name)
}

/// Saves a string to the specified file path, creating parent directories if needed.
///
/// # Arguments
/// * `path` - Path to the file.
/// * `data` - String data to write.
///
/// # Returns
/// Ok(()) if successful, or an io::Error if saving fails.
pub fn save_string_to_file(path: &Path, data: &str) -> io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, data)
}

/// Loads a string from the specified file path.
///
/// # Arguments
/// * `path` - Path to the file.
///
/// # Returns
/// Ok(String) if successful, or an io::Error if loading fails.
pub fn load_string_from_file(path: &Path) -> io::Result<String> {
    fs::read_to_string(path)
}
