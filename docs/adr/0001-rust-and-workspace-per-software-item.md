# ADR-0001 Rust; one crate per IEC 62304 software item
Status: accepted · Context: class B medical software, need for memory safety and enforceable segregation. · Decision: Rust 2024, cargo workspace, crate = software item, `unsafe` forbidden. · Consequences: architecture is checked mechanically; SOUP ecosystem smaller than JVM/.NET (DICOM via dicom-rs).
