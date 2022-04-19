/*!
# graphrs

`graphrs` is a Rust package for the creation, manipulation and analysis of [graphs](https://en.wikipedia.org/wiki/Graph_(discrete_mathematics)).

It allows graphs to be created with support for:
* directed and undirected edges
* multiple edges between two nodes
* self-loops

## Major structs

* [Graph](./struct.Graph.html)
* [Node](./struct.Node.html)
* [Edge](./struct.Edge.html)

## Example: create a graph

```
use graphrs::{Edge, Graph, GraphSpecs, Node};

let nodes = vec![
    Node::from_name("n1"),
    Node::from_name("n2"),
    Node::from_name("n3"),
];

let edges = vec![
    Edge::with_weight("n1", "n2", 1.0),
    Edge::with_weight("n2", "n1", 2.0),
    Edge::with_weight("n1", "n3", 3.0),
    Edge::with_weight("n2", "n3", 3.0),
];

let specs = GraphSpecs::directed(); // change this to `::undirected()` to get an undirected graph

let graph = Graph::<&str, ()>::new_from_nodes_and_edges(
    nodes,
    edges,
    specs
);
```

## Example: create a graph from just edges

```
use graphrs::{Edge, Graph, GraphSpecs, Node};

let mut graph = Graph::<&str, ()>::new(GraphSpecs::directed_create_missing());
graph.add_edges(vec![
    Edge::with_weight("n1", "n2", 1.0),
    Edge::with_weight("n2", "n1", 2.0),
    Edge::with_weight("n1", "n3", 3.0),
    Edge::with_weight("n2", "n3", 3.0),
]);
```

## Example: create a graph with nodes that have attributes

```
use graphrs::{Edge, Graph, GraphSpecs, Node};

#[derive(Copy, Clone)]
struct NodeAttribute<'a> {
    first_name: &'a str,
    last_name: &'a str,
};

let mut graph: Graph<i32, NodeAttribute> = Graph::new(GraphSpecs::undirected());
graph.add_node(Node {
    name: 1,
    attributes: Some(NodeAttribute {first_name: "Jane", last_name: "Smith"})
});
```
!*/

// run doc tests on the README.md file
#[macro_use]
extern crate doc_comment;
doc_comment!(include_str!("../README.md"));

mod edge;
pub use edge::Edge;

mod error;
pub use error::{Error, ErrorKind};

mod ext;

mod graph;
pub use graph::Graph;

pub mod algorithms;
pub mod generators;
pub mod readwrite;

mod graph_specs;
pub use graph_specs::{
    EdgeDedupeStrategy, GraphSpecs, MissingNodeStrategy, SelfLoopsFalseStrategy,
};

mod node;
pub use node::Node;
