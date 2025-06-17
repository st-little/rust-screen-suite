use anyhow::Result;
use core_utils::{
    capture::capture_screen_buffer,
    file_util::{data_file_path, load_string_from_file, save_string_to_file},
};
use dioxus::logger::tracing::debug;
use image::{imageops::FilterType, DynamicImage};
use opencv::{
    core::{self, Point},
    imgproc,
    prelude::*,
};
use std::{
    io,
    path::PathBuf,
    sync::{Arc, Mutex},
};

use crate::{
    types::alert::{AlertDetection, AlertKind, TemplateCache},
    types::settings::AppConfig,
};

/// Runs template matching for all enabled alert types and returns the first detected alert kind and its score.
///
/// # Arguments
/// * `config` - Reference to the application configuration.
/// * `templates` - Reference to the loaded template cache.
///
/// # Returns
/// Ok(Some((AlertKind, score))) if a detection is made, Ok(None) if not, or an error.
pub fn run_template_matching(
    config: &AppConfig,
    templates: &TemplateCache,
) -> Result<Option<(AlertKind, f64)>> {
    let img = capture_screen_buffer()?;
    let new_w = (img.width() as f32 * config.monitor_scale) as u32;
    let new_h = (img.height() as f32 * config.monitor_scale) as u32;
    let resized_img = image::imageops::resize(&img, new_w, new_h, FilterType::Lanczos3);

    let dyn_img = DynamicImage::ImageRgba8(resized_img);
    let gray = dyn_img.to_luma8();
    let width = gray.width() as usize;
    let buf = gray.as_raw();

    let rows: Vec<&[u8]> = buf.chunks(width).collect();
    let input = Mat::from_slice_2d(&rows)?;

    macro_rules! try_match {
        ($field:ident, $kind:expr) => {
            if config.toggles.$field {
                if let Some(tmpl) = &templates.$field {
                    {
                        let cols = input.cols() - tmpl.cols() + 1;
                        let rows = input.rows() - tmpl.rows() + 1;
                        let mut result = Mat::zeros(rows, cols, core::CV_32FC1)?.to_mat()?;

                        imgproc::match_template(
                            &input,
                            &tmpl,
                            &mut result,
                            imgproc::TM_CCOEFF_NORMED,
                            &core::no_array(),
                        )?;

                        let (mut _min_val, mut max_val) = (0.0, 0.0);
                        let (_min_loc, mut max_loc) = (Point::default(), Point::default());
                        core::min_max_loc(
                            &result,
                            None,
                            Some(&mut max_val),
                            None,
                            Some(&mut max_loc),
                            &core::no_array(),
                        )?;

                        if max_val < config.monitor_threshold as f64 {
                            debug!(
                                "`{}` undetected (score: {:.2})",
                                stringify!($field),
                                max_val
                            );
                        } else {
                            debug!(
                                "`{}` detection (score: {:.2}, pos: {:?})",
                                stringify!($field),
                                max_val,
                                max_loc
                            );
                            return Ok(Some(($kind, max_val)));
                        }
                    }
                }
            }
        };
    }

    try_match!(airdrop, AlertKind::Airdrop);
    try_match!(heli, AlertKind::Heli);
    try_match!(ch47, AlertKind::Ch47);
    try_match!(cargo, AlertKind::Cargo);
    try_match!(shovel, AlertKind::Shovel);
    try_match!(oil_rig_large, AlertKind::OilRigLarge);
    try_match!(oil_rig_small, AlertKind::OilRigSmall);

    Ok(None)
}

/// Returns the path to the detections file.
pub fn detections_path() -> PathBuf {
    data_file_path("alert_sentinel", "detections.json")
}

/// Saves the given detections to the detections file as pretty JSON.
///
/// # Arguments
/// * `detections` - Slice of AlertDetection to save.
///
/// # Errors
/// Returns an io::Error if saving fails.
pub fn save_detections(detections: &[AlertDetection]) -> io::Result<()> {
    let path = detections_path();
    let json = serde_json::to_string_pretty(detections)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    save_string_to_file(&path, &json)?;
    Ok(())
}

/// Loads detections from the detections file.
/// Returns a vector of AlertDetection. If loading fails, returns an empty vector.
pub fn load_detections() -> Vec<AlertDetection> {
    let path = detections_path();
    if let Ok(data) = load_string_from_file(&path) {
        serde_json::from_str(&data).unwrap_or_default()
    } else {
        Vec::new()
    }
}

/// Loads template images for all alert types, resizing and converting them to grayscale OpenCV Mats.
///
/// # Arguments
/// * `config` - Reference to the application configuration.
///
/// # Returns
/// A TemplateCache struct containing Mats for each alert type, or None if loading fails.
pub fn load_templates(config: &AppConfig) -> TemplateCache {
    let templates = &config.templates;
    let scale = config.monitor_scale;

    macro_rules! load_template {
        ($path:expr, $scale:expr) => {{
            if let Some(path) = $path {
                if let Ok(img) = image::open(path) {
                    let new_w = (img.width() as f32 * $scale) as u32;
                    let new_h = (img.height() as f32 * $scale) as u32;
                    let resized = image::imageops::resize(&img, new_w, new_h, FilterType::Lanczos3);
                    let dyn_img = DynamicImage::ImageRgba8(resized);
                    let gray = dyn_img.to_luma8();
                    let width = gray.width() as usize;
                    let buf = gray.as_raw();
                    let rows: Vec<&[u8]> = buf.chunks(width).collect();
                    if let Ok(mat) = Mat::from_slice_2d(&rows) {
                        Some(mat)
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            }
        }};
    }

    TemplateCache {
        airdrop: load_template!(&templates.airdrop, scale),
        heli: load_template!(&templates.heli, scale),
        ch47: load_template!(&templates.ch47, scale),
        cargo: load_template!(&templates.cargo, scale),
        shovel: load_template!(&templates.shovel, scale),
        oil_rig_large: load_template!(&templates.oil_rig_large, scale),
        oil_rig_small: load_template!(&templates.oil_rig_small, scale),
    }
}

/// Clears all detections from memory and deletes the detections file on disk.
///
/// # Arguments
/// * `detections` - Arc<Mutex<Vec<AlertDetection>>> reference to the detections vector.
pub fn clear_detections(detections: &Arc<Mutex<Vec<AlertDetection>>>) {
    if let Ok(mut vec) = detections.lock() {
        vec.clear();
    }
    let _ = std::fs::remove_file(detections_path());
}

/// Adds a detection to the detections vector and saves it to disk.
///
/// # Arguments
/// * `detections` - Arc<Mutex<Vec<AlertDetection>>> reference to the detections vector.
/// * `detection` - The AlertDetection to add.
pub fn add_detection(detections: &Arc<Mutex<Vec<AlertDetection>>>, detection: AlertDetection) {
    if let Ok(mut vec) = detections.lock() {
        vec.push(detection.clone());
        let _ = save_detections(&vec);
    }
}
