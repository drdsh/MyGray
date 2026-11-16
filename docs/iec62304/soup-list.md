# MG-SOUP-001 SOUP List (IEC 62304 §5.3.3, §5.3.4, §7.1.2, §8.1.2)

One row per entry in `[workspace.dependencies]` (checked by `cargo xtask soup-check`).
Transitive dependencies are listed in the generated SBOM and assessed in bulk
via cargo-deny advisories; top-level SOUP is assessed individually below.

| SOUP | Version | Used by | Purpose | Functional/perf. requirements on SOUP | Known anomalies review | Risk (hazards) | Contains unsafe |
|------|---------|---------|---------|----------------------------------------|------------------------|----------------|:---:|
| tokio | 1.54.1 | SI-04,06,07,08 | async runtime | fair scheduling, no task loss | GitHub issues reviewed YYYY-MM-DD | HAZ-001/002 | yes |
| dicom-object / core / ul / dictionary-std | 0.10.1 | SI-03, SI-04 | DICOM parsing & network | correct PS3.5 decoding; no panic on malformed input (fuzzed by us) | | HAZ-001/003/007 | minimal |
| grafeo | 0.6.3 | SI-05 | embedded graph DB | ACID transactions, durability after commit, crash recovery | | HAZ-001/005 | yes |
| axum / tower-http | 0.8.9 / 0.7.1 | SI-07 | HTTP server | | | HAZ-006 | – |
| askama | 0.16.1 | SI-07 | compile-time templates, auto-escaping | HTML escaping by default | | HAZ-006 | – |
| argon2 | 0.6.0 | SI-07 | password hashing | | | HAZ-006 | – |
| serde / serde_json | 1.0.x | all | (de)serialisation | | | HAZ-003 | – |
| figment | 0.10.19 | SI-02 | config merge | | | – | – |
| sha2 | 0.11.1 | SI-04 | integrity hashes | | | HAZ-001 | – |
| thiserror, tracing, tracing-subscriber, uuid, time, secrecy | … | all | infrastructure | | | – | – |
| *test-only:* proptest, rstest, insta, criterion, tempfile | … | tests | verification tooling | – | – | – | – |

Required system hardware/software (§5.3.4): Linux x86_64 (glibc or musl), local filesystem honouring fsync (ext4/xfs), ≥ 4 cores, 8 GB RAM, disk sized per installation manual; supported browsers: current Chrome/Edge/Firefox ESR.
