# MG-CMP-001 Configuration Management Plan (IEC 62304 §8)
* Configuration items: source, `Cargo.lock`, `rust-toolchain.toml`, CI image digest, docs, requirements.json, test data, SOUP versions (exact pins).
* Branching: trunk-based on `main`; short-lived feature branches; protected main (review + CI required).
* Versioning: SemVer; MAJOR → new UDI-DI.
* Change control: every PR references a change request / problem report id (`CR-nnnn` / `PR-nnnn`) in title.
* Baselines: signed git tags `vX.Y.Z`; release evidence bundle archived for device lifetime + 10 years (MDR Art. 10(8)).
