use dioxus::prelude::*;
use dioxus_i18n::t;

/// Props for the SettingsMonitorTab component.
/// Contains signals for monitor interval, skip duration, threshold, and scale.
#[derive(PartialEq, Props, Clone)]
pub struct SettingsMonitorTabProps {
    pub monitor_interval: Signal<u64>,
    pub monitor_skip_duration: Signal<u64>,
    pub monitor_threshold: Signal<f32>,
    pub monitor_scale: Signal<f32>,
}

/// SettingsMonitorTab displays monitor-related settings such as interval, skip duration, threshold, and scale.
///
/// # Props
/// - `monitor_interval`: Signal for the monitoring interval in milliseconds.
/// - `monitor_skip_duration`: Signal for the skip duration after detection in milliseconds.
/// - `monitor_threshold`: Signal for the detection threshold.
/// - `monitor_scale`: Signal for the image scaling factor.
///
/// This component provides sliders for each monitor setting.
#[component]
pub fn SettingsMonitorTab(props: SettingsMonitorTabProps) -> Element {
    let SettingsMonitorTabProps {
        mut monitor_interval,
        mut monitor_skip_duration,
        mut monitor_threshold,
        mut monitor_scale,
    } = props;

    rsx! {
        div {
            h3 { class: "text-xl font-bold mb-4", {t!("setting_tab_monitor")} }
            p { class: "opacity-60 mb-4", {t!("setting_desc_monitor")} }
            div { class: "space-y-4",
                // Slider for monitor interval
                div { class: "flex items-center gap-4",
                    label { class: "w-48 text-right font-medium",
                        {t!("setting_monitor_interval", ms : monitor_interval.to_string())}
                    }
                    input {
                        r#type: "range",
                        min: "100",
                        max: "2000",
                        step: "100",
                        value: monitor_interval.to_string(),
                        class: "range w-64",
                        onchange: move |e| {
                            if let Ok(interval) = e.value().parse::<u64>() {
                                monitor_interval.set(interval);
                            }
                        },
                    }
                }
                // Slider for skip duration after detection
                div { class: "flex items-center gap-4",
                    label { class: "w-48 text-right font-medium",
                        {t!("setting_monitor_skip_duration", ms : monitor_skip_duration.to_string())}
                    }
                    input {
                        r#type: "range",
                        min: "1000",
                        max: "5000",
                        step: "500",
                        value: monitor_skip_duration.to_string(),
                        class: "range w-64",
                        onchange: move |e| {
                            if let Ok(interval) = e.value().parse::<u64>() {
                                monitor_skip_duration.set(interval);
                            }
                        },
                    }
                }
                // Slider for detection threshold
                div { class: "flex items-center gap-4",
                    label { class: "w-48 text-right font-medium",
                        {t!("setting_monitor_threshold", value : monitor_threshold.to_string())}
                    }
                    input {
                        r#type: "range",
                        min: "0.0",
                        max: "1.0",
                        step: "0.1",
                        value: monitor_threshold.to_string(),
                        class: "range w-64",
                        onchange: move |e| {
                            if let Ok(threshold) = e.value().parse::<f32>() {
                                monitor_threshold.set(threshold);
                            }
                        },
                    }
                }
                // Slider for image scaling factor
                div { class: "flex items-center gap-4",
                    label { class: "w-48 text-right font-medium",
                        {t!("setting_monitor_scale", value : monitor_scale().to_string())}
                    }
                    input {
                        r#type: "range",
                        min: "0.25",
                        max: "1.0",
                        step: "0.25",
                        value: monitor_scale().to_string(),
                        class: "range w-64",
                        onchange: move |e| {
                            if let Ok(scale) = e.value().parse::<f32>() {
                                monitor_scale.set(scale);
                            }
                        },
                    }
                }
            }
        }
    }
}
