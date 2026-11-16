# MyGray – Radiation Dose Management Software

Skeleton/architecture repository for MyGray, a standalone medical device software
(EU MDR, proposed class IIa, IEC 62304 class B) that receives DICOM Radiation Dose
Structured Reports, stores them in an embedded Grafeo graph database, evaluates
configurable dose rules and technical errors as alerts, and visualises everything
in a server-side rendered web UI.

> **Status:** architecture skeleton. Types, traits, module layout, test names and
> documentation structure are in place; function bodies are `todo!()` and tests are
> `#[ignore = "skeleton"]`.

## Repository layout
```
Cargo.toml                 workspace, pinned SOUP, lints, profiles
rust-toolchain.toml        pinned compiler
deny.toml                  SOUP licence/advisory policy
crates/
  mygray-core/        SI-01 domain model, typed dose units, error taxonomy (Reportable), audit
  mygray-config/      SI-02 config loading & fail-fast validation
  mygray-rdsr/        SI-03 pure, total, lossless RDSR parser (+ benches/, fuzz/)
  mygray-dicom-node/  SI-04 Storage SCP, durable spool + WAL journal, quarantine, pipeline
  mygray-store/       SI-05 Grafeo graph schema, repositories, migrations, backup, retention
  mygray-alert/       SI-06 alert bus, panic/error capture, rule engine, sinks, escalation
  mygray-web/         SI-07 axum + askama SSR UI, RBAC, health endpoints
apps/mygray-server/   SI-08 composition root (binary `mygray`), adapters, supervisor
tests/system/         system tests (IEC 62304 §5.7)
xtask/                compliance automation: trace, arch-check, soup-check, sbom, release
requirements/         requirements.json (+ JSON schema) – single source of truth
docs/                 MDR technical documentation, IEC 62304, ISO 14971, IEC 62366, security, DICOM CS, IFU
testdata/             anonymised DICOM fixtures, golden outputs
packaging/            build & runtime containers, systemd unit
.github/workflows/    CI gates
```

## Common commands
```sh
cargo check --workspace --all-targets
cargo test --workspace                       # unit + integration (skeleton tests are ignored)
cargo test --workspace -- --ignored          # list pending skeleton tests (they will fail)
cargo xtask trace                            # requirement ⇄ test traceability gate
cargo bench -p mygray-rdsr                   # RDSR parse performance (SRS-RDSR-005)
cargo +nightly fuzz run parse_bytes --fuzz-dir crates/mygray-rdsr/fuzz
```

Start reading at `docs/iec62304/software-architecture.md` and `requirements/requirements.json`.
