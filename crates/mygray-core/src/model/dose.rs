//! Dose report and irradiation events (normalised from RDSR templates).

use serde::{Deserialize, Serialize};

use crate::ids::{DicomUid, StudyId};
use crate::units::{AirKerma, CtdiVol, Dap, Dlp, FluoroTime, Ssde};

/// Which RDSR root template the report was derived from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReportKind {
    /// TID 10011 CT Radiation Dose.
    Ct,
    /// TID 10001 Projection X-Ray Radiation Dose (XA, RF, DX, MG).
    ProjectionXray,
    /// TID 10021 Radiopharmaceutical Radiation Dose (RRDSR).
    Radiopharmaceutical,
}

/// One dose report (= one RDSR SOP instance).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DoseReport {
    /// SOP Instance UID of the RDSR.
    pub sop_uid: DicomUid,
    /// Study.
    pub study: StudyId,
    /// Kind.
    pub kind: ReportKind,
    /// Accumulated totals.
    pub totals: AccumulatedDose,
    /// Events.
    pub events: Vec<IrradiationEvent>,
}

/// Accumulated values (TID 10012 / TID 10002).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AccumulatedDose {
    /// CT: total DLP.
    pub dlp_total: Option<Dlp>,
    /// Projection: total DAP.
    pub dap_total: Option<Dap>,
    /// Projection: total reference air kerma.
    pub ref_air_kerma_total: Option<AirKerma>,
    /// Projection: total fluoroscopy time.
    pub fluoro_time_total: Option<FluoroTime>,
}

/// One irradiation event (TID 10013 / TID 10003).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IrradiationEvent {
    /// Irradiation Event UID.
    pub event_uid: DicomUid,
    /// Acquisition protocol text.
    pub protocol: Option<String>,
    /// CT: CTDIvol.
    pub ctdi_vol: Option<CtdiVol>,
    /// CT: DLP.
    pub dlp: Option<Dlp>,
    /// CT: SSDE (if reported or computed).
    pub ssde: Option<Ssde>,
    /// Projection: DAP.
    pub dap: Option<Dap>,
    /// Coded values the parser did not map – kept, never discarded (SRS-RDSR-006).
    pub unmapped: Vec<UnmappedContentItem>,
}

/// A content item that is not (yet) understood. Kept verbatim for traceability.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnmappedContentItem {
    /// Concept name code as `scheme:value`.
    pub concept: String,
    /// String rendering of the value.
    pub value: String,
}
