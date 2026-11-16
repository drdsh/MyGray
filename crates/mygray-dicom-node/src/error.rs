//! Error type for SI-04.

use mygray_core::{AlertCode, Reportable, Severity};

/// DICOM node errors.
#[derive(Debug, thiserror::Error)]
pub enum DicomNodeError {
    /// Durable write to spool failed → object NOT accepted, sender informed via status.
    #[error("spool write failed: {0}")]
    SpoolWrite(#[source] std::io::Error),
    /// Free disk space below threshold.
    #[error("insufficient disk space")]
    DiskSpace,
    /// Calling AE not whitelisted.
    #[error("association rejected for calling AE {0}")]
    AeRejected(String),
    /// Network / upper-layer protocol error.
    #[error("association error: {0}")]
    Association(String),
    /// Journal corruption detected at startup.
    #[error("journal integrity failure: {0}")]
    JournalIntegrity(String),
    /// Downstream persistence failed after retries; raw data still in spool.
    #[error("persist failed after retries for ingest {0:?}")]
    PersistExhausted(mygray_core::ids::IngestId),
}

impl Reportable for DicomNodeError {
    fn code(&self) -> AlertCode {
        AlertCode(match self {
            Self::SpoolWrite(_) => "DCM-0001",
            Self::DiskSpace => "DCM-0002",
            Self::AeRejected(_) => "DCM-0003",
            Self::Association(_) => "DCM-0004",
            Self::JournalIntegrity(_) => "DCM-0005",
            Self::PersistExhausted(_) => "DCM-0006",
        })
    }
    fn severity(&self) -> Severity {
        match self {
            Self::AeRejected(_) | Self::Association(_) => Severity::Warning,
            Self::DiskSpace => Severity::Error,
            _ => Severity::Critical,
        }
    }
    fn risks_data_loss(&self) -> bool {
        matches!(
            self,
            Self::SpoolWrite(_) | Self::JournalIntegrity(_) | Self::PersistExhausted(_)
        )
    }
}
