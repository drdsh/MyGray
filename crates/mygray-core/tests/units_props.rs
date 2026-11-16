//! Property tests for typed dose quantities.
//! Verifies: SRS-CORE-001 (unit safety), RC-SW-010.

use proptest::prelude::*;

proptest! {
    #[test]
    #[ignore = "skeleton: enable when mygray_core::units is implemented"]
    fn srs_core_001_rejects_non_finite(v in prop::num::f64::ANY) {
        let r = mygray_core::units::CtdiVol::new(v);
        prop_assert_eq!(r.is_ok(), v.is_finite() && v >= 0.0);
    }
}
