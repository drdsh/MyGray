# MG-DM-003 Manufacturing (Build) & Release
For software, "manufacturing" = reproducible build + release + distribution.
1. Tagged commit on `main`, all CI gates green (`cargo xtask ci`).
2. `cargo xtask release` in pinned build container → artefacts + SHA-256 + SBOM + evidence bundle.
3. Release review (record from `../../templates/release-record.md`), signatures.
4. Distribution channel, integrity verification by customer (checksums / signature).
5. Version ↔ UDI table.
