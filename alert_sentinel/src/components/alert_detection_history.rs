use chrono_tz::Tz;
use dioxus::prelude::*;
use dioxus_i18n::t;
use std::sync::{Arc, Mutex};

use crate::{
    services::alert_detection_history::format_detection_time, types::alert::AlertDetection,
};

/// Component to display the alert detection history as a list.
///
/// # Props
/// - `detections`: Shared state containing the list of alert detections.
/// - `timezone`: Timezone to format the detection timestamps.
///
/// This component renders a scrollable list of alert detections.
/// If there is no detection history, a message is shown instead.
/// Each detection displays the formatted timestamp and the alert kind.
#[component]
pub fn AlertDetectionHistory(
    detections: Signal<Arc<Mutex<Vec<AlertDetection>>>>,
    timezone: Tz,
) -> Element {
    rsx! {
        ul { class: "list p-4 rounded-box",
            // Scrollable container for detection history
            div { class: "overflow-y-auto max-h-80",
                if detections.read().lock().unwrap().is_empty() {
                    // Show message when there is no detection history
                    li { class: "p-4 text-sm opacity-60 tracking-wide", {t!("alert_no_history")} }
                } else {
                    // Render each detection in reverse order (latest first)
                    for detection in detections.read().lock().unwrap().iter().rev() {
                        li { class: "list-row",
                            // Display formatted detection time
                            label { class: "label text-base ",
                                {format_detection_time(&detection.timestamp, &timezone)}
                            }
                            // Display alert kind (localized)
                            label { class: "label text-base", {t!(detection.kind.as_key())} }
                        }
                    }
                }
            }
        }
    }
}
