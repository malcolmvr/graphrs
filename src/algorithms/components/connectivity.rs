use crate::{ext::vec::VecExt, Error, Graph};
use std::collections::HashSet;
use std::fmt::Display;
use std::hash::Hash;

/**
Returns the components of a graph.
A connected component is a maximal connected subgraph of an undirected graph.

# Arguments:

* `graph`: a [Graph](../../struct.Graph.html) instance that must be undirected.

# Examples:

```
use graphrs::{Edge, Graph, GraphSpecs};
use graphrs::{algorithms::components};
let edges = vec![
    Edge::new("n1", "n2"),
    Edge::new("n2", "n3"),
    Edge::new("n3", "n4"),
    Edge::new("o1", "o2"),
    Edge::new("p1", "p2"),
    Edge::new("p2", "p3"),
];
let specs = GraphSpecs::undirected_create_missing();
let graph: Graph<&str, ()> = Graph::new_from_nodes_and_edges(vec![], edges, specs).unwrap();
let result = components::connected_components(&graph).unwrap();
assert_eq!(result.len(), 3);
```
*/
pub fn connected_components<T, A>(graph: &Graph<T, A>) -> Result<Vec<HashSet<T>>, Error>
where
    T: Hash + Eq + Clone + Ord + Display + Send + Sync,
    A: Clone + Send + Sync,
{
    graph.ensure_undirected()?;
    let mut seen = HashSet::new();
    let mut return_vec = vec![];
    for v in graph.get_all_node_names() {
        if !seen.contains(v) {
            let bfs = graph.breadth_first_search(v).to_hashset();
            seen = seen.union(&bfs).cloned().collect();
            return_vec.push(bfs);
        }
    }
    Ok(return_vec)
}

/**
Returns the number of components in a graph.
A connected component is a maximal connected subgraph of an undirected graph.

# Arguments:

* `graph`: a [Graph](../../struct.Graph.html) instance that must be undirected.

# Examples:

```
use graphrs::{Edge, Graph, GraphSpecs};
use graphrs::{algorithms::components};
let edges = vec![
    Edge::new("n1", "n2"),
    Edge::new("n2", "n3"),
    Edge::new("n3", "n4"),
    Edge::new("o1", "o2"),
    Edge::new("p1", "p2"),
    Edge::new("p2", "p3"),
];
let specs = GraphSpecs::undirected_create_missing();
let graph: Graph<&str, ()> = Graph::new_from_nodes_and_edges(vec![], edges, specs).unwrap();
let result = components::number_of_connected_components(&graph).unwrap();
assert_eq!(result, 3);
```
*/
pub fn number_of_connected_components<T, A>(graph: &Graph<T, A>) -> Result<usize, Error>
where
    T: Hash + Eq + Clone + Ord + Display + Send + Sync,
    A: Clone + Send + Sync,
{
    let result = connected_components(graph)?;
    Ok(result.len())
}

/**
Returns the set of nodes in the component of a graph containing a given node.

# Arguments:

* `graph`: a [Graph](../../struct.Graph.html) instance that must be undirected.
* `node_name`: the node whose component is to be returned.

# Examples:

```
use graphrs::{Edge, Graph, GraphSpecs};
use graphrs::{algorithms::components};
let edges = vec![
    Edge::new("n1", "n2"),
    Edge::new("n2", "n3"),
    Edge::new("n3", "n4"),
    Edge::new("o1", "o2"),
    Edge::new("p1", "p2"),
    Edge::new("p2", "p3"),
];
let specs = GraphSpecs::undirected_create_missing();
let graph: Graph<&str, ()> = Graph::new_from_nodes_and_edges(vec![], edges, specs).unwrap();
let result = components::node_connected_component(&graph, &"n2").unwrap();
assert_eq!(result.len(), 4);
```
*/
pub fn node_connected_component<T, A>(
    graph: &Graph<T, A>,
    node_name: &T,
) -> Result<HashSet<T>, Error>
where
    T: Hash + Eq + Clone + Ord + Display + Send + Sync,
    A: Clone + Send + Sync,
{
    graph.ensure_undirected()?;
    let bfs = graph.breadth_first_search(node_name);
    Ok(bfs.to_hashset())
}
