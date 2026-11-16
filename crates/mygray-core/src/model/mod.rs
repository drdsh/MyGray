//! Domain entities. These mirror the graph schema in `mygray-store::schema`
//! but are storage-agnostic.
//!
//! ```text
//! (Patient)-[:HAS_STUDY]->(Study)-[:HAS_SERIES]->(Series)
//! (Study)-[:PERFORMED_ON]->(Device)
//! (Study)-[:HAS_DOSE_REPORT]->(DoseReport)-[:HAS_EVENT]->(IrradiationEvent)
//! (User)-[:ACKNOWLEDGED]->(Alert)-[:CONCERNS]->(Study|Device|Patient)
//! ```

pub mod device;
pub mod dose;
pub mod patient;
pub mod series;
pub mod study;
pub mod user;
