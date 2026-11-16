//! Extractor for TID 10011 CT Radiation Dose (+ TID 10012/10013/10014).
//!
//! Requirements: SRS-RDSR-002 (template coverage), SRS-RDSR-006 (lossless).

#[cfg(test)]
mod tests {
    #[test]
    #[ignore = "skeleton"]
    fn srs_rdsr_002_ct_golden_files_match_snapshot() {
        // iterate testdata/rdsr/ct/*.dcm, insta::assert_json_snapshot!
    }
}
