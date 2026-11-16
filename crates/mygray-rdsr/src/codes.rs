//! Coded concepts (DCM, SCT, UCUM) used by the dose templates, as `const`s.
//! Generated from PS3.16 by `cargo xtask gen-codes` (do not edit by hand).

/// A coded concept `(CodeValue, CodingSchemeDesignator)`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Code {
    /// Code value.
    pub value: &'static str,
    /// Coding scheme designator.
    pub scheme: &'static str,
}

/// (113701, DCM, "X-Ray Radiation Dose Report") – document title.
pub const XRAY_RADIATION_DOSE_REPORT: Code = Code {
    value: "113701",
    scheme: "DCM",
};
/// (113830, DCM, "Mean CTDIvol").
pub const MEAN_CTDIVOL: Code = Code {
    value: "113830",
    scheme: "DCM",
};
/// (113838, DCM, "DLP").
pub const DLP: Code = Code {
    value: "113838",
    scheme: "DCM",
};
/// (113813, DCM, "CT Dose Length Product Total").
pub const CT_DLP_TOTAL: Code = Code {
    value: "113813",
    scheme: "DCM",
};
/// (122130, DCM, "Dose Area Product").
pub const DOSE_AREA_PRODUCT: Code = Code {
    value: "122130",
    scheme: "DCM",
};
/// (113769, DCM, "Irradiation Event UID").
pub const IRRADIATION_EVENT_UID: Code = Code {
    value: "113769",
    scheme: "DCM",
};
