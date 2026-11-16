//! # mygray-store — Software item SI-05
//!
//! Embedded Grafeo graph database. All access goes through typed
//! repositories; **no GQL strings outside `query.rs`** (reviewable, testable,
//! injection-safe via parameters only — SRS-SEC-011).
//!
//! ## Graph model (`schema/`)
//! ```text
//! (:Patient)-[:HAS_STUDY]->(:Study)-[:HAS_SERIES]->(:Series)-[:HAS_INSTANCE]->(:Instance)
//! (:Study)-[:PERFORMED_ON]->(:Device)-[:LOCATED_IN]->(:Site)
//! (:Study)-[:HAS_DOSE_REPORT]->(:DoseReport)-[:HAS_EVENT]->(:IrradiationEvent)
//! (:IrradiationEvent)-[:USES_PROTOCOL]->(:Protocol)-[:MAPPED_TO]->(:StandardProtocol)
//! (:StandardProtocol)-[:HAS_DRL]->(:ReferenceLevel)
//! (:AlertRule)-[:VERSION_OF]->(:AlertRule)
//! (:Alert)-[:TRIGGERED_BY]->(:AlertRule) ; (:Alert)-[:CONCERNS]->(:Study|:Device|:Patient)
//! (:User)-[:HAS_ROLE]->(:Role) ; (:User)-[:ACKNOWLEDGED {at}]->(:Alert)
//! (:AuditRecord)-[:PRECEDED_BY]->(:AuditRecord)      // hash chain
//! (:IngestRecord {ingest_id, sha256})-[:PRODUCED]->(:DoseReport)
//! ```
//! A graph fits because dose management is relationship-heavy: cumulative
//! patient dose across devices, protocol harmonisation across sites, and
//! alert provenance (ADR-0004).

pub mod backup;
pub mod error;
pub mod migrations;
pub mod query;
pub mod repo;
pub mod retention;
pub mod schema;
pub mod tx;

pub use error::StoreError;

/// Handle to the database. Cheap to clone; internally synchronised.
#[derive(Clone)]
pub struct Store {
    _private: (),
}

impl Store {
    /// Open (or create) the database, run pending migrations, verify schema version.
    /// SRS-STO-001, SRS-STO-005.
    pub fn open(_cfg: &mygray_config::StoreConfig) -> Result<Self, StoreError> {
        todo!()
    }
}
