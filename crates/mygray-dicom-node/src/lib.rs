//! # mygray-dicom-node — Software item SI-04
//!
//! DICOM Storage SCP (C-ECHO, C-STORE) that *never loses a received object
//! without informing the user* (SRS-DCM-001..006, HAZ-001).
//!
//! ## Durability protocol (ADR-0002)
//! ```text
//!  Modality ──C-STORE-RQ──► scp::c_store
//!                               │ 1. check AE whitelist, disk space   ──fail──► 0xA700/0x0124, alert
//!                               │ 2. journal::Spool::write_durable()
//!                               │      write tmp → fsync(file) → rename → fsync(dir)
//!                               │      + journal record {ingest_id, sha256, sop_uid, state=Received}
//!                               │                                     ──fail──► 0xA700 (modality retries), CRITICAL alert
//!                               │ 3. ONLY NOW: C-STORE-RSP Success (0x0000)
//!                               ▼
//!                         pipeline (async, decoupled from association)
//!                               │ 4. mygray_rdsr::parse  ──fatal──► quarantine (raw kept) + alert
//!                               │ 5. IngestSink::persist (mygray-store) ──fail──► retry w/ backoff, alert
//!                               │ 6. journal state=Persisted
//!                               ▼
//!                         retention: raw spool file kept N days after Persisted
//! ```
//! On startup, [`journal::Journal::recover`] replays every entry not in a
//! terminal state (SRS-DCM-004). Ingest is idempotent on (SOP Instance UID, sha256).
//!
//! The crate does **not** depend on `mygray-store`; it depends on the
//! [`pipeline::IngestSink`] trait which the composition root wires up.

pub mod conformance;
pub mod error;
pub mod journal;
pub mod pipeline;
pub mod quarantine;
pub mod scp;

pub use error::DicomNodeError;
