# MG-RM-001 Risk Management Plan (ISO 14971:2019 §4.4)
## Scope (MyGray all versions, whole life cycle)
## Responsibilities & qualification
## Risk acceptability criteria
Severity S1–S5 × probability P1–P5 matrix (see `requirements.schema.json` enums):

| | P1 | P2 | P3 | P4 | P5 |
|--|--|--|--|--|--|
| S5 | R | U | U | U | U |
| S4 | A | R | U | U | U |
| S3 | A | R | R | U | U |
| S2 | A | A | R | R | U |
| S1 | A | A | A | R | R |

A = acceptable, R = reduce as far as possible + benefit-risk, U = unacceptable.
For software, probability of a software failure is assumed 1 (IEC 62304 §4.3); P refers to the probability of harm given the sequence of events.

## Activities: hazard identification (ISO/TR 24971 annex A questions, IEC 62366 use errors, IEC 81001-5-1 threats), FMEA per software item, risk control, verification of effectiveness, overall residual risk, production & post-production information
## Data: hazards and risk controls are maintained in `requirements/requirements.json` (`hazards`, `risk_controls`); this plan + generated risk table form the risk management file.
