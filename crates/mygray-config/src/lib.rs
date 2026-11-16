//! # mygray-config — Software item SI-02
//!
//! Loads `mygray.toml` + `MYGRAY_*` environment overrides (figment), validates the
//! result **completely before startup** (SRS-CFG-001: fail fast, never run
//! with a partially valid configuration) and exposes typed sections per item.
//!
//! Alert rules are *not* part of this file: they are versioned data in the
//! store, edited via the UI with audit (SRS-ALR-010).

use serde::Deserialize;

/// Root configuration.
#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    /// DICOM node (SI-04).
    pub dicom: DicomConfig,
    /// Graph store (SI-05).
    pub store: StoreConfig,
    /// Alerting sinks (SI-06).
    pub alert: AlertConfig,
    /// Web server (SI-07).
    pub web: WebConfig,
}

/// DICOM SCP configuration.
#[derive(Debug, Clone, Deserialize)]
pub struct DicomConfig {
    /// Our AE title.
    pub ae_title: String,
    /// Listen address, e.g. `0.0.0.0:11112`.
    pub listen: String,
    /// Durable spool directory for the ingest journal (must be on local, fsync-capable FS).
    pub spool_dir: std::path::PathBuf,
    /// Quarantine directory.
    pub quarantine_dir: std::path::PathBuf,
    /// Minimum free disk space before rejecting associations (bytes).
    pub min_free_bytes: u64,
    /// Accept only listed calling AE titles.
    pub ae_whitelist: Vec<String>,
    /// Max concurrent associations.
    pub max_associations: usize,
}

/// Grafeo store configuration.
#[derive(Debug, Clone, Deserialize)]
pub struct StoreConfig {
    /// Database file path.
    pub path: std::path::PathBuf,
    /// Backup directory.
    pub backup_dir: std::path::PathBuf,
}

/// Alert sink configuration.
#[derive(Debug, Clone, Deserialize)]
pub struct AlertConfig {
    /// SMTP relay for e-mail notifications (optional).
    pub smtp: Option<String>,
    /// Webhook URLs.
    pub webhooks: Vec<String>,
    /// Syslog target (RFC 5424), used as fallback channel.
    pub syslog: Option<String>,
}

/// Web server configuration.
#[derive(Debug, Clone, Deserialize)]
pub struct WebConfig {
    /// Listen address.
    pub listen: String,
    /// TLS cert/key paths (TLS mandatory outside `--dev`, SRS-SEC-008).
    pub tls_cert: Option<std::path::PathBuf>,
    /// TLS key.
    pub tls_key: Option<std::path::PathBuf>,
    /// Idle session timeout in minutes.
    pub session_timeout_min: u32,
}

/// Configuration errors.
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    /// Parse / merge failure.
    #[error("cannot load configuration: {0}")]
    Load(String),
    /// Semantic validation failure.
    #[error("invalid configuration: {field}: {reason}")]
    Invalid {
        /// Dotted field path.
        field: String,
        /// Why.
        reason: String,
    },
}

impl mygray_core::Reportable for ConfigError {
    fn code(&self) -> mygray_core::AlertCode {
        mygray_core::AlertCode("CFG-0001")
    }
    fn severity(&self) -> mygray_core::Severity {
        mygray_core::Severity::Critical
    }
}

/// Load and fully validate configuration. SRS-CFG-001.
pub fn load(_path: &std::path::Path) -> Result<Config, ConfigError> {
    todo!("figment: TOML file + MYGRAY_ env; then validate()")
}

impl Config {
    /// Semantic validation (paths writable, spool on same FS as quarantine, etc.).
    pub fn validate(&self) -> Result<(), ConfigError> {
        todo!("SRS-CFG-002")
    }
}

#[cfg(test)]
mod tests {
    #[test]
    #[ignore = "skeleton"]
    fn srs_cfg_001_rejects_missing_spool_dir() {}

    #[test]
    #[ignore = "skeleton"]
    fn srs_cfg_002_rejects_unwritable_paths() {}
}
