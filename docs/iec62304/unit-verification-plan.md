# MG-UVP-001 Software Unit Verification Plan (IEC 62304 §5.5)

## 1 Test layout (convention, enforced by review + xtask)
```
crates/<item>/src/<module>.rs      #[cfg(test)] mod tests { … }   ← unit tests (private API, white-box)
crates/<item>/tests/*.rs           ← crate-level integration tests (public API only)
crates/<item>/benches/*.rs         ← criterion performance tests (SRS-…-perf)
crates/<item>/fuzz/                ← cargo-fuzz targets (not workspace members)
tests/system/tests/stNN_*.rs       ← system tests (black-box, running binary)
testdata/                          ← shared, version-controlled test data (anonymised!)
```

## 2 Naming → traceability
`fn <srs_id_snake>_<behaviour>()` e.g. `srs_dcm_004_unfinished_entries_replayed_after_restart`.
`cargo xtask trace` fails CI if an automatically verifiable SRS has no test.
JUnit output (`cargo nextest --profile ci`) is parsed into the trace matrix with pass/fail per SRS.

## 3 Techniques per item
| Item | Techniques |
|------|------------|
| SI-01 | property tests (proptest) on units/UIDs; serde round-trip |
| SI-02 | table tests (rstest) of invalid configs; temp dirs |
| SI-03 | golden files + insta snapshots; property tests; fuzzing ≥ 24 h per release; criterion benchmark |
| SI-04 | fault injection via filesystem trait (`FaultyFs`), crash simulation, idempotency tests |
| SI-05 | in-memory/temp Grafeo; round-trip properties; atomicity with injected failures; migration from every released schema |
| SI-06 | table-driven rule tests with boundary values; mocked clock (escalation, silence); sink failure mocks |
| SI-07 | router tests with `tower::ServiceExt::oneshot`; HTML snapshots; RBAC matrix test (all roles × routes) |
| SI-08 | startup/shutdown order tests; supervisor restart |

## 4 Acceptance criteria (§5.5.3)
* All tests pass; no ignored tests in release builds (skeleton `#[ignore]` must be gone — `xtask ci --release` checks).
* Coverage (llvm-cov, line / branch): class B items ≥ 85 % / 75 %; SI-03 and SI-04 journal ≥ 95 % line.
* clippy with workspace lints: zero warnings. `cargo deny`: clean.
* Code review per PR checklist documented in PR.
