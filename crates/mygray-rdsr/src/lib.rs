//! # mygray-rdsr — Software item SI-03
//!
//! Fast parser for DICOM Radiation Dose Structured Reports.
//!
//! ## Design constraints (see docs/iec62304/detailed-design/SI-03-rdsr.md)
//! * **Pure & total**: `parse()` never panics for any input (fuzzed, SRS-RDSR-004).
//! * **Lossless**: every content item is either mapped to the normalised model
//!   or retained in `unmapped` (SRS-RDSR-006). Nothing is silently dropped.
//! * **Diagnostics, not exceptions**: non-fatal deviations (vendor quirks,
//!   missing optional items) are collected as [`Diagnostic`]s and returned
//!   together with the result.
//! * **Fast**: single pass over the content tree, no re-reading; target
//!   p95 < 5 ms for a 500-event CT RDSR (SRS-RDSR-005, `benches/parse.rs`).
//!
//! ## Pipeline
//! ```text
//! bytes ─► dicom-object ─► content_tree::walk ─► templates::{ct,projection,rp}
//!                                                     │
//!                                   units::convert ◄──┘
//!                                         │
//!                                   validate::check ─► ParseOutcome
//! ```

pub mod codes;
pub mod content_tree;
pub mod templates;
pub mod units;
pub mod validate;
pub mod vendor;

use mygray_core::model::dose::DoseReport;

/// A non-fatal finding during parsing.
#[derive(Debug, Clone, PartialEq)]
pub struct Diagnostic {
    /// Stable code, e.g. `RDSR-W012`.
    pub code: &'static str,
    /// Content-item path (e.g. `1.4.2`) where the issue was found.
    pub path: String,
    /// Human readable message (no PHI).
    pub message: String,
}

/// Result of a successful parse.
#[derive(Debug, Clone)]
pub struct ParseOutcome {
    /// Normalised report.
    pub report: DoseReport,
    /// Non-fatal diagnostics.
    pub diagnostics: Vec<Diagnostic>,
}

/// Fatal parse errors. The caller (SI-04) MUST quarantine the raw object,
/// never discard it (SRS-DCM-006).
#[derive(Debug, thiserror::Error)]
pub enum RdsrError {
    /// Not a DICOM SR / not decodable.
    #[error("not a decodable DICOM SR: {0}")]
    Decode(String),
    /// SR, but no supported root template (TID 10001/10011/10021).
    #[error("unsupported root template: {0}")]
    UnsupportedTemplate(String),
    /// Mandatory content missing (e.g. no Irradiation Event UID).
    #[error("mandatory content missing at {path}: {what}")]
    MissingMandatory {
        /// Content-item path.
        path: String,
        /// Missing concept.
        what: String,
    },
    /// Implausible value (negative dose, unknown unit).
    #[error("implausible value at {path}: {reason}")]
    Implausible {
        /// Content-item path.
        path: String,
        /// Reason.
        reason: String,
    },
}

impl mygray_core::Reportable for RdsrError {
    fn code(&self) -> mygray_core::AlertCode {
        mygray_core::AlertCode(match self {
            Self::Decode(_) => "RDSR-0001",
            Self::UnsupportedTemplate(_) => "RDSR-0002",
            Self::MissingMandatory { .. } => "RDSR-0003",
            Self::Implausible { .. } => "RDSR-0004",
        })
    }
    fn severity(&self) -> mygray_core::Severity {
        mygray_core::Severity::Error
    }
}

/// Parse an RDSR from raw Part-10 bytes. Never panics.
pub fn parse_bytes(_bytes: &[u8]) -> Result<ParseOutcome, RdsrError> {
    todo!("SRS-RDSR-001..006")
}
