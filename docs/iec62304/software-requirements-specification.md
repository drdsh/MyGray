# MG-SRS-001 Software Requirements Specification (IEC 62304 §5.2)

The normative content is `requirements/requirements.json`
(schema: `requirements/schema/requirements.schema.json`). This document
describes the structure and the review of that file; a rendered table is
produced by `cargo xtask srs-render` for each release.

## Coverage of §5.2.2 content
| 62304 §5.2.2 item | requirements.json area |
|-------------------|------------------------|
| a) functional & capability | SRS-DCM, RDSR, STO, ALR, WEB (category `functional`, `performance`) |
| b) inputs/outputs | SRS-DCM-007, RDSR-002, conformance statement |
| c) interfaces | SRS-DCM-*, SRS-OPS-001 |
| d) alarms, warnings, operator messages | SRS-ALR-*, `../ifu/alert-codes.md` |
| e) security | SRS-SEC-*, SRS-PRV-* |
| f) usability-related | SRS-WEB-003/005, usability spec |
| g) data definition & database | SRS-STO-*, graph schema |
| h) installation & acceptance | SRS-CFG-*, SRS-OPS-* |
| i) operation & maintenance | SRS-OPS-*, SRS-STO-004/005 |
| j) IT-network | SRS-SEC-008/010, installation manual |
| k) user maintenance | backup/restore |
| l) regulatory | SRS-REG-001 |
| risk control measures (§5.2.3) | `implements_risk_controls` field |

## Review (§5.2.6) – checklist
Each requirement: atomic, unambiguous, verifiable (acceptance criteria present),
traceable (`derived_from`), no conflict, risk controls covered.
