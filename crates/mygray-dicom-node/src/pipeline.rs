//! Asynchronous ingest pipeline: spool → parse → persist.
//!
//! Decoupled from the DICOM association so slow storage never causes
//! modality time-outs. Backpressure via bounded channel; if the channel is
//! full the object is still safe in the spool and will be picked up by the
//! recovery sweep.

use mygray_core::ids::{IngestId, StudyId};
use mygray_core::model::dose::DoseReport;

/// Port to the persistence layer (implemented by an adapter around `mygray-store`
/// in the composition root). Keeps SI-04 independent of SI-05.
pub trait IngestSink: Send + Sync {
    /// Idempotently persist a parsed report. Same `ingest` twice ⇒ no duplicate.
    fn persist(
        &self,
        ingest: IngestId,
        report: &DoseReport,
    ) -> impl std::future::Future<Output = Result<StudyId, Box<dyn mygray_core::Reportable>>> + Send;
}

/// Port to the alert system (implemented by `mygray-alert::AlertBus`).
pub trait AlertPort: Send + Sync {
    /// Raise an alert for an error.
    fn raise(&self, err: &dyn mygray_core::Reportable);
}
