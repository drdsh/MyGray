# MG-ROP-001 Data inventory
| DICOM attribute | Tag | Stored? | Where | Purpose | Retention |
|-----------------|-----|---------|-------|---------|-----------|
| Patient's Name | (0010,0010) | encrypted | PatientDemographics | identification for follow-up | legal period |
| Patient's Birth Date | (0010,0030) | year only | Patient | age-dependent rules | legal period |
| Patient's Weight | (0010,1030) | yes | Study | SSDE, paediatric rules | legal period |
| … | | | | | |
