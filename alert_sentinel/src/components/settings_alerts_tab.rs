use dioxus::prelude::*;
use dioxus_i18n::t;

/// Props for the SettingsAlertsTab component.
/// Each field is a signal controlling the toggle state for a specific alert type.
#[derive(PartialEq, Props, Clone)]
pub struct SettingsAlertsTabProps {
    pub airdrop_toggle: Signal<bool>,
    pub heli_toggle: Signal<bool>,
    pub ch47_toggle: Signal<bool>,
    pub cargo_toggle: Signal<bool>,
    pub shovel_toggle: Signal<bool>,
    pub oil_rig_large_toggle: Signal<bool>,
    pub oil_rig_small_toggle: Signal<bool>,
}

/// SettingsAlertsTab displays toggles for enabling or disabling detection of each alert type.
///
/// # Props
/// - `airdrop_toggle`: Signal for airdrop alert toggle.
/// - `heli_toggle`: Signal for heli alert toggle.
/// - `ch47_toggle`: Signal for ch47 alert toggle.
/// - `cargo_toggle`: Signal for cargo alert toggle.
/// - `shovel_toggle`: Signal for shovel alert toggle.
/// - `oil_rig_large_toggle`: Signal for large oil rig alert toggle.
/// - `oil_rig_small_toggle`: Signal for small oil rig alert toggle.
///
/// Each toggle controls whether the corresponding alert type is enabled for detection.
#[component]
pub fn SettingsAlertsTab(props: SettingsAlertsTabProps) -> Element {
    let SettingsAlertsTabProps {
        mut airdrop_toggle,
        mut heli_toggle,
        mut ch47_toggle,
        mut cargo_toggle,
        mut shovel_toggle,
        mut oil_rig_large_toggle,
        mut oil_rig_small_toggle,
    } = props;

    rsx! {
        h3 { class: "text-xl font-bold mb-4", {t!("setting_tab_alerts")} }
        p { class: "opacity-60 mb-4", {t!("setting_desc_alerts")} }
        ul { class: "list p-4 rounded-box",
            // Toggle for airdrop alert
            li { class: "list-row",
                fieldset { class: "fieldset",
                    label { class: "label text-base",
                        input {
                            class: "toggle bg-primary-content checked:text-primary",
                            r#type: "checkbox",
                            checked: airdrop_toggle(),
                            onchange: move |e| {
                                airdrop_toggle.set(e.checked());
                            },
                        }
                        {t!("setting_template_airdrop")}
                    }
                }
            }
            // Toggle for heli alert
            li { class: "list-row",
                fieldset { class: "fieldset",
                    label { class: "label text-base",
                        input {
                            class: "toggle bg-primary-content checked:text-primary",
                            r#type: "checkbox",
                            checked: heli_toggle(),
                            onchange: move |e| {
                                heli_toggle.set(e.checked());
                            },
                        }
                        {t!("setting_template_heli")}
                    }
                }
            }
            // Toggle for ch47 alert
            li { class: "list-row",
                fieldset { class: "fieldset",
                    label { class: "label text-base",
                        input {
                            class: "toggle bg-primary-content checked:text-primary",
                            r#type: "checkbox",
                            checked: ch47_toggle(),
                            onchange: move |e| {
                                ch47_toggle.set(e.checked());
                            },
                        }
                        {t!("setting_template_ch47")}
                    }
                }
            }
            // Toggle for cargo alert
            li { class: "list-row",
                fieldset { class: "fieldset",
                    label { class: "label text-base",
                        input {
                            class: "toggle bg-primary-content checked:text-primary",
                            r#type: "checkbox",
                            checked: cargo_toggle(),
                            onchange: move |e| {
                                cargo_toggle.set(e.checked());
                            },
                        }
                        {t!("setting_template_cargo")}
                    }
                }
            }
            // Toggle for shovel alert
            li { class: "list-row",
                fieldset { class: "fieldset",
                    label { class: "label text-base",
                        input {
                            class: "toggle bg-primary-content checked:text-primary",
                            r#type: "checkbox",
                            checked: shovel_toggle(),
                            onchange: move |e| {
                                shovel_toggle.set(e.checked());
                            },
                        }
                        {t!("setting_template_shovel")}
                    }
                }
            }
            // Toggle for large oil rig alert
            li { class: "list-row",
                fieldset { class: "fieldset",
                    label { class: "label text-base",
                        input {
                            class: "toggle bg-primary-content checked:text-primary",
                            r#type: "checkbox",
                            checked: oil_rig_large_toggle(),
                            onchange: move |e| {
                                oil_rig_large_toggle.set(e.checked());
                            },
                        }
                        {t!("setting_template_oil_rig_large")}
                    }
                }
            }
            // Toggle for small oil rig alert
            li { class: "list-row",
                fieldset { class: "fieldset",
                    label { class: "label text-base",
                        input {
                            class: "toggle bg-primary-content checked:text-primary",
                            r#type: "checkbox",
                            checked: oil_rig_small_toggle(),
                            onchange: move |e| {
                                oil_rig_small_toggle.set(e.checked());
                            },
                        }
                        {t!("setting_template_oil_rig_small")}
                    }
                }
            }
        }
    }
}
