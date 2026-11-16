//! Device (modality) entity.

use serde::{Deserialize, Serialize};

use crate::ids::DeviceId;

/// An imaging device that sends dose data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Device {
    /// Internal key.
    pub id: DeviceId,
    /// Calling AE title.
    pub ae_title: String,
    /// Manufacturer (0008,0070).
    pub manufacturer: Option<String>,
    /// Model name (0008,1090).
    pub model: Option<String>,
    /// Device serial number (0018,1000).
    pub serial: Option<String>,
    /// Station name (0008,1010).
    pub station: Option<String>,
    /// Whether this AE is permitted to store (SRS-SEC-010 AE whitelist).
    pub accepted: bool,
}
