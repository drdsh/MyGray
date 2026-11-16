//! # mygray-core — Software item SI-01
//!
//! Shared, dependency-light foundation for all other software items:
//!
//! * [`model`]  – domain entities (patient, study, series, device, user, dose events)
//! * [`units`]  – strongly typed dose quantities (no bare `f64` for doses, RC-SW-010)
//! * [`error`]  – the system-wide error taxonomy; every error is [`error::Reportable`]
//!   so it can be routed to the alert system (SRS-ALR-001)
//! * [`audit`]  – audit trail record types (SRS-SEC-004)
//! * [`event`]  – domain events exchanged between items over the internal bus
//! * [`ids`]    – typed identifiers (DICOM UIDs, internal UUIDs)
//!
//! This crate contains **no I/O**. It is pure and fully unit testable.
//!
//! Requirements: see `requirements/requirements.json`, component `mygray-core`.

pub mod audit;
pub mod error;
pub mod event;
pub mod ids;
pub mod model;
pub mod units;

pub use error::{AlertCode, Reportable, Severity};
