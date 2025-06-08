use dioxus::prelude::*;
use std::{
    collections::VecDeque,
    time::{Duration, Instant},
};

use crate::types::toast::Toast;

/// Toasts component displays toast notifications in the application.
///
/// # Props
/// - `toast_queue`: Signal for the toast notification queue.
///
/// This component periodically cleans up expired toasts and renders active toasts on the screen.
#[component]
pub fn Toasts(toast_queue: Signal<VecDeque<Toast>>) -> Element {
    const TOAST_CLEANUP_INTERVAL_MS: u64 = 300;

    use_future(move || async move {
        loop {
            tokio::time::sleep(Duration::from_millis(TOAST_CLEANUP_INTERVAL_MS)).await;
            if !toast_queue().is_empty() {
                toast_queue.with_mut(|q| {
                    let now = Instant::now();
                    q.retain(|t| now.duration_since(t.created).as_millis() < t.duration_ms as u128);
                });
            }
        }
    });

    rsx! {
        for toast in toast_queue().iter() {
            div { class: "toast toast-middle toast-center z-50",
                div { class: toast.kind.alert_class(), "{toast.message}" }
            }
        }
    }
}
