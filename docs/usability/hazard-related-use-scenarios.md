# MG-HRUS-001 Hazard-related Use Scenarios
| ID | Scenario | Potential use error | Hazard | UI risk control |
|----|----------|--------------------|--------|-----------------|
| HRUS-01 | MPE configures rule threshold | enters Gy instead of mGy | HAZ-004 | unit selector, shown converted value, confirmation |
| HRUS-02 | Radiographer reads cumulative dose | misreads decimal separator | HAZ-004 | locale formatting, unit always shown |
| HRUS-03 | User acknowledges alert without reading | missed follow-up | HAZ-002 | mandatory comment |
| HRUS-04 | Admin overlooks RED status | malfunction unnoticed | HAZ-002 | persistent banner on all pages |
