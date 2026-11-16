# MG-REL-001 Software Release Procedure (IEC 62304 §5.8)
1. All verification complete; `cargo xtask ci --release` green (no ignored tests, coverage thresholds).
2. Residual anomalies listed and evaluated (§5.8.2/5.8.3).
3. Release notes, IFU and labelling updated (version, UDI).
4. `cargo xtask release` → `dist/mygray-X.Y.Z/` : binary + SHA-256, container image digest, SBOM (CycloneDX), trace matrix, test & coverage reports, rendered docs.
5. Release record signed (QA, RA, Dev lead); tag signed; archive.
