//! Strongly typed dose quantities (RC-SW-010: unit confusion is a hazard, HAZ-004).
//!
//! All quantities are stored in a single canonical SI-derived unit. Conversion
//! from DICOM-coded units (UCUM codes in the RDSR, e.g. `mGy`, `Gy.m2`,
//! `mGy.cm`) happens exactly once, in `mygray-rdsr::units`.
//!
//! Arithmetic is explicit and checked; NaN/inf are rejected at construction.

use serde::{Deserialize, Serialize};

/// Error for invalid physical values.
#[derive(Debug, thiserror::Error, PartialEq)]
pub enum QuantityError {
    /// Value is NaN or infinite.
    #[error("non-finite value")]
    NonFinite,
    /// Value is negative where only non-negative values are physical.
    #[error("negative value {0}")]
    Negative(f64),
}

macro_rules! quantity {
    ($(#[$m:meta])* $name:ident, $unit:literal) => {
        $(#[$m])*
        #[doc = concat!("Canonical unit: `", $unit, "`.")]
        #[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
        pub struct $name(f64);

        impl $name {
            /// Canonical UCUM unit string.
            pub const UNIT: &'static str = $unit;

            /// Construct from a value already in the canonical unit.
            pub fn new(_v: f64) -> Result<Self, QuantityError> {
                todo!("reject NaN/inf/negative")
            }

            /// Raw value in the canonical unit.
            pub fn value(self) -> f64 { self.0 }
        }
    };
}

quantity!(
    /// CT volume dose index (CTDIvol).
    CtdiVol,
    "mGy"
);
quantity!(
    /// CT dose-length product.
    Dlp,
    "mGy.cm"
);
quantity!(
    /// Dose-area product (projection X-ray).
    Dap,
    "Gy.m2"
);
quantity!(
    /// Air kerma at reference point.
    AirKerma,
    "Gy"
);
quantity!(
    /// Fluoroscopy time.
    FluoroTime,
    "s"
);
quantity!(
    /// Size-specific dose estimate (AAPM TG-204/220).
    Ssde,
    "mGy"
);
quantity!(
    /// Administered activity (radiopharmaceutical, NM/PET).
    Activity,
    "MBq"
);
