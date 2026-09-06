#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]

//! Typed, format-neutral records for streaming property-graph data.
//!
//! This crate models one node or edge at a time. It deliberately does not
//! provide a graph container, parser, RDF conversion, or storage layer, so a
//! caller can keep the surrounding pipeline streaming and bounded by the
//! current record.

mod edge;
mod node;
mod property;

pub use edge::PgEdge;
pub use node::PgNode;
pub use property::PgProperty;

#[cfg(test)]
mod tests {
    use super::{
        PgEdge,
        PgNode,
        PgProperty,
    };

    #[test]
    fn defaults_are_owned_strings() {
        let node = PgNode::new(
            "person-1".to_owned(),
            vec!["Person".to_owned()],
            vec![PgProperty::new("name".to_owned(), "Alice".to_owned())],
        );

        assert_eq!(node.source_id, "person-1");
        assert_eq!(node.labels, vec!["Person".to_owned()]);
        assert_eq!(node.properties[0].value, "Alice");
    }

    #[test]
    fn identifiers_and_values_are_independent_types() {
        let edge = PgEdge::new(
            1_u64,
            2_u64,
            "KNOWS".to_owned(),
            vec![PgProperty::new("weight".to_owned(), 75_i32)],
        );

        assert_eq!(edge.source_id, 1);
        assert_eq!(edge.target_id, 2);
        assert_eq!(edge.properties[0].value, 75);
    }

    #[test]
    fn format_type_hint_is_optional_metadata() {
        let property = PgProperty::new("age".to_owned(), 42_i64).with_type_hint("integer");

        assert_eq!(property.value, 42);
        assert_eq!(property.type_hint.as_deref(), Some("integer"));
    }
}
