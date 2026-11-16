//! Robustness tests (SRS-RDSR-004): parser is total – no panic for any input.

use proptest::prelude::*;

proptest! {
    #![proptest_config(ProptestConfig::with_cases(10_000))]
    #[test]
    #[ignore = "skeleton"]
    fn srs_rdsr_004_never_panics_on_arbitrary_bytes(bytes in prop::collection::vec(any::<u8>(), 0..65536)) {
        let _ = mygray_rdsr::parse_bytes(&bytes);
    }
}

#[test]
#[ignore = "skeleton"]
fn srs_rdsr_004_corrupted_corpus_returns_error_not_panic() {
    // iterate testdata/rdsr/corrupted/*
}
