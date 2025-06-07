use dioxus::prelude::*;
use dioxus_i18n::t;

use crate::types::settings::AppConfig;

/// Props for the SettingsNotificationTab component.
/// Contains signals for sound and Discord notification settings.
#[derive(PartialEq, Props, Clone)]
pub struct SettingsNotificationTabProps {
    pub config: Signal<AppConfig>,
    pub sound_enabled: Signal<bool>,
    pub sound_volume: Signal<u8>,
    pub discord_enabled: Signal<bool>,
    pub discord_webhook: Signal<String>,
    pub discord_username: Signal<String>,
    pub discord_avatar_url: Signal<String>,
}

/// SettingsNotificationTab displays notification-related settings such as sound and Discord integration.
///
/// # Props
/// - `config`: Application configuration signal.
/// - `sound_enabled`: Signal for enabling/disabling sound notifications.
/// - `sound_volume`: Signal for sound volume.
/// - `discord_enabled`: Signal for enabling/disabling Discord notifications.
/// - `discord_webhook`: Signal for Discord webhook URL.
/// - `discord_username`: Signal for Discord username.
/// - `discord_avatar_url`: Signal for Discord avatar URL.
///
/// This component provides toggles and input fields for notification settings.
#[component]
pub fn SettingsNotificationTab(props: SettingsNotificationTabProps) -> Element {
    let SettingsNotificationTabProps {
        config,
        mut sound_enabled,
        mut sound_volume,
        mut discord_enabled,
        mut discord_webhook,
        mut discord_username,
        mut discord_avatar_url,
    } = props;

    rsx! {
        div {
            h3 { class: "text-xl font-bold mb-4", {t!("setting_tab_notification")} }
            p { class: "opacity-60 mb-4", {t!("setting_desc_notification")} }
            div { class: "space-y-4",
                div { class: "divider divider-info", {t!("setting_notification_sound")} }
                // Toggle for enabling/disabling sound notifications
                div { class: "flex items-center gap-4",
                    label { class: "w-48 text-right font-medium", {t!("setting_notification_sound")} }
                    input {
                        r#type: "checkbox",
                        class: "toggle border-neutral-400 bg-neutral-300 checked:text-blue-500",
                        checked: sound_enabled(),
                        onchange: move |e| {
                            sound_enabled.set(e.checked());
                        },
                    }
                }
                // Slider for sound volume
                div { class: "flex items-center gap-4",
                    label { class: "w-48 text-right font-medium",
                        {t!("setting_notification_volume", value : config.read().sound_volume)}
                    }
                    input {
                        r#type: "range",
                        min: "0",
                        max: "100",
                        value: sound_volume.to_string(),
                        class: "range w-64",
                        onchange: move |e| {
                            if let Ok(volume) = e.value().parse::<u8>() {
                                sound_volume.set(volume);
                            }
                        },
                    }
                }
            }
            div { class: "space-y-4",
                div { class: "divider divider-info", {t!("setting_notification_discord")} }
                // Toggle for enabling/disabling Discord notifications
                div { class: "flex items-center gap-4",
                    label { class: "w-48 text-right font-medium", {t!("setting_notification_discord")} }
                    input {
                        r#type: "checkbox",
                        class: "toggle bg-neutral-300 checked:text-blue-500",
                        checked: discord_enabled(),
                        onchange: move |e| {
                            discord_enabled.set(e.checked());
                        },
                    }
                }
                // Input for Discord webhook URL
                div { class: "flex items-center gap-4",
                    label { class: "w-48 text-right font-medium",
                        {t!("setting_notification_webhook_url")}
                    }
                    input {
                        r#type: "text",
                        class: "input input-bordered w-full max-w-md text-gray-900",
                        placeholder: t!("setting_placeholder_discord_webhook"),
                        value: discord_webhook(),
                        onchange: move |e| {
                            discord_webhook.set(e.value().clone());
                        },
                    }
                }
                // Input for Discord username
                div { class: "flex items-center gap-4",
                    label { class: "w-48 text-right font-medium", {t!("setting_notification_username")} }
                    input {
                        r#type: "text",
                        class: "input input-bordered w-full max-w-md text-gray-900",
                        placeholder: t!("setting_placeholder_discord_username"),
                        value: discord_username(),
                        onchange: move |e| {
                            discord_username.set(e.value().clone());
                        },
                    }
                }
                // Input for Discord avatar URL
                div { class: "flex items-center gap-4",
                    label { class: "w-48 text-right font-medium",
                        {t!("setting_notification_avatar_url")}
                    }
                    input {
                        r#type: "text",
                        class: "input input-bordered w-full max-w-md text-gray-900",
                        placeholder: t!("setting_placeholder_discord_avatar_url"),
                        value: discord_avatar_url(),
                        onchange: move |e| {
                            discord_avatar_url.set(e.value().clone());
                        },
                    }
                }
            }
        }
    }
}
