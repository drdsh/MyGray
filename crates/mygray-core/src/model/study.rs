//! Study entity.

use serde::{Deserialize, Serialize};

use crate::ids::{DeviceId, DicomUid, PatientId, StudyId};

/// One imaging study.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Study {
    /// Internal key.
    pub id: StudyId,
    /// Study Instance UID (0020,000D) — natural key, unique.
    pub study_uid: DicomUid,
    /// Owning patient.
    pub patient: PatientId,
    /// Performing device.
    pub device: Option<DeviceId>,
    /// Accession number (0008,0050).
    pub accession: Option<String>,
    /// Study date/time.
    pub performed_at: Option<time::OffsetDateTime>,
    /// Study description (0008,1030).
    pub description: Option<String>,
    /// Patient age at exam, years.
    pub patient_age_years: Option<u16>,
    /// Patient weight (kg) at exam — needed for SSDE / paediatric rules.
    pub patient_weight_kg: Option<f64>,
}
