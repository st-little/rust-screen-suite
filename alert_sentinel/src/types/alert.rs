use chrono::{DateTime, Utc};
use opencv::prelude::*;
use serde::{Deserialize, Serialize};

/// Enum representing the type of alert detected.
#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum AlertKind {
    Airdrop,
    Heli,
    Ch47,
    Cargo,
    Shovel,
    OilRigLarge,
    OilRigSmall,
}

impl AlertKind {
    /// Returns the localization key string for each alert kind.
    pub fn as_key(&self) -> &'static str {
        match self {
            AlertKind::Airdrop => "setting_template_airdrop",
            AlertKind::Heli => "setting_template_heli",
            AlertKind::Ch47 => "setting_template_ch47",
            AlertKind::Cargo => "setting_template_cargo",
            AlertKind::Shovel => "setting_template_shovel",
            AlertKind::OilRigLarge => "setting_template_oil_rig_large",
            AlertKind::OilRigSmall => "setting_template_oil_rig_small",
        }
    }
}

/// Struct representing a single alert detection event.
#[derive(Serialize, Deserialize, Clone)]
pub struct AlertDetection {
    pub timestamp: DateTime<Utc>,
    pub kind: AlertKind,
    pub score: f64,
}

/// Struct holding OpenCV Mats for each alert template image.
/// Used for template matching in detection.
pub struct TemplateCache {
    pub airdrop: Option<Mat>,
    pub heli: Option<Mat>,
    pub ch47: Option<Mat>,
    pub cargo: Option<Mat>,
    pub shovel: Option<Mat>,
    pub oil_rig_large: Option<Mat>,
    pub oil_rig_small: Option<Mat>,
}
