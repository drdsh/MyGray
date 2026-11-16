#![no_main]
//! Fuzz target for SRS-RDSR-004.
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let _ = mygray_rdsr::parse_bytes(data);
});
