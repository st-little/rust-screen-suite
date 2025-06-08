use std::time::Instant;

/// Struct representing a toast notification.
/// Contains the kind, message, duration, and creation time.
#[derive(Clone)]
pub struct Toast {
    pub kind: ToastKind,
    pub message: String,
    pub duration_ms: u64,
    pub created: Instant,
}

/// Enum representing the type of toast notification.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ToastKind {
    Info,
    Success,
    // Warning,
    Error,
}

impl ToastKind {
    /// Returns the CSS class for each toast kind.
    pub fn alert_class(&self) -> &'static str {
        match self {
            ToastKind::Info => "alert alert-info",
            ToastKind::Success => "alert alert-success",
            // ToastKind::Warning => "alert alert-warning",
            ToastKind::Error => "alert alert-error",
        }
    }
}
