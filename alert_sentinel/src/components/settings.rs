use dioxus::prelude::*;
use dioxus_i18n::t;
use std::collections::VecDeque;

use crate::{
    components::{
        settings_alerts_tab::SettingsAlertsTab, settings_footer::SettingsFooter,
        settings_general_tab::SettingsGeneralTab, settings_header::SettingsHeader,
        settings_license_tab::SettingsLicenseTab, settings_monitor_tab::SettingsMonitorTab,
        settings_notification_tab::SettingsNotificationTab,
        settings_preview_modal::SettingsPreviewModal, settings_tab_button::SettingsTabButton,
        settings_templates_tab::SettingsTemplatesTab,
    },
    types::settings::AppConfig,
    Toast,
};

/// Settings component displays the settings modal, including all tabs and their contents.
///
/// # Props
/// - `config`: Application configuration signal.
/// - `show_settings`: Signal to control the visibility of the settings modal.
/// - `toast_queue`: Signal for the toast notification queue.
///
/// This component manages tab switching, state propagation, and renders each settings tab and footer.
#[component]
pub fn Settings(
    config: Signal<AppConfig>,
    show_settings: Signal<bool>,
    toast_queue: Signal<VecDeque<Toast>>,
) -> Element {
    // Signals for general settings
    let mut language = use_signal(|| config.read().language.clone());
    let mut theme = use_signal(|| config.read().theme.clone());
    let mut timezone = use_signal(|| config.read().timezone.clone());

    // Signals for monitor settings
    let mut monitor_interval = use_signal(|| config.read().monitor_interval_ms);
    let mut monitor_skip_duration = use_signal(|| config.read().monitor_skip_duration_ms);
    let mut monitor_threshold = use_signal(|| config.read().monitor_threshold);
    let mut monitor_scale = use_signal(|| config.read().monitor_scale);

    // Signals for template image paths
    let mut airdrop_path =
        use_signal(|| config.read().templates.airdrop.clone().unwrap_or_default());
    let mut heli_path = use_signal(|| config.read().templates.heli.clone().unwrap_or_default());
    let mut ch47_path = use_signal(|| config.read().templates.ch47.clone().unwrap_or_default());
    let mut cargo_path = use_signal(|| config.read().templates.cargo.clone().unwrap_or_default());
    let mut shovel_path = use_signal(|| config.read().templates.shovel.clone().unwrap_or_default());
    let mut oil_rig_large_path = use_signal(|| {
        config
            .read()
            .templates
            .oil_rig_large
            .clone()
            .unwrap_or_default()
    });
    let mut oil_rig_small_path = use_signal(|| {
        config
            .read()
            .templates
            .oil_rig_small
            .clone()
            .unwrap_or_default()
    });

    // Signals for alert toggles
    let mut airdrop_toggle = use_signal(|| config.read().toggles.airdrop);
    let mut heli_toggle = use_signal(|| config.read().toggles.heli);
    let mut ch47_toggle = use_signal(|| config.read().toggles.ch47);
    let mut cargo_toggle = use_signal(|| config.read().toggles.cargo);
    let mut shovel_toggle = use_signal(|| config.read().toggles.shovel);
    let mut oil_rig_large_toggle = use_signal(|| config.read().toggles.oil_rig_large);
    let mut oil_rig_small_toggle = use_signal(|| config.read().toggles.oil_rig_small);

    // Signals for notification settings
    let mut sound_enabled = use_signal(|| config.read().sound_enabled);
    let mut sound_volume = use_signal(|| config.read().sound_volume);

    let mut discord_enabled = use_signal(|| config.read().discord_enabled);
    let mut discord_webhook =
        use_signal(|| config.read().discord_webhook.clone().unwrap_or_default());
    let mut discord_username =
        use_signal(|| config.read().discord_username.clone().unwrap_or_default());
    let mut discord_avatar_url =
        use_signal(|| config.read().discord_avatar_url.clone().unwrap_or_default());

    // Signal for currently active tab
    let active_tab = use_signal(|| "general".to_string());
    // Signal for preview modal visibility and image path
    let show_preview_modal = use_signal(|| false);
    let preview_image_path = use_signal(String::new);

    // Function to reset all settings signals from the config
    let reset_signals_from_config = move || {
        let cfg = config.read();
        language.set(cfg.language.clone());
        theme.set(cfg.theme.clone());
        timezone.set(cfg.timezone.clone());
        monitor_interval.set(cfg.monitor_interval_ms);
        monitor_skip_duration.set(cfg.monitor_skip_duration_ms);
        monitor_threshold.set(cfg.monitor_threshold);
        monitor_scale.set(cfg.monitor_scale);
        airdrop_path.set(cfg.templates.airdrop.clone().unwrap_or_default());
        heli_path.set(cfg.templates.heli.clone().unwrap_or_default());
        ch47_path.set(cfg.templates.ch47.clone().unwrap_or_default());
        cargo_path.set(cfg.templates.cargo.clone().unwrap_or_default());
        shovel_path.set(cfg.templates.shovel.clone().unwrap_or_default());
        oil_rig_large_path.set(cfg.templates.oil_rig_large.clone().unwrap_or_default());
        oil_rig_small_path.set(cfg.templates.oil_rig_small.clone().unwrap_or_default());
        airdrop_toggle.set(cfg.toggles.airdrop);
        heli_toggle.set(cfg.toggles.heli);
        ch47_toggle.set(cfg.toggles.ch47);
        cargo_toggle.set(cfg.toggles.cargo);
        shovel_toggle.set(cfg.toggles.shovel);
        oil_rig_large_toggle.set(cfg.toggles.oil_rig_large);
        oil_rig_small_toggle.set(cfg.toggles.oil_rig_small);
        sound_enabled.set(cfg.sound_enabled);
        sound_volume.set(cfg.sound_volume);
        discord_enabled.set(cfg.discord_enabled);
        discord_webhook.set(cfg.discord_webhook.clone().unwrap_or_default());
        discord_username.set(cfg.discord_username.clone().unwrap_or_default());
        discord_avatar_url.set(cfg.discord_avatar_url.clone().unwrap_or_default());
    };

    rsx! {
        div { class: "flex flex-col h-full",
            SettingsHeader { show_settings, on_close: reset_signals_from_config }

            div { class: "flex flex-1 overflow-hidden",
                aside { class: "w-48 p-4 space-y-2",
                    // Tab buttons for each settings section
                    SettingsTabButton {
                        label: t!("setting_tab_general"),
                        tab: "general".to_string(),
                        active_tab,
                    }
                    SettingsTabButton {
                        label: t!("setting_tab_templates"),
                        tab: "templates".to_string(),
                        active_tab,
                    }
                    SettingsTabButton {
                        label: t!("setting_tab_alerts"),
                        tab: "alerts".to_string(),
                        active_tab,
                    }
                    SettingsTabButton {
                        label: t!("setting_tab_monitor"),
                        tab: "monitor".to_string(),
                        active_tab,
                    }
                    SettingsTabButton {
                        label: t!("setting_tab_notification"),
                        tab: "notification".to_string(),
                        active_tab,
                    }
                    SettingsTabButton {
                        label: t!("setting_tab_license"),
                        tab: "license".to_string(),
                        active_tab,
                    }
                }
                main { class: "flex-1 overflow-y-auto p-6 space-y-6",
                    // Render the selected tab's content
                    match active_tab().as_str() {
                        "general" => rsx! {
                            SettingsGeneralTab { language, theme, timezone }
                        },
                        "templates" => rsx! {
                            SettingsTemplatesTab {
                                preview_image_path,
                                show_preview_modal,
                                airdrop_path,
                                heli_path,
                                ch47_path,
                                cargo_path,
                                shovel_path,
                                oil_rig_large_path,
                                oil_rig_small_path,
                            }
                        },
                        "alerts" => rsx! {
                            SettingsAlertsTab {
                                airdrop_toggle,
                                heli_toggle,
                                ch47_toggle,
                                cargo_toggle,
                                shovel_toggle,
                                oil_rig_large_toggle,
                                oil_rig_small_toggle,
                            }
                        },
                        "monitor" => rsx! {
                            SettingsMonitorTab {
                                monitor_interval,
                                monitor_skip_duration,
                                monitor_threshold,
                                monitor_scale,
                            }
                        },
                        "notification" => rsx! {
                            SettingsNotificationTab {
                                config,
                                sound_enabled,
                                sound_volume,
                                discord_enabled,
                                discord_webhook,
                                discord_username,
                                discord_avatar_url,
                            }
                        },
                        "license" => rsx! {
                            SettingsLicenseTab {}
                        },
                        _ => rsx! {
                            div { {t!("setting_undefined_tab")} }
                        },
                    }
                }
            }
            // Settings footer with Save and Cancel actions
            SettingsFooter {
                config,
                show_settings,
                on_cancel: reset_signals_from_config,
                toast_queue,
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
            }
        }
        // Modal for previewing template images
        SettingsPreviewModal { show: show_preview_modal, image_path: preview_image_path }
    }
}
