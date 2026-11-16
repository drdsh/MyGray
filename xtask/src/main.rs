//! `cargo xtask <command>` – compliance automation (IEC 62304 §5.1.4 tooling).
//!
//! Commands:
//! * `trace`        – requirements.json ⇄ tests: every SRS has ≥1 test whose name
//!   starts with its id (`SRS-DCM-001` → `srs_dcm_001_…`); writes
//!   `target/compliance/trace-matrix.json`
//! * `ci`           – fmt --check, clippy -D warnings, test, deny, trace, arch-check
//! * `arch-check`   – crate dependency graph matches docs/iec62304/software-architecture.md
//! * `soup-check`   – every [workspace.dependencies] entry has a row in soup-list.md
//! * `sbom`         – CycloneDX SBOM (cargo-cyclonedx) into dist/
//! * `coverage`     – cargo-llvm-cov, per software item, thresholds per safety class
//! * `release`      – builds release evidence bundle: binaries + SHA256, SBOM,
//!   test reports (JUnit), trace matrix, coverage, docs PDF
//!
//! Only `trace` is sketched here; the rest are stubs to be implemented.

#![allow(clippy::print_stdout, clippy::print_stderr, clippy::indexing_slicing)] // serde_json::Value indexing never panics

use std::path::{Path, PathBuf};
use std::process::ExitCode;

fn main() -> ExitCode {
    let cmd = std::env::args().nth(1).unwrap_or_default();
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
    match cmd.as_str() {
        "trace" => trace(&root),
        "ci" | "arch-check" | "soup-check" | "sbom" | "coverage" | "release" => {
            eprintln!("xtask {cmd}: not yet implemented (skeleton)");
            ExitCode::FAILURE
        }
        _ => {
            eprintln!("usage: cargo xtask <trace|ci|arch-check|soup-check|sbom|coverage|release>");
            ExitCode::FAILURE
        }
    }
}

fn trace(root: &Path) -> ExitCode {
    let path = root.join("requirements/requirements.json");
    let text = match std::fs::read_to_string(&path) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("cannot read {}: {e}", path.display());
            return ExitCode::FAILURE;
        }
    };
    let doc: serde_json::Value = match serde_json::from_str(&text) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("requirements.json invalid: {e}");
            return ExitCode::FAILURE;
        }
    };
    // Only requirements verified by an automated method need a linked Rust test.
    const AUTOMATED: &[&str] = &[
        "unit_test",
        "integration_test",
        "system_test",
        "fuzz_test",
        "benchmark",
    ];
    let ids: Vec<String> = doc["software_requirements"]
        .as_array()
        .map(|a| {
            a.iter()
                .filter(|r| {
                    r["verification"]["method"].as_array().is_some_and(|m| {
                        m.iter()
                            .any(|x| x.as_str().is_some_and(|s| AUTOMATED.contains(&s)))
                    })
                })
                .filter_map(|r| r["id"].as_str().map(str::to_owned))
                .collect()
        })
        .unwrap_or_default();

    let mut sources = String::new();
    collect_rs(&root.join("crates"), &mut sources);
    collect_rs(&root.join("tests"), &mut sources);

    let mut missing = 0usize;
    for id in &ids {
        let fn_prefix = format!("fn {}_", id.to_lowercase().replace('-', "_"));
        if !sources.contains(&fn_prefix) {
            println!("UNTESTED  {id}");
            missing = missing.saturating_add(1);
        }
    }
    println!(
        "{} automatically verifiable software requirements, {} without a linked test",
        ids.len(),
        missing
    );
    if missing == 0 {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

fn collect_rs(dir: &Path, out: &mut String) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let p: PathBuf = entry.path();
        if p.is_dir() {
            if p.file_name().is_some_and(|n| n != "target") {
                collect_rs(&p, out);
            }
        } else if p.extension().is_some_and(|e| e == "rs")
            && let Ok(s) = std::fs::read_to_string(&p) {
                out.push_str(&s);
            }
    }
}
