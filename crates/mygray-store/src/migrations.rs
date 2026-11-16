//! Forward-only schema migrations (`migrations/*.gql`), embedded via
//! `include_str!`, applied in order, recorded in a `:SchemaVersion` node.
//! A backup is taken automatically before any migration (SRS-STO-005).
