# MG-SDP-001 Software Development Plan (IEC 62304 §5.1)

## 1 Scope
MyGray, all software items SI-01…SI-08, safety class B.

## 2 Life cycle model
Iterative/incremental (2-week increments) with formal baselines at design
reviews DR-1…DR-4 (see `../mdr/03-design-manufacturing/design-overview.md`).
Every change flows: **Change request → requirement update (requirements.json)
→ risk analysis update → implementation (PR) → review → verification → merge**.

## 3 Deliverables per activity (62304 clause → artefact)
| Clause | Activity | Artefact |
|--------|----------|----------|
| 5.1 | Planning | this document, `configuration-management-plan.md`, `maintenance-plan.md` |
| 5.2 | Requirements analysis | `requirements/requirements.json`, `software-requirements-specification.md` |
| 5.3 | Architectural design | `software-architecture.md`, `soup-list.md`, ADRs |
| 5.4 | Detailed design | `detailed-design/SI-*.md` + rustdoc (`cargo doc`) |
| 5.5 | Unit implementation & verification | code, `unit-verification-plan.md`, unit test results, coverage |
| 5.6 | Integration & integration testing | `integration-test-plan.md`, results |
| 5.7 | System testing | `system-test-plan.md`, results |
| 5.8 | Release | `release-procedure.md`, release record, evidence bundle |
| 6 | Maintenance | `maintenance-plan.md` |
| 7 | Risk management | `../mdr/05-risk-management/` |
| 8 | Configuration management | `configuration-management-plan.md` |
| 9 | Problem resolution | `problem-resolution-process.md` |

## 4 Standards, methods, tools (validated per ISO 13485 §4.1.6)
| Tool | Purpose | Version control | Validation |
|------|---------|-----------------|------------|
| rustc / cargo | compile, build | `rust-toolchain.toml` | compiler test suite + our tests (low risk) |
| clippy, rustfmt | static analysis, style | toolchain | rule set in `Cargo.toml [workspace.lints]` |
| cargo-deny | SOUP licence/advisories | pinned in CI image | sample violation test |
| cargo-llvm-cov | coverage | pinned | sample project |
| cargo-fuzz | robustness | pinned | known-crash corpus |
| cargo-cyclonedx | SBOM | pinned | manual check of sample |
| xtask | traceability, gates | in repo | own unit tests |
| Git + forge | CM, reviews | – | – |

## 5 Coding standard
Rust 2024 edition, lints as defined in workspace; additional rules:
no `unsafe`; no `unwrap/expect/panic` outside tests; errors via `thiserror` +
`Reportable`; no bare `f64` for doses; no GQL outside `mygray-store::query`;
public items documented. Reviewed in PR checklist (`../templates/pr-checklist.md`).

## 6 Definition of Done (per change)
- requirement(s) updated / linked · tests named after SRS id · `cargo xtask ci` green
- risk analysis reviewed · docs updated · PR reviewed by ≥1 independent reviewer
