# MG-STP-001 Software System Test Plan (IEC 62304 §5.7)
Environment: reference installation (container image + reference HW profile), test SCU, browser automation (optional), fault injection hooks.

| Suite | File | Scope |
|-------|------|-------|
| ST-01 | `tests/system/tests/st01_ingest_end_to_end.rs` | multi-vendor RDSR → UI values; throughput |
| ST-02 | `st02_crash_recovery.rs` | kill -9, disk full, power-loss simulation |
| ST-03 | `st03_alerting.rs` | rule fire → notification; alert system failure visibility |
| ST-04 | `st04_security.rs` | AE whitelist, authN/Z, headers, TLS |
| ST-05 | `st05_dose_accuracy.rs` | reference dataset vs vendor console values |
| Manual | protocols MP-xx (templates/test-protocol.md) | install/upgrade/backup-restore, usability smoke |

Anomalies found → problem reports (§9); release requires all anomalies evaluated (§5.8.2).
