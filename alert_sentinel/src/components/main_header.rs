use dioxus::prelude::*;
use dioxus_i18n::t;
use std::{env, rc::Rc};

use crate::{services::main_header::get_help_file_path, types::settings::AppConfig};

/// MainHeader component displays the application header bar with controls.
///
/// # Props
/// - `config`: Application configuration signal.
/// - `show_notification`: Signal to control notification modal visibility.
/// - `show_settings`: Signal to control settings modal visibility.
/// - `show_history`: Signal to control history modal visibility.
///
/// The header includes the app name, settings/help buttons, and window controls (minimize, maximize, close).
#[component]
pub fn MainHeader(
    config: Signal<AppConfig>,
    show_notification: Signal<bool>,
    show_settings: Signal<bool>,
    show_history: Signal<bool>,
) -> Element {
    let window = dioxus::desktop::use_window();

    rsx! {
        header { class: "navbar bg-blue-950 text-primary-content select-none px-4",
            div {
                class: "navbar-start gap-2",
                // Enable window dragging by mouse down on the header
                onmousedown: {
                    let window = Rc::clone(&window);
                    move |_| window.drag()
                },
                span { class: "text-lg font-bold", {t!("header_app_name")} }
            }
            div { class: "navbar-end gap-2",
                // Settings button
                button {
                    class: "btn btn-ghost btn-circle tooltip tooltip-bottom",
                    "data-tip": t!("header_open_settings"),
                    onclick: move |_| {
                        show_notification.set(false);
                        show_settings.set(!show_settings());
                        show_history.set(false);
                    },
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        width: "16",
                        height: "16",
                        view_box: "0 0 24 24",
                        path {
                            fill: "currentColor",
                            fill_rule: "evenodd",
                            d: "m19.85 8.75l4.15.83v4.84l-4.15.83l2.35 3.52l-3.43 3.43l-3.52-2.35l-.83 4.15H9.58l-.83-4.15l-3.52 2.35l-3.43-3.43l2.35-3.52L0 14.42V9.58l4.15-.83L1.8 5.23L5.23 1.8l3.52 2.35L9.58 0h4.84l.83 4.15l3.52-2.35l3.43 3.43zm-1.57 5.07l4-.81v-2l-4-.81l-.54-1.3l2.29-3.43l-1.43-1.43l-3.43 2.29l-1.3-.54l-.81-4h-2l-.81 4l-1.3.54l-3.43-2.29l-1.43 1.43L6.38 8.9l-.54 1.3l-4 .81v2l4 .81l.54 1.3l-2.29 3.43l1.43 1.43l3.43-2.29l1.3.54l.81 4h2l.81-4l1.3-.54l3.43 2.29l1.43-1.43l-2.29-3.43zm-8.186-4.672A3.43 3.43 0 0 1 12 8.57A3.44 3.44 0 0 1 15.43 12a3.43 3.43 0 1 1-5.336-2.852m.956 4.274c.281.188.612.288.95.288A1.7 1.7 0 0 0 13.71 12a1.71 1.71 0 1 0-2.66 1.422",
                            clip_rule: "evenodd",
                        }
                    }
                }
                // Help button
                button {
                    class: "btn btn-ghost btn-circle mr-8 tooltip tooltip-bottom",
                    "data-tip": t!("header_open_help"),
                    onclick: move |_| {
                        if let Ok(exe_path) = env::current_exe() {
                            let path = get_help_file_path(&exe_path, &config().language);
                            let _ = webbrowser::open(&path.to_string_lossy());
                        }
                    },
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        width: "22",
                        height: "22",
                        view_box: "0 0 16 16",
                        path {
                            fill: "currentColor",
                            fill_rule: "evenodd",
                            d: "M7.5 1a6.5 6.5 0 1 0 0 13a6.5 6.5 0 0 0 0-13m0 12a5.5 5.5 0 1 1 0-11a5.5 5.5 0 0 1 0 11m1.55-8.42a1.8 1.8 0 0 0-.61-.42A2.25 2.25 0 0 0 7.53 4a2.2 2.2 0 0 0-.88.17c-.239.1-.45.254-.62.45a1.9 1.9 0 0 0-.38.62a3 3 0 0 0-.15.72h1.23a.84.84 0 0 1 .506-.741a.7.7 0 0 1 .304-.049a.9.9 0 0 1 .27 0a.6.6 0 0 1 .22.14a.6.6 0 0 1 .16.22a.7.7 0 0 1 .06.3c0 .173-.037.343-.11.5a2.4 2.4 0 0 1-.27.46l-.35.42c-.12.13-.24.27-.35.41a2.3 2.3 0 0 0-.27.45a1.2 1.2 0 0 0-.1.5v.66H8v-.49a.94.94 0 0 1 .11-.42a3 3 0 0 1 .28-.41l.36-.44a4 4 0 0 0 .36-.48a2.6 2.6 0 0 0 .28-.55a1.9 1.9 0 0 0 .11-.64a2.2 2.2 0 0 0-.1-.67a1.5 1.5 0 0 0-.35-.55M6.8 9.83h1.17V11H6.8z",
                            clip_rule: "evenodd",
                        }
                    }
                }
                // Minimize button
                button {
                    class: "btn btn-ghost",
                    onclick: {
                        let window = Rc::clone(&window);
                        move |_| {
                            window.set_minimized(true);
                        }
                    },
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        width: "16",
                        height: "16",
                        view_box: "0 0 16 16",
                        path { fill: "currentColor", d: "M14 8v1H3V8z" }
                    }
                }
                // Maximize/Restore button
                button {
                    class: "btn btn-ghost",
                    onclick: {
                        let window = Rc::clone(&window);
                        move |_| {
                            window.toggle_maximized();
                        }
                    },
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        width: "16",
                        height: "16",
                        view_box: "0 0 16 16",
                        g { fill: "currentColor",
                            path { d: "M3 5v9h9V5zm8 8H4V6h7z" }
                            path {
                                fill_rule: "evenodd",
                                d: "M5 5h1V4h7v7h-1v1h2V3H5z",
                                clip_rule: "evenodd",
                            }
                        }
                    }
                }
                // Close button
                button {
                    class: "btn btn-ghost",
                    onclick: {
                        let window = Rc::clone(&window);
                        move |_| {
                            window.close();
                        }
                    },
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
}
