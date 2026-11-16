//! System test harness: starts the `mygray` binary with a temp config,
//! sends DICOM objects with a test SCU (dicom-rs `storescu`), queries HTTP,
//! kills/restarts the process for crash-recovery tests.
//!
//! Each test file maps to one section of docs/iec62304/system-test-plan.md.
