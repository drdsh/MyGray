//! System-wide error taxonomy (SRS-ALR-001: *every* error is reportable).
//!
//! Each software item defines its own `thiserror` enum and implements
//! [`Reportable`] for it. The alert system (SI-06) consumes `&dyn Reportable`
//! and never needs to know concrete error types — this keeps the dependency
//! direction `item -> core <- alert` and avoids cycles.

use serde::{Deserialize, Serialize};

/// Severity of an alert. Ordering is meaningful (`Info < … < Critical`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Severity {
    /// Informational, no action required.
    Info,
    /// Degraded operation, action recommended.
    Warning,
    /// Function failed; user action required.
    Error,
    /// Potential data loss or patient-safety relevant; immediate action.
    Critical,
}

/// Stable, documented alert code (e.g. `DCM-0003`). Codes are listed in
/// `docs/ifu/alert-codes.md` and never reused once released.
/// Serialize-only: codes are compiled-in constants; persisted alerts are read back as strings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
pub struct AlertCode(pub &'static str);

/// Anything that can be turned into an alert.
pub trait Reportable: std::error::Error + Send + Sync + 'static {
    /// Stable alert code.
    fn code(&self) -> AlertCode;
    /// Severity of this occurrence.
    fn severity(&self) -> Severity;
    /// True if this error implies a received object may not be persisted.
    /// Such errors MUST escalate to `Critical` (SRS-DCM-003, HAZ-001).
    fn risks_data_loss(&self) -> bool {
        false
    }
}
