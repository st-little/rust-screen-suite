use dioxus::prelude::*;
use std::{fs, io};

use crate::{
    services::settings::{config_path, save_template_image},
    types::settings::AppConfig,
};

/// Saves the application configuration to the config file as pretty JSON.
///
/// # Arguments
/// * `config` - Reference to the AppConfig to save.
///
/// # Returns
/// Ok(()) if successful, or an io::Error if saving fails.
pub fn save_config(config: &AppConfig) -> Result<(), io::Error> {
    let path = config_path();

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let json = serde_json::to_string_pretty(config)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    fs::write(path, json)?;

    Ok(())
}

/// Saves all template images specified in the given paths and updates the config accordingly.
///
/// # Arguments
/// * `config` - Signal for the AppConfig to update.
/// * `paths` - Slice of tuples containing template names and their corresponding path signals.
pub fn save_all_templates(mut config: Signal<AppConfig>, paths: &[(&str, &Signal<String>)]) {
    for (name, path_signal) in paths {
        if let Some(new_path) = save_template_image(&path_signal(), name) {
            config.with_mut(|cfg| match *name {
                "airdrop" => cfg.templates.airdrop = Some(new_path),
                "heli" => cfg.templates.heli = Some(new_path),
                "ch47" => cfg.templates.ch47 = Some(new_path),
                "cargo" => cfg.templates.cargo = Some(new_path),
                "shovel" => cfg.templates.shovel = Some(new_path),
                "oil_rig_large" => cfg.templates.oil_rig_large = Some(new_path),
                "oil_rig_small" => cfg.templates.oil_rig_small = Some(new_path),
                _ => {}
            });
        }
    }
}
