//! Property records attached to nodes and edges.

/// A key/value property attached to a property-graph node or edge.
///
/// `Value` is generic because input formats and downstream applications often
/// have a typed value representation. `String` is used by default for parsers
/// that defer value interpretation. `type_hint` remains opaque metadata from
/// the source format; a format-specific adapter decides how to interpret it.
#[derive(Debug, Clone, PartialEq)]
pub struct PgProperty<Value = String> {
    /// Property key.
    pub key: String,
    /// Property value.
    pub value: Value,
    /// Optional source-format type hint.
    pub type_hint: Option<String>,
}

impl<Value> PgProperty<Value> {
    /// Creates a property without a source-format type hint.
    #[must_use]
    pub const fn new(key: String, value: Value) -> Self {
        Self {
            key,
            value,
            type_hint: None,
        }
    }

    /// Adds an opaque type hint from the source format.
    #[must_use]
    pub fn with_type_hint(mut self, type_hint: impl Into<String>) -> Self {
        self.type_hint = Some(type_hint.into());
        self
    }
}
