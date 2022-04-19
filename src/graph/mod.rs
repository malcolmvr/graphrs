use crate::{Edge, GraphSpecs, Node};
use std::collections::{HashMap, HashSet};

/**
The `Graph` struct represents a graph of nodes and vertices.
It allows graphs to be created with support for:
* directed and undirected edges
* multiple edges between two nodes
* self-loops

A `Graph` has two generic arguments:
* `T`: Specifies the type to use for node names.
* `A`: Specifies the type to use for node and edge attributes. Attributes are *optional*
extra data that are associated with a node or an edge. For example, if nodes represent
people and `T` is an `i32` of their employee ID then the node attributes might store
their first and last names.

# Example

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

let specs = GraphSpecs::directed();

let graph = Graph::<&str, ()>::new_from_nodes_and_edges(
    nodes,
    edges,
    specs
);
```
*/
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Graph<T: PartialOrd + Send, A: Clone> {
    /// The graph's nodes, stored as a `HashMap` keyed by the node names.
    nodes: HashMap<T, Node<T, A>>,
    /// The graph's edges, stored as a `HashMap` keyed by a tuple of node names.
    edges: HashMap<(T, T), Vec<Edge<T, A>>>,
    /// The [GraphSpecs](./struct.GraphSpecs.html) for the graph.
    pub specs: GraphSpecs,
    /// Stores the successors of nodes. A successor of u is a node v such that there
    /// exists a directed edge from u to v. For an undirected graph `successors` stores
    /// all the adjacent nodes. An adjacent node to u is a node v such that there exists
    /// an edge from u to v *or* from v to u.
    successors: HashMap<T, HashSet<T>>,
    /// Stores the predecessors of nodes. A predecessor of v is a node u such that there
    /// exists a directed edge from u to v. For an undirected graph `precessors` is not used.
    predecessors: HashMap<T, HashSet<T>>,
}

mod convert;
mod creation;
mod degree;
mod density;
mod ensure;
mod query;
mod subgraph;
