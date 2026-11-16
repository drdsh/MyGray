//! Zero-copy-ish, iterative (non-recursive → no stack overflow on hostile
//! input, SRS-RDSR-004) walker over the SR Content Sequence.

/// Maximum nesting depth accepted before aborting with a diagnostic.
pub const MAX_DEPTH: usize = 32;

/// Flat view of one content item.
#[derive(Debug, Clone)]
pub struct ContentItem {
    /// Dotted path, e.g. `1.4.2`.
    pub path: String,
    /// Relationship type (CONTAINS, HAS PROPERTIES, …).
    pub relationship: String,
    /// Value type (CONTAINER, NUM, CODE, UIDREF, TEXT, DATETIME…).
    pub value_type: String,
    /// Concept name code.
    pub concept: Option<crate::codes::Code>,
}
