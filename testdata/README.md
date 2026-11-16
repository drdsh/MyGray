# Test data (configuration item, IEC 62304 §8)

**Only anonymised or synthetic data.** Every file has a row in `MANIFEST.json`
(source, vendor/model/SW version, anonymisation method, licence, SHA-256).

| Folder | Content | Used by |
|--------|---------|---------|
| `rdsr/ct/` | TID 10011 samples per vendor + `large_500_events.dcm` | golden tests, benchmark |
| `rdsr/projection/` | TID 10001 (XA, RF, DX, MG) | golden tests |
| `rdsr/radiopharma/` | TID 10021 | golden tests |
| `rdsr/vendor/` | one fixture per documented vendor quirk | SI-03 vendor table tests |
| `rdsr/corrupted/` | truncated, bit-flipped, deep-nesting, huge-length fields | robustness tests, fuzz seed corpus |
| `golden/` | expected normalised JSON (insta snapshots) | SRS-RDSR-001/002/006 |
| `configs/` | valid/invalid `mygray.toml` variants | SRS-CFG-001/002 |
