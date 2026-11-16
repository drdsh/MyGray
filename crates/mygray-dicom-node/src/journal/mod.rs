//! Durable ingest journal.
//!
//! * `spool`  – content-addressed raw object storage with crash-safe writes
//! * `wal`    – append-only state log (Received → Parsed → Persisted | Quarantined)

pub mod spool;
pub mod wal;

use mygray_core::ids::IngestId;

/// State machine of one ingested object. Transitions are append-only.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IngestState {
    /// Raw bytes durable; C-STORE success sent.
    Received,
    /// Parsed by SI-03.
    Parsed,
    /// Committed in SI-05. Terminal.
    Persisted,
    /// Moved to quarantine, alert raised. Terminal (until manual reprocess).
    Quarantined,
}

/// Facade over spool + WAL.
pub struct Journal {
    _private: (),
}

impl Journal {
    /// Return all entries not in a terminal state, for replay after crash/restart.
    /// SRS-DCM-004.
    pub fn recover(&self) -> Result<Vec<IngestId>, crate::DicomNodeError> {
        todo!()
    }
}
