# MG-TM-001 Threat Model (STRIDE per data flow)
| Asset / flow | Threat (STRIDE) | Example | Control (SRS) |
|--------------|-----------------|---------|---------------|
| DICOM port 11112 | Spoofing | rogue AE sends fake RDSR | SRS-SEC-010, DICOM TLS (SEC-008) |
| DICOM port | DoS | malformed PDUs, association flood | SRS-RDSR-004, DCM-009 limits |
| Web UI | Elevation | role bypass | SRS-SEC-003 |
| Web UI | Tampering | CSRF on rule edit | SRS-SEC-009 |
| Store file | Information disclosure | DB file copied | SRS-SEC-007 |
| Audit trail | Repudiation | audit records edited | SRS-SEC-005 |
| Notifications | Information disclosure | PHI in e-mails | SRS-PRV-002 |
| Supply chain | Tampering | malicious crate | cargo-deny sources, pinned versions, SBOM |
