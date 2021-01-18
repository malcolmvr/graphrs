/*!
# GraphRS
GraphRS is a Rust crate for the analysis of [graphs](https://en.wikipedia.org/wiki/Graph_(discrete_mathematics)).
It allows graphs to be created with support for:
* directed and undirected edges
* multiple edges between two nodes
* self-loops
* acyclic enforcement
## Major structs
* [Graph](./struct.Graph.html)
* [Node](./struct.Node.html)
* [Edge](./struct.Edge.html)
## Example
```
use graphrs::{Edge, Graph, GraphSpecs, MissingNodeStrategy, Node};

let nodes = vec![
    Node::from_name("n1"),
    Node::from_name("n2"),
    Node::from_name("n3"),
];

let edges = vec![
    Edge::with_weight("n1", "n2", &1.0),
    Edge::with_weight("n2", "n1", &2.0),
    Edge::with_weight("n1", "n3", &3.0),
    Edge::with_weight("n2", "n3", &3.0),
];

let graph = Graph::<&str, &str, &f64>::new_from_nodes_and_edges(
    nodes,
    edges,
    GraphSpecs::new(false, true, false, false),
    MissingNodeStrategy::Error,
);
```
!*/

mod edge;
pub use edge::{Edge, EdgeSide};

mod error;
pub use error::{Error, ErrorKind};

mod graph;
pub use graph::Graph;

mod graph_specs;
pub use graph_specs::GraphSpecs;

mod merge_attributes;
pub use merge_attributes::AttributeMergeStrategy;

mod missing_node_strategy;
pub use missing_node_strategy::MissingNodeStrategy;

mod node;
pub use node::Node;
