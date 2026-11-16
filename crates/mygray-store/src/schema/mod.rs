//! Node labels, edge types, property keys, constraints and indexes — single
//! source of truth used by repositories and migrations.

pub mod edges;
pub mod nodes;

/// Current schema version. Bump together with a new file in `migrations/`.
pub const SCHEMA_VERSION: u32 = 1;
