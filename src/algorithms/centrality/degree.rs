use crate::Graph;
use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;

/**
Compute degree centrality for nodes.

The degree centrality for a node is the fraction of nodes it is connected to.

The degree centrality values are normalized by dividing by the maximum
possible degree in a simple graph n-1 where n is the number of nodes in G.

For multigraphs or graphs with self loops the maximum degree might
be higher than n-1 and values of degree centrality greater than 1
are possible.

# Arguments

* `graph`: a [Graph](../../../struct.Graph.html) instance

# Examples

```
use graphrs::{algorithms::{centrality::{degree}}, generators};
let graph = generators::social::karate_club_graph();
let centralities = degree::degree_centrality(&graph);
```
*/
pub fn degree_centrality<T, A>(graph: &Graph<T, A>) -> HashMap<T, f64>
where
    T: Hash + Eq + Clone + Ord + Display + Send + Sync,
    A: Clone + Send + Sync,
{
    let num_nodes = graph.get_all_nodes().len();
    if num_nodes <= 1 {
        return graph
            .get_all_nodes()
            .iter()
            .map(|n| (n.name.clone(), 1.0))
            .collect();
    }
    let s = 1.0 / (num_nodes as f64 - 1.0);
    graph
        .get_all_nodes()
        .iter()
        .map(|n| {
            (
                n.name.clone(),
                graph.get_node_degree(n.name.clone()).unwrap() as f64 * s,
            )
        })
        .collect()
}
