//! Quarantine: objects that cannot be parsed/validated. Raw bytes are kept
//! together with a JSON sidecar (reason, diagnostics, timestamps). Objects
//! leave quarantine only by explicit, audited user action (reprocess or
//! documented discard) – SRS-DCM-006.
