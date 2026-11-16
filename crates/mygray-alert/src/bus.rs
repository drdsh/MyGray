//! The alert bus: multi-producer, bounded, never blocks producers on the
//! hot path (DICOM ingest). Overflow increments a counter and raises a single
//! aggregated `ALR-0002 alerts dropped` critical alert — loss is never silent.

use mygray_core::ids::AlertId;
use mygray_core::{AlertCode, Severity};
use serde::{Deserialize, Serialize};

/// Origin of an alert.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertSource {
    /// A technical error from a software item.
    Technical {
        /// Software item id, e.g. `SI-04`.
        item: String,
    },
    /// A configured dose rule fired.
    Rule {
        /// Rule id.
        rule_id: String,
        /// Rule version (immutable, audited).
        rule_version: u32,
    },
    /// Internal supervision (watchdog, heartbeat).
    Supervision,
}

/// One alert instance.
#[derive(Debug, Clone, Serialize)]
pub struct Alert {
    /// Id.
    pub id: AlertId,
    /// Code.
    pub code: AlertCode,
    /// Severity.
    pub severity: Severity,
    /// Origin.
    pub source: AlertSource,
    /// Message (no PHI in technical alerts; dose alerts reference study id only).
    pub message: String,
    /// Raised at.
    pub raised_at: time::OffsetDateTime,
    /// Dedup key: same key within the window ⇒ counter++ instead of new alert.
    pub dedup_key: String,
}

/// Sending half; cloned into every item.
#[derive(Clone)]
pub struct AlertBus {
    _private: (),
}

impl AlertBus {
    /// Report any error. Never fails, never blocks.
    pub fn report(&self, _err: &dyn mygray_core::Reportable) {
        todo!("SRS-ALR-001")
    }
    /// Raise a fully formed alert (used by the rule engine).
    pub fn raise(&self, _alert: Alert) {
        todo!()
    }
}

/// Convenience: escalate data-loss-risk errors to Critical regardless of declared severity.
pub fn effective_severity(err: &dyn mygray_core::Reportable) -> Severity {
    if err.risks_data_loss() {
        Severity::Critical
    } else {
        err.severity()
    }
}
