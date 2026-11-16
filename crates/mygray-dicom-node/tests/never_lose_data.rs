//! Integration tests for the never-lose-data guarantee (HAZ-001).
//! Uses fault injection (full disk via tmpfs quota, fsync failure via
//! test-only `FaultyFs` implementation of the spool's filesystem trait).

#[test]
#[ignore = "skeleton"]
fn srs_dcm_001_store_success_only_after_durable_write() {}

#[test]
#[ignore = "skeleton"]
fn srs_dcm_003_spool_failure_returns_a700_and_raises_critical_alert() {}

#[test]
#[ignore = "skeleton"]
fn srs_dcm_004_unfinished_entries_replayed_after_restart() {}

#[test]
#[ignore = "skeleton"]
fn srs_dcm_005_duplicate_send_is_idempotent() {}

#[test]
#[ignore = "skeleton"]
fn srs_dcm_006_unparseable_object_is_quarantined_not_deleted() {}
