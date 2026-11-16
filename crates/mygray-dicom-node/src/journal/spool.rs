//! Crash-safe raw object storage.
//!
//! Write protocol: `tmp/<uuid>` → `fsync` → `rename` to `objects/<sha256[0..2]>/<sha256>`
//! → `fsync(parent dir)`. Content-addressed ⇒ duplicate sends are free and
//! integrity is verifiable at any time (SRS-DCM-002).

use mygray_core::ids::IngestId;

/// Raw object spool.
pub struct Spool {
    _root: std::path::PathBuf,
}

impl Spool {
    /// Durably store `bytes`; returns only after data and directory entry are fsynced.
    pub fn write_durable(
        &self,
        _bytes: &[u8],
    ) -> Result<(IngestId, [u8; 32]), crate::DicomNodeError> {
        todo!("SRS-DCM-001/002")
    }
}

#[cfg(test)]
mod tests {
    #[test]
    #[ignore = "skeleton"]
    fn srs_dcm_002_hash_verified_on_read() {}

    #[test]
    #[ignore = "skeleton"]
    fn srs_dcm_001_no_partial_file_visible_after_simulated_crash() {}
}
