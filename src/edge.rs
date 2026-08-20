//! Property-graph edge records.

use crate::PgProperty;

/// A property-graph edge record.
///
/// The edge refers to its endpoints using the source-format identifiers. A
/// parser or consumer is responsible for resolving those identifiers in its
/// own streaming workflow.
#[derive(Debug, Clone, PartialEq)]
pub struct PgEdge<Id = String, Value = String> {
    /// Format-native identifier of the source node.
    pub source_id: Id,
    /// Format-native identifier of the target node.
    pub target_id: Id,
    /// Relationship type.
    pub rel_type: String,
    /// Key/value properties attached to the edge.
    pub properties: Vec<PgProperty<Value>>,
}

impl<Id, Value> PgEdge<Id, Value> {
    /// Creates an edge record.
    #[must_use]
    pub const fn new(
        source_id: Id,
        target_id: Id,
        rel_type: String,
        properties: Vec<PgProperty<Value>>,
    ) -> Self {
        Self {
            source_id,
            target_id,
            rel_type,
            properties,
        }
    }
}
