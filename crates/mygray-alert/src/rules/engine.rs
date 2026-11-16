//! Deterministic rule evaluation. Input: DoseReport + DoseContext + active rule
//! set snapshot. Output: list of fired alerts with full provenance (rule id,
//! version, observed value, threshold) — SRS-ALR-012. Pure; exhaustively unit
//! tested with table-driven cases (`tests/rule_engine.rs`).
