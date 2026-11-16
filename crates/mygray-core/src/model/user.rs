//! User and role entities (RBAC, SRS-SEC-001..003).

use serde::{Deserialize, Serialize};

use crate::ids::UserId;

/// Roles. Permissions per role are defined in `docs/mdr/03-design/rbac-matrix.md`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Role {
    /// Read dose data.
    Viewer,
    /// Medical physics expert: configure rules, DRLs.
    Physicist,
    /// Acknowledge alerts, view PHI.
    Radiographer,
    /// User/system administration (no PHI by default).
    Administrator,
    /// Read-only audit access (DPO / QA).
    Auditor,
}

/// A user account.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    /// Internal key.
    pub id: UserId,
    /// Login name.
    pub login: String,
    /// Assigned roles.
    pub roles: Vec<Role>,
    /// Account enabled.
    pub active: bool,
}
