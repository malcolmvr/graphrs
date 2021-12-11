/*!
# graphrs

`graphrs` is a Rust package for the creation, manipulation and analysis of [graphs](https://en.wikipedia.org/wiki/Graph_(discrete_mathematics)).

It allows graphs to be created with support for:
* directed and undirected edges
* multiple edges between two nodes
* self-loops
* acyclic enforcement

## Major structs

* [Graph](./struct.Graph.html)
* [Node](./struct.Node.html)
* [Edge](./struct.Edge.html)

## Examples

```
use graphrs::{Edge, Graph, GraphSpecs, MissingNodeStrategy, Node};

let nodes = vec![
    Node::from_name("n1"),
    Node::from_name("n2"),
    Node::from_name("n3"),
];

let edges = vec![
    Edge::with_attribute("n1", "n2", "weight", &1.0),
    Edge::with_attribute("n2", "n1", "weight", &2.0),
    Edge::with_attribute("n1", "n3", "weight", &3.0),
    Edge::with_attribute("n2", "n3", "weight", &3.0),
];

let specs = GraphSpecs::directed();

let graph = Graph::<&str, &str, &f64>::new_from_nodes_and_edges(
    nodes,
    edges,
    specs
);
```

!*/

mod edge;
pub use edge::{Edge, EdgeSide};

mod error;
pub use error::{Error, ErrorKind};

mod graph;
pub use graph::Graph;

mod mod_generators;
pub use mod_generators::generators;

mod graph_specs;
pub use graph_specs::{
    EdgeDedupeStrategy, GraphSpecs, MissingNodeStrategy, SelfLoopsFalseStrategy,
};

mod node;
pub use node::Node;
