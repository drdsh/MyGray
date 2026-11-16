# MG-VV-001 Verification & Validation Overview
| Level | Plan | Executed by | Evidence |
|-------|------|-------------|----------|
| Unit (IEC 62304 §5.5) | `../../iec62304/unit-verification-plan.md` | `cargo test` (CI) | JUnit, coverage |
| Integration (§5.6) | `../../iec62304/integration-test-plan.md` | `cargo test -p <crate> --test '*'` | JUnit |
| System (§5.7) | `../../iec62304/system-test-plan.md` | `tests/system` + manual protocols | JUnit + signed protocols |
| Usability (IEC 62366-1) | `../../usability/summative-evaluation-plan.md` | usability team | report |
| Cybersecurity (IEC 81001-5-1 §5.7) | `../../cybersecurity/security-plan.md` | external pentest | report |
| Clinical validation | `clinical-evaluation-plan.md` | MPE at pilot sites | CER |
