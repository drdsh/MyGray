//! View models (askama `Template` structs) and formatting helpers.
//! ALL dose value formatting goes through `format` (unit always shown,
//! fixed significant digits, locale-aware decimal separator) — SRS-WEB-003.

pub mod charts;
pub mod format;
