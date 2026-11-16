//! Storage SCP: association acceptance and DIMSE dispatch.
//! Each association runs in its own task; a bounded semaphore enforces
//! `max_associations` (SRS-DCM-009).

pub mod association;
pub mod c_echo;
pub mod c_store;

/// Supported SOP classes (see `conformance` and docs/dicom/conformance-statement.md).
pub const ACCEPTED_SOP_CLASSES: &[&str] = &[
    "1.2.840.10008.1.1",             // Verification
    "1.2.840.10008.5.1.4.1.1.88.67", // X-Ray Radiation Dose SR
    "1.2.840.10008.5.1.4.1.1.88.68", // Radiopharmaceutical Radiation Dose SR
    "1.2.840.10008.5.1.4.1.1.88.22", // Enhanced SR (vendor dose sheets)
    "1.2.840.10008.5.1.4.1.1.7",     // Secondary Capture (dose screens, stored only)
];
