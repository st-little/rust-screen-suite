use dioxus::prelude::*;

/// SettingsTabButton renders a button for switching between settings tabs.
///
/// # Props
/// - `label`: The label to display on the button.
/// - `tab`: The tab identifier associated with this button.
/// - `active_tab`: Signal for the currently active tab.
///
/// When clicked, this button sets the active tab to its own tab identifier.
#[component]
pub fn SettingsTabButton(label: String, tab: String, active_tab: Signal<String>) -> Element {
    rsx! {
        button {
            class: if active_tab() == tab { "btn btn-ghost btn-wide btn-active text-left" } else { "btn btn-ghost btn-wide text-left" },
            onclick: move |_| active_tab.set(tab.clone()),
            {label}
        }
    }
}
