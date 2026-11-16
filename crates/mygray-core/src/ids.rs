//! Typed identifiers.
//!
//! DICOM UIDs are validated on construction (PS3.5 §9.1: max 64 chars,
//! digits and dots, no leading zeros in components). Internal IDs are UUIDv7
//! (time-ordered, good for graph index locality).

use serde::{Deserialize, Serialize};

/// A validated DICOM UID (e.g. StudyInstanceUID, SOPInstanceUID).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DicomUid(String);

/// Error returned when a string is not a valid DICOM UID.
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum UidError {
    /// Empty or longer than 64 characters.
    #[error("UID length {0} out of range 1..=64")]
    Length(usize),
    /// Contains a character other than `0-9` or `.`.
    #[error("UID contains invalid character")]
    Charset,
    /// A component has a leading zero or is empty.
    #[error("UID component malformed")]
    Component,
}

impl DicomUid {
    /// Parse and validate a UID. SRS-DCM-011.
    pub fn parse(_s: &str) -> Result<Self, UidError> {
        todo!("SRS-DCM-011: validate per PS3.5 §9.1")
    }

    /// Borrow as `&str`.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

macro_rules! internal_id {
    ($(#[$m:meta])* $name:ident) => {
        $(#[$m])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
        pub struct $name(pub uuid::Uuid);
    };
}

internal_id!(
    /// Internal patient identity (pseudonymous key, never the MRN).
    PatientId
);
internal_id!(
    /// Internal study identity.
    StudyId
);
internal_id!(
    /// Internal series identity.
    SeriesId
);
internal_id!(
    /// Internal device (modality) identity.
    DeviceId
);
internal_id!(
    /// Internal user identity.
    UserId
);
internal_id!(
    /// Identity of one received object in the ingest journal.
    IngestId
);
internal_id!(
    /// Identity of a raised alert.
    AlertId
);

#[cfg(test)]
mod tests {
    //! Unit tests. Naming convention: `<requirement id>_<behaviour>` so that
    //! `cargo xtask trace` can link test results to requirements.
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("1.2.840.10008.5.1.4.1.1.88.67", true)]
    #[case("1.2.03", false)] // leading zero
    #[case("", false)]
    #[case("1..2", false)]
    #[ignore = "skeleton"]
    fn srs_dcm_011_uid_validation(#[case] input: &str, #[case] ok: bool) {
        assert_eq!(DicomUid::parse(input).is_ok(), ok);
    }
}
