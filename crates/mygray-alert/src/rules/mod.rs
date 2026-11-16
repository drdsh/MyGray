//! Configurable dose rules.
//!
//! Rules are **data**, not code: stored versioned in SI-05, edited in the UI
//! by role `Physicist`, every change audited and producing a new immutable
//! version (SRS-ALR-010). The DSL is deliberately *not* Turing complete:
//! a rule is a scope filter + metric + comparator + threshold + window.
//!
//! Example (JSON form, see `examples/rules.json`):
//! ```json
//! { "id": "ct-head-ctdi-notif", "version": 3, "enabled": true,
//!   "scope": { "modality": "CT", "standard_protocol": "HEAD_ROUTINE", "age_years": {"min": 18} },
//!   "metric": "event.ctdi_vol", "op": ">", "threshold": { "value": 60, "unit": "mGy" },
//!   "severity": "Warning", "notify": ["role:Physicist"] }
//! ```

pub mod builtin;
pub mod dsl;
pub mod engine;

/// Read-only data access the engine needs (implemented over mygray-store).
pub trait DoseContext: Send + Sync {
    /// Cumulative value of a metric for a patient over a time window.
    fn cumulative(
        &self,
        patient: mygray_core::ids::PatientId,
        metric: dsl::Metric,
        window: time::Duration,
    ) -> Result<f64, Box<dyn mygray_core::Reportable>>;
}
