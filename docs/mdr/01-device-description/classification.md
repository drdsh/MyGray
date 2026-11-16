# MG-DD-002 Qualification & Classification Rationale

## Qualification as MDSW (MDCG 2019-11 decision tree)
| Step | Question | Answer / rationale |
|------|----------|--------------------|
| 1 | Is it software? | Yes |
| 2 | Medical device software per Art. 2(1)? | Processes patient-specific dose data to inform follow-up and optimisation → yes |
| 3 | Action on data beyond storage/archival/communication/simple search? | Yes: calculation of cumulative dose, DRL comparison, alerting |
| 4 | For the benefit of individual patients? | Yes (follow-up of individual patients) |

## Classification (Annex VIII)
Rule 11: software providing information used to take decisions with diagnosis
or therapeutic purposes → **Class IIa** (proposed). Discussion of why
IIb/III do not apply (decision impact cannot cause death or irreversible
deterioration … ) — *to be completed and confirmed by RA / Notified Body*.

## IEC 62304 software safety classification
System: **Class B** (non-serious injury possible: delayed follow-up of
skin injury, unnecessary repeat exposure). Hardware-independent external risk
controls considered: clinical processes (e.g. physician skin check after
long fluoroscopy) – not credited for classification.
Per-item classes: see `../../iec62304/software-architecture.md` §5.
