//! # mygray-alert — Software item SI-06
//!
//! Two kinds of alert inputs, one pipeline:
//!
//! ```text
//!  technical sources                       clinical/dose sources
//!  ─────────────────                       ─────────────────────
//!  Reportable errors (all items)           DoseDataUpdated events
//!  panic hook                              scheduled rules (cumulative, device silence)
//!  watchdog / heartbeats                          │
//!  tracing ERROR layer                     rules::engine (configurable rules)
//!         │                                       │
//!         └──────────────► AlertBus ◄─────────────┘
//!                             │ dedup / rate-limit / escalation
//!                             ▼
//!                 persist (AlertRepository)  ──►  sinks: UI, e-mail, webhook, syslog
//! ```
//!
//! ## Safety properties
//! * SRS-ALR-001: every `Reportable` error reaches the bus.
//! * SRS-ALR-002: panics in any task are captured (panic hook) and alerted.
//! * SRS-ALR-004: if a sink fails, fallback to the next sink; if *all* sinks
//!   fail the system health turns RED (visible in UI + `/health`).
//! * SRS-ALR-005: the alert system itself is supervised (heartbeat); its
//!   failure is detectable from outside (watchdog / health endpoint).
//! * SRS-ALR-003: alerts are persisted before notification ⇒ none lost on crash.

pub mod ack;
pub mod bus;
pub mod escalation;
pub mod rules;
pub mod sinks;
pub mod sources;

pub use bus::{Alert, AlertBus};
