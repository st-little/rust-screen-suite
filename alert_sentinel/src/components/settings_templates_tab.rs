use dioxus::{logger::tracing::debug, prelude::*};
use dioxus_i18n::t;

/// Props for the SettingsTemplatesTab component.
/// Contains signals for preview modal and template image paths.
#[derive(PartialEq, Props, Clone)]
pub struct SettingsTemplatesTabProps {
    preview_image_path: Signal<String>,
    show_preview_modal: Signal<bool>,
    airdrop_path: Signal<String>,
    heli_path: Signal<String>,
    ch47_path: Signal<String>,
    cargo_path: Signal<String>,
    shovel_path: Signal<String>,
    oil_rig_large_path: Signal<String>,
    oil_rig_small_path: Signal<String>,
}

/// SettingsTemplatesTab displays template image settings for each alert type.
///
/// # Props
/// - `preview_image_path`: Signal for the preview image path.
/// - `show_preview_modal`: Signal to control the preview modal visibility.
/// - `*_path`: Signals for each alert template image path.
///
/// This component allows users to set, preview, and check the status of template images for detection.
#[component]
pub fn SettingsTemplatesTab(props: SettingsTemplatesTabProps) -> Element {
    let SettingsTemplatesTabProps {
        mut preview_image_path,
        mut show_preview_modal,
        mut airdrop_path,
        mut heli_path,
        mut ch47_path,
        mut cargo_path,
        mut shovel_path,
        mut oil_rig_large_path,
        mut oil_rig_small_path,
    } = props;

    rsx! {
        div {
            h3 { class: "text-xl font-bold mb-4", {t!("setting_tab_templates")} }
            p { class: "opacity-60 mb-4", {t!("setting_desc_templates")} }
            div { class: "space-y-4",
                div { class: "flex items-center gap-4",
                    label { class: "w-48 text-right font-medium", {t!("setting_template_airdrop")} }
                    {
                        let path = airdrop_path();
                        if !path.is_empty() && std::path::Path::new(&path).exists() {
                            rsx! {
                                div {
                                    style: "width:40px; display:flex; justify-content:center; align-items:center;",
                                    class: "tooltip tooltip-accent",
                                    "data-tip": t!("setting_set"),
                                    svg {
                                        class: "text-accent",
                                        xmlns: "http://www.w3.org/2000/svg",
                                        width: "24",
                                        height: "24",
                                        view_box: "0 0 16 16",
                                        g { fill: "currentColor",
                                            path { d: "M6.27 10.87h.71l4.56-4.56l-.71-.71l-4.2 4.21l-1.92-1.92L4 8.6z" }
                                            path {
                                                fill_rule: "evenodd",
                                                d: "M8.6 1c1.6.1 3.1.9 4.2 2c1.3 1.4 2 3.1 2 5.1c0 1.6-.6 3.1-1.6 4.4c-1 1.2-2.4 2.1-4 2.4s-3.2.1-4.6-.7s-2.5-2-3.1-3.5S.8 7.5 1.3 6c.5-1.6 1.4-2.9 2.8-3.8C5.4 1.3 7 .9 8.6 1m.5 12.9c1.3-.3 2.5-1 3.4-2.1c.8-1.1 1.3-2.4 1.2-3.8c0-1.6-.6-3.2-1.7-4.3c-1-1-2.2-1.6-3.6-1.7c-1.3-.1-2.7.2-3.8 1S2.7 4.9 2.3 6.3c-.4 1.3-.4 2.7.2 4q.9 1.95 2.7 3c1.2.7 2.6.9 3.9.6",
                                                clip_rule: "evenodd",
                                            }
                                        }
                                    }
                                }
                            }
                        } else {
                            rsx! {
                                div {
                                    style: "width:40px; display:flex; justify-content:center; align-items:center;",
                                    class: "text-warning tooltip tooltip-warning",
                                    "data-tip": t!("setting_not_set"),
                                    svg {
                                        class: "text-warning",
                                        xmlns: "http://www.w3.org/2000/svg",
                                        width: "24",
                                        height: "24",
                                        view_box: "0 0 16 16",
                                        path {
                                            fill: "currentColor",
                                            fill_rule: "evenodd",
                                            d: "M7.56 1h.88l6.54 12.26l-.44.74H1.44L1 13.26zM8 2.28L2.28 13H13.7zM8.625 12v-1h-1.25v1zm-1.25-2V6h1.25v4z",
                                            clip_rule: "evenodd",
                                        }
                                    }
                                }
                            }
                        }
                    }
                    input {
                        r#type: "file",
                        accept: ".png",
                        multiple: false,
                        class: "file-input file-input-bordered w-64",
                        onchange: move |evt| {
                            if let Some(file_engine) = &evt.files() {
                                let files = file_engine.files();
                                for file_name in files {
                                    debug!("selected file: {}", file_name);
                                    airdrop_path.set(file_name);
                                }
                            }
                        },
                    }
                    {
                        let path = airdrop_path();
                        rsx! {
                            div { style: "width:40px; display:flex; justify-content:center; align-items:center;",
                                button {
                                    class: "btn btn-xs btn-ghost tooltip",
                                    "data-tip": t!("setting_preview"),
                                    onclick: move |_| {
                                        preview_image_path.set(path.clone());
                                        show_preview_modal.set(true);
                                    },
                                    svg {
                                        xmlns: "http://www.w3.org/2000/svg",
                                        width: "24",
                                        height: "24",
                                        view_box: "0 0 16 16",
                                        path {
                                            fill: "currentColor",
                                            fill_rule: "evenodd",
                                            d: "M2 2h12l1 1v10l-1 1H2l-1-1V3zm0 11h12V3H2zm11-9H3v3h10zm-1 2H4V5h8zm-3 6h4V8H9zm1-3h2v2h-2zM7 8H3v1h4zm-4 3h4v1H3z",
                                            clip_rule: "evenodd",
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                div { class: "flex items-center gap-4",
                    label { class: "w-48 text-right font-medium", {t!("setting_template_heli")} }
                    {
                        let path = heli_path();
                        if !path.is_empty() && std::path::Path::new(&path).exists() {
                            rsx! {
                                div {
                                    style: "width:40px; display:flex; justify-content:center; align-items:center;",
                                    class: "tooltip tooltip-accent",
                                    "data-tip": t!("setting_set"),
                                    svg {
                                        class: "text-accent",
                                        xmlns: "http://www.w3.org/2000/svg",
                                        width: "24",
                                        height: "24",
                                        view_box: "0 0 16 16",
                                        g { fill: "currentColor",
                                            path { d: "M6.27 10.87h.71l4.56-4.56l-.71-.71l-4.2 4.21l-1.92-1.92L4 8.6z" }
                                            path {
                                                fill_rule: "evenodd",
                                                d: "M8.6 1c1.6.1 3.1.9 4.2 2c1.3 1.4 2 3.1 2 5.1c0 1.6-.6 3.1-1.6 4.4c-1 1.2-2.4 2.1-4 2.4s-3.2.1-4.6-.7s-2.5-2-3.1-3.5S.8 7.5 1.3 6c.5-1.6 1.4-2.9 2.8-3.8C5.4 1.3 7 .9 8.6 1m.5 12.9c1.3-.3 2.5-1 3.4-2.1c.8-1.1 1.3-2.4 1.2-3.8c0-1.6-.6-3.2-1.7-4.3c-1-1-2.2-1.6-3.6-1.7c-1.3-.1-2.7.2-3.8 1S2.7 4.9 2.3 6.3c-.4 1.3-.4 2.7.2 4q.9 1.95 2.7 3c1.2.7 2.6.9 3.9.6",
                                                clip_rule: "evenodd",
                                            }
                                        }
                                    }
                                }
                            }
                        } else {
                            rsx! {
                                div {
                                    style: "width:40px; display:flex; justify-content:center; align-items:center;",
                                    class: "text-warning tooltip tooltip-warning",
                                    "data-tip": t!("setting_not_set"),
                                    svg {
                                        class: "text-warning",
                                        xmlns: "http://www.w3.org/2000/svg",
                                        width: "24",
                                        height: "24",
                                        view_box: "0 0 16 16",
                                        path {
                                            fill: "currentColor",
                                            fill_rule: "evenodd",
                                            d: "M7.56 1h.88l6.54 12.26l-.44.74H1.44L1 13.26zM8 2.28L2.28 13H13.7zM8.625 12v-1h-1.25v1zm-1.25-2V6h1.25v4z",
                                            clip_rule: "evenodd",
                                        }
                                    }
                                }
                            }
                        }
                    }
                    input {
                        r#type: "file",
                        accept: ".png",
                        multiple: false,
                        class: "file-input file-input-bordered w-64",
                        onchange: move |evt| {
                            if let Some(file_engine) = &evt.files() {
                                let files = file_engine.files();
                                for file_name in files {
                                    debug!("selected file: {}", file_name);
                                    heli_path.set(file_name);
                                }
                            }
                        },
                    }
                    {
                        let path = heli_path();
                        rsx! {
                            div { style: "width:40px; display:flex; justify-content:center; align-items:center;",
                                button {
                                    class: "btn btn-xs btn-ghost tooltip",
                                    "data-tip": t!("setting_preview"),
                                    onclick: move |_| {
                                        preview_image_path.set(path.clone());
                                        show_preview_modal.set(true);
                                    },
                                    svg {
                                        xmlns: "http://www.w3.org/2000/svg",
                                        width: "24",
                                        height: "24",
                                        view_box: "0 0 16 16",
                                        path {
                                            fill: "currentColor",
                                            fill_rule: "evenodd",
                                            d: "M2 2h12l1 1v10l-1 1H2l-1-1V3zm0 11h12V3H2zm11-9H3v3h10zm-1 2H4V5h8zm-3 6h4V8H9zm1-3h2v2h-2zM7 8H3v1h4zm-4 3h4v1H3z",
                                            clip_rule: "evenodd",
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                div { class: "flex items-center gap-4",
                    label { class: "w-48 text-right font-medium", {t!("setting_template_ch47")} }
                    {
                        let path = ch47_path();
                        if !path.is_empty() && std::path::Path::new(&path).exists() {
                            rsx! {
                                div {
                                    style: "width:40px; display:flex; justify-content:center; align-items:center;",
                                    class: "tooltip tooltip-accent",
                                    "data-tip": t!("setting_set"),
                                    svg {
                                        class: "text-accent",
                                        xmlns: "http://www.w3.org/2000/svg",
                                        width: "24",
                                        height: "24",
                                        view_box: "0 0 16 16",
                                        g { fill: "currentColor",
                                            path { d: "M6.27 10.87h.71l4.56-4.56l-.71-.71l-4.2 4.21l-1.92-1.92L4 8.6z" }
                                            path {
                                                fill_rule: "evenodd",
                                                d: "M8.6 1c1.6.1 3.1.9 4.2 2c1.3 1.4 2 3.1 2 5.1c0 1.6-.6 3.1-1.6 4.4c-1 1.2-2.4 2.1-4 2.4s-3.2.1-4.6-.7s-2.5-2-3.1-3.5S.8 7.5 1.3 6c.5-1.6 1.4-2.9 2.8-3.8C5.4 1.3 7 .9 8.6 1m.5 12.9c1.3-.3 2.5-1 3.4-2.1c.8-1.1 1.3-2.4 1.2-3.8c0-1.6-.6-3.2-1.7-4.3c-1-1-2.2-1.6-3.6-1.7c-1.3-.1-2.7.2-3.8 1S2.7 4.9 2.3 6.3c-.4 1.3-.4 2.7.2 4q.9 1.95 2.7 3c1.2.7 2.6.9 3.9.6",
                                                clip_rule: "evenodd",
                                            }
                                        }
                                    }
                                }
                            }
                        } else {
                            rsx! {
                                div {
                                    style: "width:40px; display:flex; justify-content:center; align-items:center;",
                                    class: "text-warning tooltip tooltip-warning",
                                    "data-tip": t!("setting_not_set"),
                                    svg {
                                        class: "text-warning",
                                        xmlns: "http://www.w3.org/2000/svg",
                                        width: "24",
                                        height: "24",
                                        view_box: "0 0 16 16",
                                        path {
                                            fill: "currentColor",
                                            fill_rule: "evenodd",
                                            d: "M7.56 1h.88l6.54 12.26l-.44.74H1.44L1 13.26zM8 2.28L2.28 13H13.7zM8.625 12v-1h-1.25v1zm-1.25-2V6h1.25v4z",
                                            clip_rule: "evenodd",
                                        }
                                    }
                                }
                            }
                        }
                    }
                    input {
                        r#type: "file",
                        accept: ".png",
                        multiple: false,
                        class: "file-input file-input-bordered w-64",
                        onchange: move |evt| {
                            if let Some(file_engine) = &evt.files() {
                                let files = file_engine.files();
                                for file_name in files {
                                    debug!("selected file: {}", file_name);
                                    ch47_path.set(file_name);
                                }
                            }
                        },
                    }
                    {
                        let path = ch47_path();
                        rsx! {
                            div { style: "width:40px; display:flex; justify-content:center; align-items:center;",
                                button {
                                    class: "btn btn-xs btn-ghost tooltip",
                                    "data-tip": t!("setting_preview"),
                                    onclick: move |_| {
                                        preview_image_path.set(path.clone());
                                        show_preview_modal.set(true);
                                    },
                                    svg {
                                        xmlns: "http://www.w3.org/2000/svg",
                                        width: "24",
                                        height: "24",
                                        view_box: "0 0 16 16",
                                        path {
                                            fill: "currentColor",
                                            fill_rule: "evenodd",
                                            d: "M2 2h12l1 1v10l-1 1H2l-1-1V3zm0 11h12V3H2zm11-9H3v3h10zm-1 2H4V5h8zm-3 6h4V8H9zm1-3h2v2h-2zM7 8H3v1h4zm-4 3h4v1H3z",
                                            clip_rule: "evenodd",
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                div { class: "flex items-center gap-4",
                    label { class: "w-48 text-right font-medium", {t!("setting_template_cargo")} }
                    {
                        let path = cargo_path();
                        if !path.is_empty() && std::path::Path::new(&path).exists() {
                            rsx! {
                                div {
                                    style: "width:40px; display:flex; justify-content:center; align-items:center;",
                                    class: "tooltip tooltip-accent",
                                    "data-tip": t!("setting_set"),
                                    svg {
                                        class: "text-accent",
                                        xmlns: "http://www.w3.org/2000/svg",
                                        width: "24",
                                        height: "24",
                                        view_box: "0 0 16 16",
                                        g { fill: "currentColor",
                                            path { d: "M6.27 10.87h.71l4.56-4.56l-.71-.71l-4.2 4.21l-1.92-1.92L4 8.6z" }
                                            path {
                                                fill_rule: "evenodd",
                                                d: "M8.6 1c1.6.1 3.1.9 4.2 2c1.3 1.4 2 3.1 2 5.1c0 1.6-.6 3.1-1.6 4.4c-1 1.2-2.4 2.1-4 2.4s-3.2.1-4.6-.7s-2.5-2-3.1-3.5S.8 7.5 1.3 6c.5-1.6 1.4-2.9 2.8-3.8C5.4 1.3 7 .9 8.6 1m.5 12.9c1.3-.3 2.5-1 3.4-2.1c.8-1.1 1.3-2.4 1.2-3.8c0-1.6-.6-3.2-1.7-4.3c-1-1-2.2-1.6-3.6-1.7c-1.3-.1-2.7.2-3.8 1S2.7 4.9 2.3 6.3c-.4 1.3-.4 2.7.2 4q.9 1.95 2.7 3c1.2.7 2.6.9 3.9.6",
                                                clip_rule: "evenodd",
                                            }
                                        }
                                    }
                                }
                            }
                        } else {
                            rsx! {
                                div {
                                    style: "width:40px; display:flex; justify-content:center; align-items:center;",
                                    class: "text-warning tooltip tooltip-warning",
                                    "data-tip": t!("setting_not_set"),
                                    svg {
                                        class: "text-warning",
                                        xmlns: "http://www.w3.org/2000/svg",
                                        width: "24",
                                        height: "24",
                                        view_box: "0 0 16 16",
                                        path {
                                            fill: "currentColor",
                                            fill_rule: "evenodd",
                                            d: "M7.56 1h.88l6.54 12.26l-.44.74H1.44L1 13.26zM8 2.28L2.28 13H13.7zM8.625 12v-1h-1.25v1zm-1.25-2V6h1.25v4z",
                                            clip_rule: "evenodd",
                                        }
                                    }
                                }
                            }
                        }
                    }
                    input {
                        r#type: "file",
                        accept: ".png",
                        multiple: false,
                        class: "file-input file-input-bordered w-64",
                        onchange: move |evt| {
                            if let Some(file_engine) = &evt.files() {
                                let files = file_engine.files();
                                for file_name in files {
                                    debug!("selected file: {}", file_name);
                                    cargo_path.set(file_name);
                                }
                            }
                        },
                    }
                    {
                        let path = cargo_path();
                        rsx! {
                            div { style: "width:40px; display:flex; justify-content:center; align-items:center;",
                                button {
                                    class: "btn btn-xs btn-ghost tooltip",
                                    "data-tip": t!("setting_preview"),
                                    onclick: move |_| {
                                        preview_image_path.set(path.clone());
                                        show_preview_modal.set(true);
                                    },
                                    svg {
                                        xmlns: "http://www.w3.org/2000/svg",
                                        width: "24",
                                        height: "24",
                                        view_box: "0 0 16 16",
                                        path {
                                            fill: "currentColor",
                                            fill_rule: "evenodd",
                                            d: "M2 2h12l1 1v10l-1 1H2l-1-1V3zm0 11h12V3H2zm11-9H3v3h10zm-1 2H4V5h8zm-3 6h4V8H9zm1-3h2v2h-2zM7 8H3v1h4zm-4 3h4v1H3z",
                                            clip_rule: "evenodd",
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                div { class: "flex items-center gap-4",
                    label { class: "w-48 text-right font-medium", {t!("setting_template_shovel")} }
                    {
                        let path = shovel_path();
                        if !path.is_empty() && std::path::Path::new(&path).exists() {
                            rsx! {
                                div {
                                    style: "width:40px; display:flex; justify-content:center; align-items:center;",
                                    class: "tooltip tooltip-accent",
                                    "data-tip": t!("setting_set"),
                                    svg {
                                        class: "text-accent",
                                        xmlns: "http://www.w3.org/2000/svg",
                                        width: "24",
                                        height: "24",
                                        view_box: "0 0 16 16",
                                        g { fill: "currentColor",
                                            path { d: "M6.27 10.87h.71l4.56-4.56l-.71-.71l-4.2 4.21l-1.92-1.92L4 8.6z" }
                                            path {
                                                fill_rule: "evenodd",
                                                d: "M8.6 1c1.6.1 3.1.9 4.2 2c1.3 1.4 2 3.1 2 5.1c0 1.6-.6 3.1-1.6 4.4c-1 1.2-2.4 2.1-4 2.4s-3.2.1-4.6-.7s-2.5-2-3.1-3.5S.8 7.5 1.3 6c.5-1.6 1.4-2.9 2.8-3.8C5.4 1.3 7 .9 8.6 1m.5 12.9c1.3-.3 2.5-1 3.4-2.1c.8-1.1 1.3-2.4 1.2-3.8c0-1.6-.6-3.2-1.7-4.3c-1-1-2.2-1.6-3.6-1.7c-1.3-.1-2.7.2-3.8 1S2.7 4.9 2.3 6.3c-.4 1.3-.4 2.7.2 4q.9 1.95 2.7 3c1.2.7 2.6.9 3.9.6",
                                                clip_rule: "evenodd",
                                            }
                                        }
                                    }
                                }
                            }
                        } else {
                            rsx! {
                                div {
                                    style: "width:40px; display:flex; justify-content:center; align-items:center;",
                                    class: "text-warning tooltip tooltip-warning",
                                    "data-tip": t!("setting_not_set"),
                                    svg {
                                        class: "text-warning",
                                        xmlns: "http://www.w3.org/2000/svg",
                                        width: "24",
                                        height: "24",
                                        view_box: "0 0 16 16",
                                        path {
                                            fill: "currentColor",
                                            fill_rule: "evenodd",
                                            d: "M7.56 1h.88l6.54 12.26l-.44.74H1.44L1 13.26zM8 2.28L2.28 13H13.7zM8.625 12v-1h-1.25v1zm-1.25-2V6h1.25v4z",
                                            clip_rule: "evenodd",
                                        }
                                    }
                                }
                            }
                        }
                    }
                    input {
                        r#type: "file",
                        accept: ".png",
                        multiple: false,
                        class: "file-input file-input-bordered w-64",
                        onchange: move |evt| {
                            if let Some(file_engine) = &evt.files() {
                                let files = file_engine.files();
                                for file_name in files {
                                    debug!("selected file: {}", file_name);
                                    shovel_path.set(file_name);
                                }
                            }
                        },
                    }
                    {
                        let path = shovel_path();
                        rsx! {
                            div { style: "width:40px; display:flex; justify-content:center; align-items:center;",
                                button {
                                    class: "btn btn-xs btn-ghost tooltip",
                                    "data-tip": t!("setting_preview"),
                                    onclick: move |_| {
                                        preview_image_path.set(path.clone());
                                        show_preview_modal.set(true);
                                    },
                                    svg {
                                        xmlns: "http://www.w3.org/2000/svg",
                                        width: "24",
                                        height: "24",
                                        view_box: "0 0 16 16",
                                        path {
                                            fill: "currentColor",
                                            fill_rule: "evenodd",
                                            d: "M2 2h12l1 1v10l-1 1H2l-1-1V3zm0 11h12V3H2zm11-9H3v3h10zm-1 2H4V5h8zm-3 6h4V8H9zm1-3h2v2h-2zM7 8H3v1h4zm-4 3h4v1H3z",
                                            clip_rule: "evenodd",
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                div { class: "flex items-center gap-4",
                    label { class: "w-48 text-right font-medium",
                        {t!("setting_template_oil_rig_large")}
                    }
                    {
                        let path = oil_rig_large_path();
                        if !path.is_empty() && std::path::Path::new(&path).exists() {
                            rsx! {
                                div {
                                    style: "width:40px; display:flex; justify-content:center; align-items:center;",
                                    class: "tooltip tooltip-accent",
                                    "data-tip": t!("setting_set"),
                                    svg {
                                        class: "text-accent",
                                        xmlns: "http://www.w3.org/2000/svg",
                                        width: "24",
                                        height: "24",
                                        view_box: "0 0 16 16",
                                        g { fill: "currentColor",
                                            path { d: "M6.27 10.87h.71l4.56-4.56l-.71-.71l-4.2 4.21l-1.92-1.92L4 8.6z" }
                                            path {
                                                fill_rule: "evenodd",
                                                d: "M8.6 1c1.6.1 3.1.9 4.2 2c1.3 1.4 2 3.1 2 5.1c0 1.6-.6 3.1-1.6 4.4c-1 1.2-2.4 2.1-4 2.4s-3.2.1-4.6-.7s-2.5-2-3.1-3.5S.8 7.5 1.3 6c.5-1.6 1.4-2.9 2.8-3.8C5.4 1.3 7 .9 8.6 1m.5 12.9c1.3-.3 2.5-1 3.4-2.1c.8-1.1 1.3-2.4 1.2-3.8c0-1.6-.6-3.2-1.7-4.3c-1-1-2.2-1.6-3.6-1.7c-1.3-.1-2.7.2-3.8 1S2.7 4.9 2.3 6.3c-.4 1.3-.4 2.7.2 4q.9 1.95 2.7 3c1.2.7 2.6.9 3.9.6",
                                                clip_rule: "evenodd",
                                            }
                                        }
                                    }
                                }
                            }
                        } else {
                            rsx! {
                                div {
                                    style: "width:40px; display:flex; justify-content:center; align-items:center;",
                                    class: "text-warning tooltip tooltip-warning",
                                    "data-tip": t!("setting_not_set"),
                                    svg {
                                        class: "text-warning",
                                        xmlns: "http://www.w3.org/2000/svg",
                                        width: "24",
                                        height: "24",
                                        view_box: "0 0 16 16",
                                        path {
                                            fill: "currentColor",
                                            fill_rule: "evenodd",
                                            d: "M7.56 1h.88l6.54 12.26l-.44.74H1.44L1 13.26zM8 2.28L2.28 13H13.7zM8.625 12v-1h-1.25v1zm-1.25-2V6h1.25v4z",
                                            clip_rule: "evenodd",
                                        }
                                    }
                                }
                            }
                        }
                    }
                    input {
                        r#type: "file",
                        accept: ".png",
                        multiple: false,
                        class: "file-input file-input-bordered w-64",
                        onchange: move |evt| {
                            if let Some(file_engine) = &evt.files() {
                                let files = file_engine.files();
                                for file_name in files {
                                    debug!("selected file: {}", file_name);
                                    oil_rig_large_path.set(file_name);
                                }
                            }
                        },
                    }
                    {
                        let path = oil_rig_large_path();
                        rsx! {
                            div { style: "width:40px; display:flex; justify-content:center; align-items:center;",
                                button {
                                    class: "btn btn-xs btn-ghost tooltip",
                                    "data-tip": t!("setting_preview"),
                                    onclick: move |_| {
                                        preview_image_path.set(path.clone());
                                        show_preview_modal.set(true);
                                    },
                                    svg {
                                        xmlns: "http://www.w3.org/2000/svg",
                                        width: "24",
                                        height: "24",
                                        view_box: "0 0 16 16",
                                        path {
                                            fill: "currentColor",
                                            fill_rule: "evenodd",
                                            d: "M2 2h12l1 1v10l-1 1H2l-1-1V3zm0 11h12V3H2zm11-9H3v3h10zm-1 2H4V5h8zm-3 6h4V8H9zm1-3h2v2h-2zM7 8H3v1h4zm-4 3h4v1H3z",
                                            clip_rule: "evenodd",
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                div { class: "flex items-center gap-4",
                    label { class: "w-48 text-right font-medium",
                        {t!("setting_template_oil_rig_small")}
                    }
                    {
                        let path = oil_rig_small_path();
                        if !path.is_empty() && std::path::Path::new(&path).exists() {
                            rsx! {
                                div {
                                    style: "width:40px; display:flex; justify-content:center; align-items:center;",
                                    class: "tooltip tooltip-accent",
                                    "data-tip": t!("setting_set"),
                                    svg {
                                        class: "text-accent",
                                        xmlns: "http://www.w3.org/2000/svg",
                                        width: "24",
                                        height: "24",
                                        view_box: "0 0 16 16",
                                        g { fill: "currentColor",
                                            path { d: "M6.27 10.87h.71l4.56-4.56l-.71-.71l-4.2 4.21l-1.92-1.92L4 8.6z" }
                                            path {
                                                fill_rule: "evenodd",
                                                d: "M8.6 1c1.6.1 3.1.9 4.2 2c1.3 1.4 2 3.1 2 5.1c0 1.6-.6 3.1-1.6 4.4c-1 1.2-2.4 2.1-4 2.4s-3.2.1-4.6-.7s-2.5-2-3.1-3.5S.8 7.5 1.3 6c.5-1.6 1.4-2.9 2.8-3.8C5.4 1.3 7 .9 8.6 1m.5 12.9c1.3-.3 2.5-1 3.4-2.1c.8-1.1 1.3-2.4 1.2-3.8c0-1.6-.6-3.2-1.7-4.3c-1-1-2.2-1.6-3.6-1.7c-1.3-.1-2.7.2-3.8 1S2.7 4.9 2.3 6.3c-.4 1.3-.4 2.7.2 4q.9 1.95 2.7 3c1.2.7 2.6.9 3.9.6",
                                                clip_rule: "evenodd",
                                            }
                                        }
                                    }
                                }
                            }
                        } else {
                            rsx! {
                                div {
                                    style: "width:40px; display:flex; justify-content:center; align-items:center;",
                                    class: "text-warning tooltip tooltip-warning",
                                    "data-tip": t!("setting_not_set"),
                                    svg {
                                        class: "text-warning",
                                        xmlns: "http://www.w3.org/2000/svg",
                                        width: "24",
                                        height: "24",
                                        view_box: "0 0 16 16",
                                        path {
                                            fill: "currentColor",
                                            fill_rule: "evenodd",
                                            d: "M7.56 1h.88l6.54 12.26l-.44.74H1.44L1 13.26zM8 2.28L2.28 13H13.7zM8.625 12v-1h-1.25v1zm-1.25-2V6h1.25v4z",
                                            clip_rule: "evenodd",
                                        }
                                    }
                                }
                            }
                        }
                    }
                    input {
                        r#type: "file",
                        accept: ".png",
                        multiple: false,
                        class: "file-input file-input-bordered w-64",
                        onchange: move |evt| {
                            if let Some(file_engine) = &evt.files() {
                                let files = file_engine.files();
                                for file_name in files {
                                    debug!("selected file: {}", file_name);
                                    oil_rig_small_path.set(file_name);
                                }
                            }
                        },
                    }
                    {
                        let path = oil_rig_small_path();
                        rsx! {
                            div { style: "width:40px; display:flex; justify-content:center; align-items:center;",
                                button {
                                    class: "btn btn-xs btn-ghost tooltip",
                                    "data-tip": t!("setting_preview"),
                                    onclick: move |_| {
                                        preview_image_path.set(path.clone());
                                        show_preview_modal.set(true);
                                    },
                                    svg {
                                        xmlns: "http://www.w3.org/2000/svg",
                                        width: "24",
                                        height: "24",
                                        view_box: "0 0 16 16",
                                        path {
                                            fill: "currentColor",
                                            fill_rule: "evenodd",
                                            d: "M2 2h12l1 1v10l-1 1H2l-1-1V3zm0 11h12V3H2zm11-9H3v3h10zm-1 2H4V5h8zm-3 6h4V8H9zm1-3h2v2h-2zM7 8H3v1h4zm-4 3h4v1H3z",
                                            clip_rule: "evenodd",
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
