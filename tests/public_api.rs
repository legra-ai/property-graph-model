//! Public-API integration test: the format-neutral node, edge, and
//! property records construct with string and typed values.

use property_graph_model::{
    PgEdge,
    PgNode,
    PgProperty,
};

#[test]
fn nodes_carry_ids_labels_and_properties() {
    let node = PgNode::new(
        "person-1".to_owned(),
        vec!["Person".to_owned(), "Employee".to_owned()],
        vec![PgProperty::new("name".to_owned(), "Alice".to_owned())],
    );
    assert_eq!(node.source_id, "person-1");
    assert_eq!(node.labels, ["Person", "Employee"]);
    assert_eq!(node.properties[0].key, "name");
    assert_eq!(node.properties[0].value, "Alice");
    assert_eq!(node.properties[0].type_hint, None);
}

#[test]
fn values_and_ids_are_generic_and_type_hints_are_optional() {
    let typed = PgNode::new(
        7_u64,
        Vec::new(),
        vec![PgProperty::new("age".to_owned(), 42_i64).with_type_hint("integer")],
    );
    assert_eq!(typed.source_id, 7);
    assert_eq!(typed.properties[0].value, 42);
    assert_eq!(typed.properties[0].type_hint.as_deref(), Some("integer"));

    let edge = PgEdge::new(
        10_u64,
        20_u64,
        "KNOWS".to_owned(),
        vec![PgProperty::new("confidence".to_owned(), 0.9_f32)],
    );
    assert_eq!((edge.source_id, edge.target_id), (10, 20));
    assert_eq!(edge.rel_type, "KNOWS");
    assert!((edge.properties[0].value - 0.9_f32).abs() < f32::EPSILON);
}
