# MG-DM-002 Role / Permission Matrix (SRS-SEC-003)
Mirrors `crates/mygray-web/src/auth/rbac.rs`; a unit test fails if they diverge.

| Permission | Viewer | Radiographer | Physicist | Administrator | Auditor |
|------------|:-----:|:------------:|:---------:|:-------------:|:-------:|
| View dose statistics (pseudonymised) | ✔ | ✔ | ✔ | – | – |
| View patient identity (PHI) | – | ✔ | ✔ | – | – |
| Acknowledge alerts | – | ✔ | ✔ | – | – |
| Create/edit alert rules, DRLs, protocol mapping | – | – | ✔ | – | – |
| Manage users, devices, configuration | – | – | – | ✔ | – |
| Reprocess / discard quarantined objects | – | – | ✔ | ✔ | – |
| Read audit trail | – | – | – | – | ✔ |
| Export data | – | – | ✔ | – | – |
