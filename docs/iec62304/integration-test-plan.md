# MG-ITP-001 Software Integration & Integration Test Plan (IEC 62304 §5.6)
Integration order (bottom-up): SI-01 → SI-02/SI-03 → SI-05 → SI-06 → SI-04 (with real SI-03, mocked sink) → SI-07 (with SI-05/SI-06) → SI-08 (all real).

| ID | Integration | Test location | SRS |
|----|-------------|---------------|-----|
| IT-01 | SI-04 + SI-03 parse pipeline | `crates/mygray-dicom-node/tests/` | DCM-001…006 |
| IT-02 | SI-05 atomic ingest | `crates/mygray-store/tests/` | STO-002…006 |
| IT-03 | SI-06 rules over SI-05 DoseContext adapter | `apps/mygray-server` tests | ALR-010…013 |
| IT-04 | SI-07 + SI-05 + SI-06 | `crates/mygray-web/tests/` | WEB-*, SEC-* |
Regression: complete integration suite on every PR.
