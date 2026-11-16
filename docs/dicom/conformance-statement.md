# MyGray DICOM Conformance Statement (structure per DICOM PS3.2 Annex A)
1. Overview · 2. Table of contents · 3. Introduction (revision history, audience, remarks, terms, references)
4. Networking
 4.1 Implementation model (application data flow, functional definition of AEs, sequencing)
 4.2 AE specifications — Storage SCP "MYGRAY"
   * SOP classes: Verification (1.2.840.10008.1.1), X-Ray Radiation Dose SR (…88.67), Radiopharmaceutical Radiation Dose SR (…88.68), Enhanced SR (…88.22), Secondary Capture (…1.1.7)
   * Transfer syntaxes: Implicit VR LE, Explicit VR LE, Deflated Explicit VR LE
   * Association policies: max associations (configurable, default 16), max PDU, AE whitelist, ARTIM timeout
   * C-STORE status codes: 0000 (only after durable storage), A700, A900, 0122 — see SRS-DCM-001/003
 4.3 Network interfaces · 4.4 Configuration
5. Media interchange — not supported
6. Support of character sets (ISO_IR 100, ISO_IR 192 UTF-8, …)
7. Security (DICOM TLS profile, AE whitelist)
8. Annexes: IOD contents, SR templates supported (TID 10001, 10011, 10021), coded terminology, private attributes (none), IHE REM integration statement
