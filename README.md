# property-graph-model

[![Crates.io](https://img.shields.io/crates/v/property-graph-model.svg)](https://crates.io/crates/property-graph-model)
[![Documentation](https://docs.rs/property-graph-model/badge.svg)](https://docs.rs/property-graph-model)
[![CI](https://github.com/legra-ai/property-graph-model/actions/workflows/ci.yml/badge.svg)](https://github.com/legra-ai/property-graph-model/actions/workflows/ci.yml)
[![License](https://img.shields.io/crates/l/property-graph-model.svg)](https://github.com/legra-ai/property-graph-model#license)

Typed, format-neutral records for streaming property-graph data.

## Why this crate exists

Property-graph parsers should be able to share a small intermediate model
without sharing a parser, an RDF implementation, a graph store, or an
application's query architecture. `property-graph-model` defines that model
at the record boundary:

- `PgNode` represents one node with a source identifier, labels, and
  properties;
- `PgEdge` represents one relationship between source identifiers;
- `PgProperty<Value>` represents one key/value property and optional opaque
  type metadata.

The crate does not contain a graph-sized collection. A parser should emit one
record at a time, allowing the surrounding pipeline to keep its memory use
proportional to the current record rather than the complete input.

## Example

```rust
use property_graph_model::{PgNode, PgProperty};

let node = PgNode::new(
    "person-1".to_owned(),
    vec!["Person".to_owned()],
    vec![PgProperty::new("name".to_owned(), "Alice".to_owned())],
);

assert_eq!(node.source_id, "person-1");
assert_eq!(node.labels, vec!["Person".to_owned()]);
assert_eq!(node.properties[0].value, "Alice");

let typed_node = PgNode::new(
    "person-1".to_owned(),
    vec!["Person".to_owned()],
    vec![PgProperty::new("age".to_owned(), 42_i64).with_type_hint("integer")],
);

assert_eq!(typed_node.properties[0].value, 42);
```

The default types use owned `String` identifiers and values. Use the generic
parameters when a parser already has typed identifiers or values:

```rust
use property_graph_model::{PgEdge, PgProperty};

let edge = PgEdge::new(
    10_u64,
    20_u64,
    "KNOWS".to_owned(),
    vec![PgProperty::new("confidence".to_owned(), 0.9_f32)],
);

assert_eq!(edge.source_id, 10);
assert_eq!(edge.properties[0].value, 0.9);
```

## Scope

This crate intentionally does not decide how property-graph records are
parsed, streamed, stored, converted to RDF, or rendered. Those concerns can
be implemented by separate adapters. Related crates include
[`neo4j-csv`](https://crates.io/crates/neo4j-csv) for streaming Neo4j annotated
CSV input and [`shacl-structural-model`](https://crates.io/crates/shacl-structural-model)
for typed SHACL structural values.

## License

Licensed under either of:

- Apache License, Version 2.0
- MIT License

at your option.
