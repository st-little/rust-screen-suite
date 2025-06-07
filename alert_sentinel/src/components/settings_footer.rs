use crate::{
    services::settings_footer::{save_all_templates, save_config},
    types::{
        settings::AppConfig,
        toast::{Toast, ToastKind},
    },
};
use dioxus::{logger::tracing::debug, prelude::*};
use dioxus_i18n::t;
use std::{collections::VecDeque, time::Instant};

/// Props for the SettingsFooter component.
/// Contains all signals and state needed to save or cancel settings changes.
#[derive(PartialEq, Props, Clone)]
pub struct SettingsFooterProps {
    pub config: Signal<AppConfig>,
    pub show_settings: Signal<bool>,
    pub toast_queue: Signal<VecDeque<Toast>>,
    pub language: Signal<String>,
    pub theme: Signal<String>,
    pub timezone: Signal<String>,
    pub airdrop_path: Signal<String>,
    pub heli_path: Signal<String>,
    pub ch47_path: Signal<String>,
    pub cargo_path: Signal<String>,
    pub shovel_path: Signal<String>,
    pub oil_rig_large_path: Signal<String>,
    pub oil_rig_small_path: Signal<String>,
    pub airdrop_toggle: Signal<bool>,
    pub heli_toggle: Signal<bool>,
    pub ch47_toggle: Signal<bool>,
    pub cargo_toggle: Signal<bool>,
    pub shovel_toggle: Signal<bool>,
    pub oil_rig_large_toggle: Signal<bool>,
    pub oil_rig_small_toggle: Signal<bool>,
    pub monitor_interval: Signal<u64>,
    pub monitor_skip_duration: Signal<u64>,
    pub monitor_threshold: Signal<f32>,
    pub monitor_scale: Signal<f32>,
    pub sound_enabled: Signal<bool>,
    pub sound_volume: Signal<u8>,
    pub discord_enabled: Signal<bool>,
    pub discord_webhook: Signal<String>,
    pub discord_username: Signal<String>,
    pub discord_avatar_url: Signal<String>,
}

/// SettingsFooter component displays the footer of the settings modal,
/// providing "Cancel" and "Save" buttons to discard or persist changes.
///
/// - On "Save", all settings and template paths are updated and persisted.
/// - On error, an error toast is shown; on success, a success toast is shown.
#[component]
pub fn SettingsFooter(props: SettingsFooterProps) -> Element {
    let SettingsFooterProps {
        mut config,
        mut show_settings,
        mut toast_queue,
        language,
        theme,
        timezone,
        airdrop_path,
        heli_path,
        ch47_path,
        cargo_path,
        shovel_path,
        oil_rig_large_path,
        oil_rig_small_path,
        airdrop_toggle,
        heli_toggle,
        ch47_toggle,
        cargo_toggle,
        shovel_toggle,
        oil_rig_large_toggle,
        oil_rig_small_toggle,
        monitor_interval,
        monitor_skip_duration,
        monitor_threshold,
        monitor_scale,
        sound_enabled,
        sound_volume,
        discord_enabled,
        discord_webhook,
        discord_username,
        discord_avatar_url,
    } = props;

    rsx! {
        footer { class: "flex justify-end gap-4 px-6 py-4 border-t border-gray-700",
            // Cancel button: closes the settings modal without saving
            button { class: "btn", onclick: move |_| show_settings.set(false), {t!("setting_cancel")} }
            // Save button: persists all settings and shows a toast on success or error
            button {
                class: "btn btn-primary",
                onclick: move |_| {
                    config
                        .with_mut(|cfg| {
                            cfg.language = language();
                            cfg.theme = theme();
                            cfg.timezone = timezone();
                        });
                    let template_paths = [
                        ("airdrop", &airdrop_path),
                        ("heli", &heli_path),
                        ("ch47", &ch47_path),
                        ("cargo", &cargo_path),
                        ("shovel", &shovel_path),
                        ("oil_rig_large", &oil_rig_large_path),
                        ("oil_rig_small", &oil_rig_small_path),
                    ];
                    save_all_templates(config, &template_paths);
                    config
                        .with_mut(|cfg| {
                            cfg.toggles.airdrop = airdrop_toggle();
                            cfg.toggles.heli = heli_toggle();
                            cfg.toggles.ch47 = ch47_toggle();
                            cfg.toggles.cargo = cargo_toggle();
                            cfg.toggles.shovel = shovel_toggle();
                            cfg.toggles.oil_rig_large = oil_rig_large_toggle();
                            cfg.toggles.oil_rig_small = oil_rig_small_toggle();
                        });
                    config
                        .with_mut(|cfg| {
                            cfg.monitor_interval_ms = monitor_interval();
                            cfg.monitor_skip_duration_ms = monitor_skip_duration();
                            cfg.monitor_threshold = monitor_threshold();
                            cfg.monitor_scale = monitor_scale();
                        });
                    config
                        .with_mut(|cfg| {
                            cfg.sound_enabled = sound_enabled();
                            cfg.sound_volume = sound_volume();
                        });
                    config
                        .with_mut(|cfg| {
                            cfg.discord_enabled = discord_enabled();
                            cfg.discord_webhook = Some(discord_webhook().to_string());
                            cfg.discord_username = Some(discord_username().to_string());
                            cfg.discord_avatar_url = Some(discord_avatar_url().to_string());
                        });
                    let cfg = config.read();
                    if let Err(err) = save_config(&cfg) {
                        debug!("Failed to save settings: {err}");
                        toast_queue
                            .with_mut(|q| {
                                q.push_back(Toast {
                                    kind: ToastKind::Error,
                                    message: t!("setting_msg_save_error"),
                                    duration_ms: 2000,
                                    created: Instant::now(),
                                });
                            });
                    } else {
                        toast_queue
                            .with_mut(|q| {
                                q.push_back(Toast {
                                    kind: ToastKind::Success,
                                    message: t!("setting_msg_save_success"),
                                    duration_ms: 2000,
                                    created: Instant::now(),
                                });
                            });
                    }
                },
                {t!("setting_save")}
            }
        }
    }
}
