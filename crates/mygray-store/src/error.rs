//! Store errors.

use mygray_core::{AlertCode, Reportable, Severity};

/// Persistence errors.
#[derive(Debug, thiserror::Error)]
pub enum StoreError {
    /// Underlying engine error.
    #[error("graph engine error: {0}")]
    Engine(String),
    /// Uniqueness violated (e.g. two studies with same StudyInstanceUID but different patient).
    #[error("conflict: {0}")]
    Conflict(String),
    /// Schema version on disk is newer than the software supports.
    #[error("schema version {found} unsupported (max {supported})")]
    SchemaVersion {
        /// On disk.
        found: u32,
        /// Supported.
        supported: u32,
    },
    /// Not found.
    #[error("not found")]
    NotFound,
}

impl Reportable for StoreError {
    fn code(&self) -> AlertCode {
        AlertCode(match self {
            Self::Engine(_) => "STO-0001",
            Self::Conflict(_) => "STO-0002",
            Self::SchemaVersion { .. } => "STO-0003",
            Self::NotFound => "STO-0004",
        })
    }
    fn severity(&self) -> Severity {
        match self {
            Self::NotFound => Severity::Info,
            Self::Conflict(_) => Severity::Warning,
            _ => Severity::Critical,
        }
    }
}
