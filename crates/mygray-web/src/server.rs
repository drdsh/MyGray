//! Router assembly and server start (TLS via rustls unless `--dev`).
//!
//! ```text
//! GET  /                       dashboard (alerts, ingest status, KPIs)
//! GET  /studies?filter…        study list
//! GET  /studies/{id}           study detail: events, CTDIvol/DLP/DAP, DRL ratio
//! GET  /patients/{id}          cumulative dose timeline   (PHI – audited)
//! GET  /devices                devices, last contact, silence status
//! GET  /devices/{id}/stats     protocol stats, DRL comparison
//! GET  /alerts                 alert list ; POST /alerts/{id}/ack
//! GET  /rules ; POST /rules    rule management (Physicist)
//! GET  /admin/users …          user admin (Administrator)
//! GET  /admin/ingest           journal / quarantine view, reprocess
//! GET  /audit                  audit trail (Auditor)
//! GET  /health/live, /health/ready   probes (no auth, no data)
//! GET  /metrics                Prometheus (internal network only)
//! ```
