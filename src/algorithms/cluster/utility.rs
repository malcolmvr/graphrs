use crate::{ext::hashset::HashSetExt, Graph};
use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::hash::Hash;

/**
Gets the predecessors or successors of `node_name`, as a `HashSet`, that doesn't
contain `node_name`.

# Arguments

* `graph`: a [Graph](../../struct.Graph.html) instance
* `node_name`: The name of the node to get predecessors or successors of
* `preds`: If `true` returns predecessors, if `false` returns successors
*/
pub fn get_adjacent_nodes_without<T, A>(
    graph: &Graph<T, A>,
    node_name: &T,
    preds: bool,
) -> HashSet<T>
where
    T: Hash + Eq + Clone + Ord + Display + Send + Sync,
    A: Clone + Send + Sync,
{
    let f = match preds {
        true => Graph::get_predecessor_node_names,
        false => Graph::get_successor_node_names,
    };
    f(graph, node_name.clone())
        .unwrap()
        .into_iter()
        .cloned()
        .collect::<HashSet<T>>()
        .without(node_name)
}

#[inline]
pub fn get_normalized_edge_weight<T, A>(u: &T, v: &T, max_weight: &f64, graph: &Graph<T, A>) -> f64
where
    T: Hash + Eq + Clone + Ord + Display + Send + Sync,
    A: Clone + Send + Sync,
{
    match graph.get_edge(u.clone(), v.clone()) {
        Err(_) => 1.0 / max_weight,
        Ok(e) => e.weight / max_weight,
    }
}

// Returns the names of the neighbors of a list of nodes, as a `HashMap<T, HashSet<T>>`.
pub fn get_neighbors_of_nodes<T, A>(
    node_names: Option<&[T]>,
    graph: &Graph<T, A>,
) -> HashMap<T, HashSet<T>>
where
    T: Hash + Eq + Clone + Ord + Display + Send + Sync,
    A: Clone + Send + Sync,
{
    let all_node_names: Vec<T> = match node_names.is_none() || node_names.unwrap().is_empty() {
        true => graph.get_all_node_names().into_iter().cloned().collect(),
        false => node_names.unwrap().to_vec(),
    };
    all_node_names
        .into_iter()
        .map(|n| {
            let hs = graph
                .get_neighbor_nodes(n.clone())
                .unwrap()
                .into_iter()
                .map(|nn| nn.name.clone())
                .collect();
            (n, hs)
        })
        .collect()
}
