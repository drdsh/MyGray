# MG-DD-SI-04 Detailed Design – SI-04-dicom-node (`mygray-dicom-node`)

> Class B: detailed design required for each software unit (IEC 62304 §5.4.2/5.4.3).
> API-level design is maintained as rustdoc (`cargo doc -p mygray-dicom-node`); this document
> covers what rustdoc cannot: rationale, state machines, error handling, units.

## 1 Responsibilities & requirements (SRS ids)
## 2 Software units (modules) and their interfaces
## 3 State machines / sequence diagrams
## 4 Error handling: error enum → alert codes → severity
## 5 Concurrency & resource limits
## 6 Unit verification: test files, acceptance criteria (→ unit-verification-plan.md)

## Appendix A – Ingest state machine
```
            write_durable ok                parse ok             persist ok
 (none) ───────────────────► Received ──────────────► Parsed ───────────────► Persisted*
    │ write fails                │ parse fatal          │ persist fails (n retries)
    ▼                            ▼                      ▼
 C-STORE A700 + CRITICAL    Quarantined* + alert   stays Parsed, DCM-0006 CRITICAL, retried by sweep
                                                                           (* terminal)
```
