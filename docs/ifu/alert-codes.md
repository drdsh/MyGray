# Alert Code Catalogue
Codes are stable and never reused. Source of truth: `Reportable::code()` implementations (xtask verifies this table is complete).

| Code | Severity | Item | Meaning | User action |
|------|----------|------|---------|-------------|
| CFG-0001 | Critical | SI-02 | configuration invalid, service not started | fix configuration per message |
| DCM-0001 | Critical | SI-04 | object could not be stored; sender was told to retry | check disk/filesystem immediately |
| DCM-0002 | Error | SI-04 | spool disk space below minimum | free/extend disk |
| DCM-0003 | Warning | SI-04 | association from unknown AE rejected | register device if legitimate |
| DCM-0004 | Warning | SI-04 | association/protocol error | check modality network config |
| DCM-0005 | Critical | SI-04 | journal integrity failure | contact support |
| DCM-0006 | Critical | SI-04 | persistence failed repeatedly; raw data safe in spool | check database health |
| RDSR-0001…0004 | Error | SI-03 | object quarantined (decode / unsupported / missing / implausible) | review in quarantine view |
| STO-0001…0004 | var. | SI-05 | database errors | see message |
| ALR-0001 | Critical | SI-06 | notification delivery failed on all channels | check sink config |
| ALR-0002 | Critical | SI-06 | alerts were dropped due to overload (count given) | review system load |
| SUP-0001 | Critical | SI-08 | task crashed / heartbeat missing | contact support if repeated |
