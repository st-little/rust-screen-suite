use chrono::Utc;
use chrono_tz::Tz;
use dioxus::{logger::tracing::debug, prelude::*};
use dioxus_i18n::t;
use std::{
    collections::VecDeque,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    thread,
    time::{Duration, Instant},
};

use crate::types::alert::AlertDetection;
use crate::{
    components::alert_detection_history::AlertDetectionHistory,
    services::alert::{
        add_detection, clear_detections, load_detections, load_templates, run_template_matching,
    },
    types::{
        settings::AppConfig,
        toast::{Toast, ToastKind},
    },
};

/// Alert component for monitoring and managing alert detections.
///
/// # Props
/// - `running`: Signal indicating whether monitoring is active.
/// - `config`: Application configuration signal.
/// - `toast_queue`: Queue for toast notifications.
///
/// This component provides UI controls to start/stop monitoring,
/// displays the alert detection history, and allows clearing the history.
#[component]
pub fn Alert(
    running: Signal<bool>,
    config: Signal<AppConfig>,
    toast_queue: Signal<VecDeque<Toast>>,
) -> Element {
    // Signal to control stopping the monitoring thread
    let mut stop_flag = use_signal(|| Arc::new(AtomicBool::new(false)));
    // Signal holding the detection history (thread-safe)
    let mut detections = use_signal(|| Arc::new(Mutex::new(load_detections())));
    // Parse timezone from config
    let timezone_str = &config.read().timezone;
    let timezone: Tz = timezone_str.parse().unwrap_or(chrono_tz::UTC);

    // Periodically trigger re-render to update detection history
    use_future(move || async move {
        loop {
            tokio::time::sleep(Duration::from_millis(500)).await;
            // Trigger re-render by mutating the signal (no-op)
            detections.with_mut(|_| {});
        }
    });

    rsx! {
        div { class: "flex gap-2 items-center",
            // Start monitoring button
            button {
                class: "btn btn-accent tooltip tooltip-right",
                "data-tip": t!("alert_tooltip_start"),
                disabled: "{running()}",
                onclick: move |_| {
                    if !running() {
                        stop_flag.write().store(false, Ordering::SeqCst);
                        let flag = stop_flag.read().clone();
                        let cfg = config.read().clone();
                        let template_cache = Arc::new(load_templates(&cfg));
                        let detections = detections.read().clone();
                        thread::spawn(move || {
                            while !flag.load(Ordering::SeqCst) {
                                match run_template_matching(&cfg, &template_cache) {
                                    Ok(Some((kind, score))) => {
                                        let det = AlertDetection {
                                            timestamp: Utc::now(),
                                            kind,
                                            score,
                                        };
                                        add_detection(&detections, det);
                                        thread::sleep(
                                            Duration::from_millis(cfg.monitor_skip_duration_ms),
                                        );
                                    }
                                    Ok(None) => {
                                        thread::sleep(
                                            Duration::from_millis(cfg.monitor_interval_ms),
                                        );
                                    }
                                    Err(e) => {
                                        debug!("Error: {:?}", e);
                                        thread::sleep(
                                            Duration::from_millis(cfg.monitor_interval_ms),
                                        );
                                    }
                                }
                            }
                        });
                        running.set(true);
                        toast_queue
                            .with_mut(|q| {
                                q.push_back(Toast {
                                    kind: ToastKind::Info,
                                    message: t!("alert_toast_started"),
                                    duration_ms: 4000,
                                    created: Instant::now(),
                                });
                            });
                    }
                },
                svg {
                    xmlns: "http://www.w3.org/2000/svg",
                    width: "24",
                    height: "24",
                    view_box: "0 0 16 16",
                    path {
                        fill: "currentColor",
                        d: "M8 6.003a2.667 2.667 0 1 1 0 5.334a2.667 2.667 0 0 1 0-5.334m0 1a1.667 1.667 0 1 0 0 3.334a1.667 1.667 0 0 0 0-3.334m0-3.336c3.076 0 5.73 2.1 6.467 5.043a.5.5 0 1 1-.97.242a5.67 5.67 0 0 0-10.995.004a.5.5 0 0 1-.97-.243A6.67 6.67 0 0 1 8 3.667",
                    }
                }
                {t!("alert_start_monitoring")}
            }
            // Stop monitoring button
            button {
                class: "btn btn-neutral tooltip tooltip-right",
                "data-tip": t!("alert_tooltip_stop"),
                disabled: "{!running()}",
                onclick: move |_| {
                    if running() {
                        stop_flag().store(true, Ordering::SeqCst);
                        running.set(false);
                        toast_queue
                            .with_mut(|q| {
                                q.push_back(Toast {
                                    kind: ToastKind::Info,
                                    message: t!("alert_toast_stopped"),
                                    duration_ms: 4000,
                                    created: Instant::now(),
                                });
                            });
                    }
                },
                svg {
                    xmlns: "http://www.w3.org/2000/svg",
                    width: "24",
                    height: "24",
                    view_box: "0 0 16 16",
                    path {
                        fill: "currentColor",
                        d: "M1.48 1.48a.5.5 0 0 0-.049.65l.049.057l2.69 2.69A6.66 6.66 0 0 0 1.533 8.71a.5.5 0 0 0 .97.242a5.66 5.66 0 0 1 2.386-3.356l1.207 1.207a2.667 2.667 0 0 0 3.771 3.771l3.946 3.946a.5.5 0 0 0 .756-.65l-.049-.057l-4.075-4.076v-.001l-.8-.799l-1.913-1.913h.001l-1.92-1.919v-.001l-.755-.754l-2.871-2.87a.5.5 0 0 0-.707 0m5.323 6.03l2.356 2.357A1.667 1.667 0 0 1 6.802 7.51M8 3.667c-.667 0-1.314.098-1.926.283l.825.824Q7.435 4.668 8 4.667a5.67 5.67 0 0 1 5.498 4.288a.5.5 0 0 0 .97-.242A6.67 6.67 0 0 0 8 3.667m.13 2.34l2.534 2.533A2.67 2.67 0 0 0 8.13 6.006",
                    }
                }
                {t!("alert_stop_monitoring")}
            }
        }
        div { class: "overflow-hidden",
            div { class: "flex items-center justify-between px-4 pt-4 border-b",
                span { class: "font-bold", {t!("alert_history")} }
                // Button to clear detection history
                button {
                    class: "btn btn-ghost btn-circle ml-4 tooltip tooltip-left",
                    "data-tip": t!("alert_tooltip_clear_history"),
                    onclick: {
                        move |_| {
                            clear_detections(&detections.read());
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
                            d: "M10 3h3v1h-1v9l-1 1H4l-1-1V4H2V3h3V2a1 1 0 0 1 1-1h3a1 1 0 0 1 1 1zM9 2H6v1h3zM4 13h7V4H4zm2-8H5v7h1zm1 0h1v7H7zm2 0h1v7H9z",
                            clip_rule: "evenodd",
                        }
                    }
                }
            }
            // Render the alert detection history list
            AlertDetectionHistory { detections, timezone }
        }
    }
}
