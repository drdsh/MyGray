//! Domain events passed between software items over in-process channels.
//!
//! Events are the *only* coupling between the DICOM node, the store and the
//! alert engine at runtime (see ADR-0003 "event-driven ingest pipeline").

use serde::{Deserialize, Serialize};

use crate::ids::{IngestId, StudyId};

/// Lifecycle of one received DICOM object.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IngestEvent {
    /// Raw bytes durably journaled; C-STORE success may be sent.
    Journaled(IngestId),
    /// Parsed and validated.
    Parsed(IngestId),
    /// Persisted in the graph store.
    Persisted {
        /// Journal entry.
        ingest: IngestId,
        /// Study it was linked to.
        study: StudyId,
    },
    /// Moved to quarantine (unparseable / invalid). Raw data is retained.
    Quarantined {
        /// Journal entry.
        ingest: IngestId,
        /// Human readable reason (no PHI).
        reason: String,
    },
}

/// Emitted after a study's dose data is complete/updated; input for rule evaluation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DoseDataUpdated {
    /// Updated study.
    pub study: StudyId,
}
