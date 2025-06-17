use dioxus::desktop::{Config, WindowBuilder};
use dioxus::prelude::*;
use dioxus_i18n::prelude::*;
use std::collections::VecDeque;
use unic_langid::langid;

use crate::components::alert::Alert;
use crate::components::main_footer::MainFooter;
use crate::components::main_header::MainHeader;
use crate::components::settings::Settings;
use crate::components::toasts::Toasts;
use crate::services::settings::load_config;
use crate::types::toast::Toast;

mod components;
mod services;
mod types;

const DAISYUI_CSS: Asset = asset!("/assets/styling/daisyui.css");
const TAILWIND_CSS: Asset = asset!("/assets/styling/tailwind.css");

/// Entry point for the Alert Sentinel application.
fn main() {
    dioxus::LaunchBuilder::desktop()
        .with_cfg(
            Config::new().with_window(
                WindowBuilder::new()
                    .with_resizable(true)
                    .with_decorations(false)
                    .with_title("Alert Sentinel"),
            ),
        )
        .launch(App)
}

/// Main application component for Alert Sentinel.
/// Handles initialization, localization, and layout of the main UI.
#[component]
fn App() -> Element {
    let running = use_signal(|| false);
    let show_notification = use_signal(|| false);
    let show_settings = use_signal(|| false);
    let show_history = use_signal(|| false);
    let config = use_signal(load_config);
    let toast_queue = use_signal(VecDeque::<Toast>::new);

    // Initialize i18n with supported locales
    use_init_i18n(|| {
        I18nConfig::new(langid!("en-US"))
            .with_locale((langid!("de-DE"), include_str!("../public/i18n/de-DE.ftl")))
            .with_locale((langid!("en-GB"), include_str!("../public/i18n/en-GB.ftl")))
            .with_locale((langid!("en-US"), include_str!("../public/i18n/en-US.ftl")))
            .with_locale((langid!("fr-FR"), include_str!("../public/i18n/fr-FR.ftl")))
            .with_locale((langid!("ja-JP"), include_str!("../public/i18n/ja-JP.ftl")))
            .with_locale((langid!("ko-KR"), include_str!("../public/i18n/ko-KR.ftl")))
            .with_locale((langid!("pt-BR"), include_str!("../public/i18n/pt-BR.ftl")))
            .with_locale((langid!("ru-RU"), include_str!("../public/i18n/ru-RU.ftl")))
            .with_locale((langid!("th-TH"), include_str!("../public/i18n/th-TH.ftl")))
            .with_locale((langid!("zh-CN"), include_str!("../public/i18n/zh-CN.ftl")))
            .with_locale((langid!("zh-TW"), include_str!("../public/i18n/zh-TW.ftl")))
    });

    // Set the language for i18n based on the config
    let mut i18n = i18n();
    use_effect(move || {
        let lang = config.read().language.parse().unwrap_or(langid!("en-US"));
        i18n.set_language(lang);
    });

    // CSS class for the settings modal (handles open/close animation)
    let settings_class = format!(
        "absolute top-0 left-0 w-full h-full bg-base-100 p-6 z-40 \
        transition-all duration-300 origin-right {}",
        if show_settings() {
            "opacity-100 scale-x-100 pointer-events-auto"
        } else {
            "opacity-0 scale-x-0 pointer-events-none"
        }
    );

    rsx! {
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        document::Link { rel: "stylesheet", href: DAISYUI_CSS }

        div {
            class: "flex flex-col min-h-screen",
            "data-theme": config.read().theme.as_str(),
            MainHeader {
                config,
                show_notification,
                show_settings,
                show_history,
            }
            main { class: "relative flex-1 bg-base-100 p-6 flex flex-col gap-4",
                Alert { running, config, toast_queue }
                div { class: settings_class,
                    Settings { config, show_settings, toast_queue }
                }
            }
            MainFooter { running }
        }
        Toasts { toast_queue }
    }
}
