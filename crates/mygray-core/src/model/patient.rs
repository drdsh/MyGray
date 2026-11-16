//! Patient entity. PHI fields are isolated in [`PatientDemographics`] so they
//! can be encrypted/pseudonymised independently (SRS-SEC-007, GDPR Art. 25).

use serde::{Deserialize, Serialize};

use crate::ids::PatientId;

/// Patient sex as coded in DICOM (0010,0040).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Sex {
    /// M
    Male,
    /// F
    Female,
    /// O
    Other,
    /// absent / empty
    Unknown,
}

/// Non-identifying patient data relevant for dose assessment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Patient {
    /// Internal key.
    pub id: PatientId,
    /// Sex.
    pub sex: Sex,
    /// Birth year only (data minimisation) – age at exam is stored per study.
    pub birth_year: Option<i32>,
}

/// Identifying data. Stored encrypted; access is audited.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientDemographics {
    /// Patient ID (0010,0020) + Issuer (0010,0021).
    pub mrn: String,
    /// Issuer of patient ID.
    pub issuer: Option<String>,
    /// Patient name (0010,0010).
    pub name: String,
}
