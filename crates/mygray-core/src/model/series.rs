//! Series entity.

use serde::{Deserialize, Serialize};

use crate::ids::{DicomUid, SeriesId, StudyId};

/// One series within a study.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Series {
    /// Internal key.
    pub id: SeriesId,
    /// Series Instance UID (0020,000E).
    pub series_uid: DicomUid,
    /// Parent study.
    pub study: StudyId,
    /// Modality (0008,0060), e.g. "CT", "SR", "XA".
    pub modality: String,
    /// Series description.
    pub description: Option<String>,
}
