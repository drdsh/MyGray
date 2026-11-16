//! Notification sinks. Each implements `Sink`; the dispatcher tries sinks in
//! configured priority with fallback (SRS-ALR-004).

pub mod email;
pub mod syslog;
pub mod ui;
pub mod webhook;

/// A notification channel.
pub trait Sink: Send + Sync {
    /// Name for diagnostics.
    fn name(&self) -> &'static str;
    /// Deliver. Must time out within the configured bound.
    fn deliver(
        &self,
        alert: &crate::Alert,
    ) -> impl std::future::Future<Output = Result<(), String>> + Send;
}
