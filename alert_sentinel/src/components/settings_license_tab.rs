use dioxus::prelude::*;
use dioxus_i18n::t;
use std::env;

use crate::services::settings::get_license_file_path;

/// SettingsLicenseTab component displays the license information tab in the settings modal.
///
/// This tab provides buttons to open license files for Rust dependencies, npm modules, and other third-party software.
/// Each button opens the corresponding license file in the default browser.
///
/// # License Files
/// - Rust dependencies: rust-licenses.html
/// - npm modules: npm-licenses.md
/// - Other licenses: other-licenses.html
#[component]
pub fn SettingsLicenseTab() -> Element {
    rsx! {
        div {
            h3 { class: "text-xl font-bold mb-4", {t!("setting_tab_license")} }
            p { class: "opacity-60", {t!("setting_desc_license")} }
            ul { class: "list-disc ml-6 mt-4 space-y-2",
                // Button to open Rust dependencies license file
                li {
                    button {
                        class: "link link-info text-left inline-flex items-center gap-1",
                        onclick: move |_| {
                            if let Ok(exe_path) = env::current_exe() {
                                let license_path = get_license_file_path(&exe_path, "rust-licenses.html");
                                let _ = webbrowser::open(license_path.to_string_lossy().as_ref());
                            }
                        },
                        {t!("setting_btn_open_rust_licenses")}
                        svg {
                            xmlns: "http://www.w3.org/2000/svg",
                            width: "16",
                            height: "16",
                            view_box: "0 0 16 16",
                            g { fill: "currentColor",
                                path { d: "M1.5 1H6v1H2v12h12v-4h1v4.5l-.5.5h-13l-.5-.5v-13z" }
                                path { d: "M15 1.5V8h-1V2.707L7.243 9.465l-.707-.708L13.293 2H8V1h6.5z" }
                            }
                        }
                    }
                }
                // Button to open npm modules license file
                li {
                    button {
                        class: "link link-info text-left inline-flex items-center gap-1",
                        onclick: move |_| {
                            if let Ok(exe_path) = env::current_exe() {
                                let mut license_path = exe_path.clone();
                                license_path.pop();
                                license_path.push("assets");
                                license_path.push("licenses");
                                license_path.push("npm-licenses.md");
                                let _ = webbrowser::open(license_path.to_string_lossy().as_ref());
                            }
                        },
                        {t!("setting_btn_open_npm_licenses")}
                        svg {
                            xmlns: "http://www.w3.org/2000/svg",
                            width: "16",
                            height: "16",
                            view_box: "0 0 16 16",
                            g { fill: "currentColor",
                                path { d: "M1.5 1H6v1H2v12h12v-4h1v4.5l-.5.5h-13l-.5-.5v-13z" }
                                path { d: "M15 1.5V8h-1V2.707L7.243 9.465l-.707-.708L13.293 2H8V1h6.5z" }
                            }
                        }
                    }
                }
                // Button to open other licenses file
                li {
                    button {
                        class: "link link-info text-left inline-flex items-center gap-1",
                        onclick: move |_| {
                            if let Ok(exe_path) = env::current_exe() {
                                let mut license_path = exe_path.clone();
                                license_path.pop();
                                license_path.push("assets");
                                license_path.push("licenses");
                                license_path.push("other-licenses.html");
                                let _ = webbrowser::open(license_path.to_string_lossy().as_ref());
                            }
                        },
                        {t!("setting_btn_open_other_licenses")}
                        svg {
                            xmlns: "http://www.w3.org/2000/svg",
                            width: "16",
                            height: "16",
                            view_box: "0 0 16 16",
                            g { fill: "currentColor",
                                path { d: "M1.5 1H6v1H2v12h12v-4h1v4.5l-.5.5h-13l-.5-.5v-13z" }
                                path { d: "M15 1.5V8h-1V2.707L7.243 9.465l-.707-.708L13.293 2H8V1h6.5z" }
                            }
                        }
                    }
                }
            }
        }
    }
}
