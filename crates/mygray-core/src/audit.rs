//! Audit trail record types (SRS-SEC-004, GDPR Art. 30/32, IHE ATNA semantics).
//!
//! Records are append-only. Persistence is done by `mygray-store`; this module
//! only defines the immutable record shape.

use serde::{Deserialize, Serialize};

use crate::ids::UserId;

/// Who caused an auditable action.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Actor {
    /// Authenticated human user.
    User(UserId),
    /// A remote DICOM application entity.
    RemoteAe {
        /// Calling AE title.
        ae_title: String,
        /// Peer address.
        peer: String,
    },
    /// The system itself (scheduled job, recovery, rule engine).
    System(String),
}

/// What happened.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuditAction {
    /// Object received over DICOM.
    DicomReceived,
    /// Patient data viewed.
    PatientRecordViewed,
    /// Data exported.
    Export,
    /// Login success / failure.
    Login {
        /// Whether authentication succeeded.
        success: bool,
    },
    /// Configuration or alert rule changed.
    ConfigChanged,
    /// Alert acknowledged.
    AlertAcknowledged,
}

/// One immutable audit entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditRecord {
    /// When.
    pub at: time::OffsetDateTime,
    /// Who.
    pub actor: Actor,
    /// What.
    pub action: AuditAction,
    /// On which object (opaque reference).
    pub object_ref: Option<String>,
    /// Hash of previous record → tamper-evident chain (SRS-SEC-005).
    pub prev_hash: [u8; 32],
}
