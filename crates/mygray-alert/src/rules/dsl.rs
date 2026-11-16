//! Rule definition types (serde) + validation (unit compatibility, threshold
//! sanity, scope references exist). Invalid rules are rejected on save, never
//! partially applied (SRS-ALR-011).

use serde::{Deserialize, Serialize};

/// Measurable quantity a rule can test.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Metric {
    /// Per-event CTDIvol.
    EventCtdiVol,
    /// Per-event DLP.
    EventDlp,
    /// Per-study total DLP.
    StudyDlpTotal,
    /// Per-study total DAP.
    StudyDapTotal,
    /// Per-study reference air kerma.
    StudyRefAirKerma,
    /// Per-study fluoroscopy time.
    StudyFluoroTime,
    /// SSDE.
    EventSsde,
    /// Patient cumulative (window) – requires `window`.
    PatientCumulativeDlp,
    /// Patient cumulative reference air kerma (skin dose follow-up).
    PatientCumulativeRefAirKerma,
    /// Ratio to applicable DRL (e.g. 1.0 = at DRL).
    DrlRatio,
}

/// Comparator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Op {
    /// >
    #[serde(rename = ">")]
    Gt,
    /// >=
    #[serde(rename = ">=")]
    Ge,
    /// <
    #[serde(rename = "<")]
    Lt,
}
