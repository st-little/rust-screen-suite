use dioxus::prelude::*;
use dioxus_i18n::t;

/// SettingsHeader component displays the header of the settings modal,
/// including the title and a close button.
///
/// # Props
/// - `show_settings`: Signal to control the visibility of the settings modal.
#[component]
pub fn SettingsHeader(show_settings: Signal<bool>) -> Element {
    rsx! {
        header { class: "flex justify-between items-center px-6 py-4 border-b border-gray-700",
            h2 { class: "text-lg font-bold", {t!("setting_settings")} }
            // Close button for the settings modal
            button {
                class: "btn btn-sm btn-ghost",
                onclick: move |_| show_settings.set(false),
                svg {
                    xmlns: "http://www.w3.org/2000/svg",
                    width: "16",
                    height: "16",
                    view_box: "0 0 16 16",
                    path {
                        fill: "currentColor",
                        fill_rule: "evenodd",
                        d: "m7.116 8l-4.558 4.558l.884.884L8 8.884l4.558 4.558l.884-.884L8.884 8l4.558-4.558l-.884-.884L8 7.116L3.442 2.558l-.884.884z",
                        clip_rule: "evenodd",
                    }
                }
            }
        }
    }
}
