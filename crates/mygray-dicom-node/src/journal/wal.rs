//! Append-only write-ahead log of `IngestState` transitions. Each record is
//! length-prefixed and CRC32C-checked; a torn tail record is detected and
//! truncated on recovery, every earlier record is preserved (SRS-DCM-004).
