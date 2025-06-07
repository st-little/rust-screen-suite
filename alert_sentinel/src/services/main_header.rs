use std::path::PathBuf;

/// Returns the file path to the help HTML file for the specified language.
/// If the file does not exist, falls back to the English help file.
///
/// # Arguments
/// * `exe_path` - Path to the current executable.
/// * `lang` - Language code (e.g., "en-US").
///
/// # Returns
/// PathBuf to the help HTML file.
pub fn get_help_file_path(exe_path: &std::path::Path, lang: &str) -> PathBuf {
    let mut help_path = exe_path.to_path_buf();
    help_path.pop();
    help_path.push("assets");
    help_path.push("docs");
    help_path.push("help");
    help_path.push(format!("help_{}.html", lang));
    if help_path.exists() {
        help_path
    } else {
        let mut fallback = exe_path.to_path_buf();
        fallback.pop();
        fallback.push("assets");
        fallback.push("docs");
        fallback.push("help");
        fallback.push("help_en-US.html");
        fallback
    }
}
