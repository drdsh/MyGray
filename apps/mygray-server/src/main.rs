//! # MyGray service binary (SI-08, composition root)
//!
//! Startup order (each step fails fast with a Critical alert to syslog/stderr
//! when the alert bus is not yet up):
//! 1. load + validate config                    (SRS-CFG-001)
//! 2. init tracing, start AlertBus + panic hook (SRS-ALR-002) — before anything else can fail silently
//! 3. open Store, run migrations                (SRS-STO-001/005)
//! 4. recover ingest journal, replay pending    (SRS-DCM-004)
//! 5. start rule engine, heartbeat supervisor   (SRS-ALR-005)
//! 6. start DICOM SCP                           (SI-04)
//! 7. start web server                          (SI-07)
//! 8. wait for SIGTERM → graceful shutdown: stop accepting associations,
//!    drain pipeline, flush journal, close store.

mod adapters;
mod supervisor;

fn main() {
    // Skeleton: see module docs for the startup sequence.
}
