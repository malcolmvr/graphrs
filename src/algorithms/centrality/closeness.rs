use crate::algorithms::shortest_path::dijkstra;
use crate::algorithms::shortest_path::ShortestPathInfo;
use crate::{Error, Graph};
use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;

/**
Compute closeness centrality for nodes.

Closeness centrality of a node u is the reciprocal of the average shortest path
distance to u over all n-1 reachable nodes.

# Arguments

* `graph`: a [Graph](../../../struct.Graph.html) instance
* `weighted`: set to `true` to use edge weights when computing the betweenness centrality
* `wf_improved`: if `true`, scale by the fraction of nodes reachable; this gives the
Wasserman and Faust improved formula. For single component graphs it is the same as the
original formula.

# Examples

```
use graphrs::{algorithms::{centrality::{closeness}}, generators};
let graph = generators::social::karate_club_graph();
let centralities = closeness::closeness_centrality(&graph, false, true);
```

# References

1. Linton C. Freeman: Centrality in networks: I. Conceptual clarification. Social Networks 1:215-239, 1979.
<https://doi.org/10.1016/0378-8733(78)90021-7>

2. pg. 201 of Wasserman, S. and Faust, K., Social Network Analysis: Methods and Applications, 1994,
Cambridge University Press.
*/
pub fn closeness_centrality<T, A>(
    graph: &Graph<T, A>,
    weighted: bool,
    wf_improved: bool,
) -> Result<HashMap<T, f64>, Error>
where
    T: Hash + Eq + Clone + Ord + Display + Send + Sync,
    A: Clone + Send + Sync,
{
    let mut the_graph = graph;
    let x: Graph<T, A>;
    if graph.specs.directed {
        x = graph.reverse().unwrap();
        the_graph = &x;
    }
    let num_nodes = the_graph.get_all_nodes().len();
    let all_pairs = dijkstra::all_pairs(the_graph, weighted, None, false);
    match all_pairs {
        Err(e) => Err(e),
        Ok(ap) => {
            let centralities = ap
                .into_iter()
                .map(|(k, v)| get_closeness_centrality_for_node(k, v, num_nodes, wf_improved))
                .collect();
            Ok(centralities)
        }
    }
}

/// Gets the closeness centrality for a single node.
fn get_closeness_centrality_for_node<T>(
    node_name: T,
    shortest_paths: HashMap<T, ShortestPathInfo<T>>,
    num_nodes: usize,
    wf_improved: bool,
) -> (T, f64) {
    let total: f64 = shortest_paths.values().map(|sp| sp.distance).sum();
    let centrality: f64 = match total > 0.0 && num_nodes > 1 {
        false => 0.0,
        true => {
            let mut x = (shortest_paths.len() as f64 - 1.0) / total;
            if wf_improved {
                let s = (shortest_paths.len() as f64 - 1.0) / ((num_nodes - 1) as f64);
                x *= s;
            }
            x
        }
    };
    (node_name, centrality)
}
