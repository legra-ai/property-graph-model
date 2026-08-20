//! Property-graph node records.

use crate::PgProperty;

/// A property-graph node record.
///
/// A node is one record in a larger stream. The model does not provide a
/// graph-sized collection; parsers should emit each node as soon as its input
/// record is complete.
#[derive(Debug, Clone, PartialEq)]
pub struct PgNode<Id = String, Value = String> {
    /// Format-native node identifier.
    pub source_id: Id,
    /// Labels attached to the node.
    pub labels: Vec<String>,
    /// Key/value properties attached to the node.
    pub properties: Vec<PgProperty<Value>>,
}

impl<Id, Value> PgNode<Id, Value> {
    /// Creates a node record.
    #[must_use]
    pub const fn new(
        source_id: Id,
        labels: Vec<String>,
        properties: Vec<PgProperty<Value>>,
    ) -> Self {
        Self {
            source_id,
            labels,
            properties,
        }
    }
}
