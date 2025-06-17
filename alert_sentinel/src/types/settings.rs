use serde::{Deserialize, Serialize};

/// Application configuration structure.
/// Holds all user-configurable settings for the application.
#[derive(Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub language: String,
    pub theme: String,
    pub timezone: String,
    pub monitor_interval_ms: u64,
    pub monitor_skip_duration_ms: u64,
    pub monitor_threshold: f32,
    pub monitor_scale: f32,
    pub sound_path: String,
    pub sound_enabled: bool,
    pub sound_volume: u8,
    pub discord_enabled: bool,
    pub discord_webhook: Option<String>,
    pub discord_username: Option<String>,
    pub discord_avatar_url: Option<String>,
    pub templates: TemplateConfig,
    pub toggles: NotificationToggles,
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            language: "en-US".to_string(),
            theme: "night".to_string(),
            timezone: "UTC".to_string(),
            monitor_interval_ms: 1000,
            monitor_skip_duration_ms: 2000,
            monitor_threshold: 0.7,
            monitor_scale: 1.0,
            sound_path: "C:\\Windows\\Media\\Windows Background.wav".to_string(),
            sound_enabled: false,
            sound_volume: 80,
            discord_enabled: false,
            discord_webhook: None,
            discord_username: Some("Alert Sentinel".to_string()),
            discord_avatar_url: None,
            templates: TemplateConfig::default(),
            toggles: NotificationToggles::default(),
        }
    }
}

/// TemplateConfig holds the file paths for each alert template image.
#[derive(Serialize, Deserialize, Default, Clone)]
pub struct TemplateConfig {
    pub airdrop: Option<String>,
    pub heli: Option<String>,
    pub ch47: Option<String>,
    pub cargo: Option<String>,
    pub shovel: Option<String>,
    pub oil_rig_large: Option<String>,
    pub oil_rig_small: Option<String>,
}

/// NotificationToggles holds the enable/disable state for each alert type.
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct NotificationToggles {
    pub airdrop: bool,
    pub heli: bool,
    pub ch47: bool,
    pub cargo: bool,
    pub shovel: bool,
    pub oil_rig_large: bool,
    pub oil_rig_small: bool,
}
