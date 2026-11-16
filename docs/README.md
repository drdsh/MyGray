# MyGray – Documentation Index

All controlled documents live in this repository (docs-as-code). A document is
*released* when the containing release tag is signed and the document header
status is `approved`. Every document starts with the standard header:

```
| Doc ID | MG-xxx-nnn | Version | x.y | Status | draft/reviewed/approved |
| Author | … | Reviewer | … | Approver | … | Date | YYYY-MM-DD |
```

| Area | Folder | Governing standard / regulation |
|------|--------|---------------------------------|
| Technical documentation (Annex II/III) | [`mdr/`](mdr/00-technical-documentation-index.md) | MDR 2017/745 |
| Software life cycle | [`iec62304/`](iec62304/software-development-plan.md) | IEC 62304:2006+A1:2015 |
| Requirements (machine-readable) | [`../requirements/requirements.json`](../requirements/requirements.json) | IEC 62304 §5.2, ISO 14971 |
| Risk management | [`mdr/05-risk-management/`](mdr/05-risk-management/risk-management-plan.md) | ISO 14971:2019, ISO/TR 24971 |
| Usability engineering | [`usability/`](usability/usability-engineering-plan.md) | IEC 62366-1:2015+A1:2020 |
| Cybersecurity | [`cybersecurity/`](cybersecurity/security-plan.md) | IEC 81001-5-1, MDCG 2019-16 |
| Privacy | [`privacy/`](privacy/dpia.md) | GDPR |
| DICOM conformance | [`dicom/`](dicom/conformance-statement.md) | DICOM PS3.2 |
| Instructions for use / labelling | [`ifu/`](ifu/ifu-outline.md) | MDR Annex I ch. III, Reg. (EU) 2021/2226 |
| Architecture decisions | [`adr/`](adr/) | – |
| Record templates | [`templates/`](templates/) | ISO 13485 §4.2.5 |

Generated artefacts (not committed, produced by `cargo xtask release`):
trace matrix, test reports (JUnit), coverage, SBOM, rendered PDF of this tree.

The QMS itself (ISO 13485 procedures: document control, CAPA, supplier
control, training…) is out of scope of this repository and referenced by ID.
