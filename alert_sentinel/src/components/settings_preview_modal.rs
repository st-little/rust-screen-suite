use core_utils::image_util::image_file_to_data_url;
use dioxus::prelude::*;
use dioxus_i18n::t;

/// SettingsPreviewModal displays a modal dialog showing a preview of an image file.
///
/// # Props
/// - `show`: Signal controlling the visibility of the modal.
/// - `image_path`: Signal containing the path to the image file to preview.
///
/// The modal shows the image if it can be loaded, or an error message if loading fails.
/// Clicking the close button or the backdrop will close the modal.
#[component]
pub fn SettingsPreviewModal(show: Signal<bool>, image_path: Signal<String>) -> Element {
    if show() {
        let img_src = image_file_to_data_url(&image_path());
        rsx!(
            div { class: "modal modal-open",
                div {
                    class: "modal-box text-neutral-content",
                    style: "background-color: #111827 !important;",
                    h3 { class: "font-bold text-lg mb-4", {t!("setting_preview")} }
                    // Show image preview or error message
                    match img_src {
                        Some(src) => rsx! {
                            img {
                                class: "rounded w-full max-h-96 object-contain bg-black",
                                src,
                                alt: t!("setting_preview"),
                            }
                        },
                        None => rsx! {
                            div { class: "text-error", {t!("setting_msg_image_load_error")} }
                        },
                    }
                    div { class: "modal-action",
                        // Close button for the modal
                        button { class: "btn", onclick: move |_| show.set(false),
                            {t!("setting_btn_close")}
                        }
                    }
                }
                // Click on the backdrop to close the modal
                label {
                    class: "modal-backdrop",
                    onclick: move |_| show.set(false),
                }
            }
        )
    } else {
        rsx! {}
    }
}
