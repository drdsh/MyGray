# MG-RM-002 Risk Analysis
Generated table from `requirements.json` (`cargo xtask risk-table`): hazard →
sequence of events → hazardous situation → harm → S/P before → controls →
implementing SRS → verification result → S/P after.

## Software FMEA per item (SI-01 … SI-08)
| Item | Failure mode | Effect | Hazard | Control |
|------|--------------|--------|--------|---------|
| SI-04 | fsync not honoured by filesystem | object lost on power failure | HAZ-001 | install requirement: local FS with write barriers; SRS-CFG-002 check |
| SI-03 | vendor-specific template deviation | wrong/missing value | HAZ-003 | vendor quirk table, golden files, plausibility |
| SI-06 | sink misconfigured | alert not delivered | HAZ-002 | test-notification button, fallback, RED health |
| … | | | | |
