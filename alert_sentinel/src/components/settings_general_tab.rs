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
                    class: "select select-bordered w-64",
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
                    class: "select select-bordered w-64",
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
                    class: "select select-bordered w-64",
                    value: theme(),
                    onchange: move |evt| {
                        theme.set(evt.value());
                    },
                    option { value: "light", {t!("setting_light")} }
                    option { value: "dark", {t!("setting_dark")} }
                    option { value: "cupcake", {t!("setting_cupcake")} }
                    option { value: "bumblebee", {t!("setting_bumblebee")} }
                    option { value: "emerald", {t!("setting_emerald")} }
                    option { value: "corporate", {t!("setting_corporate")} }
                    option { value: "synthwave", {t!("setting_synthwave")} }
                    option { value: "retro", {t!("setting_retro")} }
                    option { value: "cyberpunk", {t!("setting_cyberpunk")} }
                    option { value: "valentine", {t!("setting_valentine")} }
                    option { value: "halloween", {t!("setting_halloween")} }
                    option { value: "garden", {t!("setting_garden")} }
                    option { value: "forest", {t!("setting_forest")} }
                    option { value: "aqua", {t!("setting_aqua")} }
                    option { value: "lofi", {t!("setting_lofi")} }
                    option { value: "pastel", {t!("setting_pastel")} }
                    option { value: "fantasy", {t!("setting_fantasy")} }
                    option { value: "wireframe", {t!("setting_wireframe")} }
                    option { value: "black", {t!("setting_black")} }
                    option { value: "luxury", {t!("setting_luxury")} }
                    option { value: "dracula", {t!("setting_dracula")} }
                    option { value: "cmyk", {t!("setting_cmyk")} }
                    option { value: "autumn", {t!("setting_autumn")} }
                    option { value: "business", {t!("setting_business")} }
                    option { value: "acid", {t!("setting_acid")} }
                    option { value: "lemonade", {t!("setting_lemonade")} }
                    option { value: "night", {t!("setting_night")} }
                    option { value: "coffee", {t!("setting_coffee")} }
                    option { value: "winter", {t!("setting_winter")} }
                    option { value: "dim", {t!("setting_dim")} }
                    option { value: "nord", {t!("setting_nord")} }
                    option { value: "sunset", {t!("setting_sunset")} }
                    option { value: "caramellatte", {t!("setting_caramellatte")} }
                    option { value: "abyss", {t!("setting_abyss")} }
                    option { value: "silk", {t!("setting_silk")} }
                }
            }
        }
    }
}
