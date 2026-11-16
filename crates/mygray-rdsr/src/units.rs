//! UCUM unit conversion into canonical `mygray_core::units` types (RC-SW-010).
//! Unknown units are a *fatal* error for that value (never guess), reported
//! as `RdsrError::Implausible` → quarantine + alert.
