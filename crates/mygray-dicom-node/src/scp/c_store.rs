//! C-STORE handler. MUST NOT reply Success before `Spool::write_durable`
//! returned Ok (SRS-DCM-001). Status mapping:
//! * 0x0000 Success — durable in spool
//! * 0xA700 Out of resources — spool/disk failure (sender retries)
//! * 0xA900 Data set does not match SOP class — rejected, but still spooled to quarantine
//! * 0x0122 SOP class not supported
