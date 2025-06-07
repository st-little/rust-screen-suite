use dioxus::prelude::*;
use dioxus_i18n::t;

/// Props for the SettingsGeneralTab component.
/// Contains signals for language, theme, and timezone settings.
#[derive(PartialEq, Props, Clone)]
pub struct SettingsGeneralTabProps {
    pub language: Signal<String>,
    pub theme: Signal<String>,
    pub timezone: Signal<String>,
}

/// SettingsGeneralTab displays general settings such as language, timezone, and theme.
///
/// # Props
/// - `language`: Signal for the selected language.
/// - `theme`: Signal for the selected theme.
/// - `timezone`: Signal for the selected timezone.
///
/// This component provides dropdowns for language, timezone, and theme selection.
#[component]
pub fn SettingsGeneralTab(props: SettingsGeneralTabProps) -> Element {
    let SettingsGeneralTabProps {
        mut language,
        mut theme,
        mut timezone,
    } = props;

    rsx! {
        div {
            h3 { class: "text-xl font-bold mb-4", {t!("setting_tab_general")} }
            // Language selection
            div { class: "flex items-center gap-4 mb-4",
                label { class: "w-32 text-right font-medium", {t!("setting_language")} }
                select {
                    class: "select select-bordered w-64 text-gray-900",
                    value: language(),
                    onchange: move |evt| {
                        language.set(evt.value());
                    },
                    option { value: "en-US", "English (United States)" }
                    option { value: "en-GB", "English (United Kingdom)" }
                    option { value: "ja-JP", "日本語 (日本)" }
                    option { value: "zh-CN", "简体中文 (中国)" }
                    option { value: "zh-TW", "繁體中文 (台灣)" }
                    option { value: "ko-KR", "한국어 (대한민국)" }
                    option { value: "fr-FR", "Français (France)" }
                    option { value: "de-DE", "Deutsch (Deutschland)" }
                    option { value: "ru-RU", "Русский (Россия)" }
                    option { value: "th-TH", "ไทย (ประเทศไทย)" }
                    option { value: "pt-BR", "Português (Brasil)" }
                }
            }
            // Timezone selection
            div { class: "flex items-center gap-4 mb-4",
                label { class: "w-32 text-right font-medium", {t!("setting_timezone")} }
                select {
                    class: "select select-bordered w-64 text-gray-900",
                    value: timezone(),
                    onchange: move |evt| {
                        timezone.set(evt.value());
                    },
                    option { value: "Asia/Tokyo", {t!("setting_timezone_Asia_Tokyo")} }
                    option { value: "Asia/Shanghai", {t!("setting_timezone_Asia_Shanghai")} }
                    option { value: "Asia/Seoul", {t!("setting_timezone_Asia_Seoul")} }
                    option { value: "Asia/Singapore", {t!("setting_timezone_Asia_Singapore")} }
                    option { value: "Asia/Bangkok", {t!("setting_timezone_Asia_Bangkok")} }
                    option { value: "Europe/London", {t!("setting_timezone_Europe_London")} }
                    option { value: "Europe/Paris", {t!("setting_timezone_Europe_Paris")} }
                    option { value: "Europe/Berlin", {t!("setting_timezone_Europe_Berlin")} }
                    option { value: "Europe/Moscow", {t!("setting_timezone_Europe_Moscow")} }
                    option { value: "America/New_York", {t!("setting_timezone_America_New_York")} }
                    option { value: "America/Chicago", {t!("setting_timezone_America_Chicago")} }
                    option { value: "America/Denver", {t!("setting_timezone_America_Denver")} }
                    option { value: "America/Los_Angeles", {t!("setting_timezone_America_Los_Angeles")} }
                    option { value: "America/Sao_Paulo", {t!("setting_timezone_America_Sao_Paulo")} }
                    option { value: "Australia/Sydney", {t!("setting_timezone_Australia_Sydney")} }
                    option { value: "Pacific/Auckland", {t!("setting_timezone_Pacific_Auckland")} }
                    option { value: "UTC", {t!("setting_timezone_UTC")} }
                }
            }
            // Theme selection
            div { class: "flex items-center gap-4 mb-4",
                label { class: "w-32 text-right font-medium", {t!("setting_theme")} }
                select {
                    class: "select select-bordered w-64 text-gray-900",
                    value: theme(),
                    onchange: move |evt| {
                        theme.set(evt.value());
                    },
                    option { value: "dark", {t!("setting_dark")} }
                }
            }
        }
    }
}
