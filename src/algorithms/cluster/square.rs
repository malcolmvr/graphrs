use crate::{ext::hashset::HashSetExt, Graph};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::hash::Hash;

/**
Returns the squares clustering coefficient for nodes in a graph.

# Arguments

* `graph`: a [Graph](../../struct.Graph.html) instance
* `node_names`: node names that optionally define a subset of the graph to work with

# Examples

```
use graphrs::{algorithms::cluster, generators};
let graph = generators::social::karate_club_graph();
let result = cluster::square_clustering(&graph, None);
```
 */
pub fn square_clustering<T, A>(graph: &Graph<T, A>, node_names: Option<&[T]>) -> HashMap<T, f64>
where
    T: Hash + Eq + Clone + Ord + Display + Send + Sync,
    A: Clone + Send + Sync,
{
    let ns: Vec<T> = match node_names {
        None => graph.get_all_nodes().into_iter().map(|n| n.name.clone()).collect(),
        Some(names) => names.to_vec(),
    };
    ns.into_iter().map(|v| get_coefficient_for_node(v, graph)).collect()
}

///////////////////////
/// PRIVATE METHODS ///
///////////////////////

fn get_coefficient_for_node<T, A>(v: T, graph: &Graph<T, A>) -> (T, f64)
where
    T: Hash + Eq + Clone + Ord + Display + Send + Sync,
    A: Clone + Send + Sync,
{
    let nbrs = graph.get_successors_or_neighbors(v.clone());
    let (clustering_v, potential) = nbrs
        .into_iter()
        .combinations(2)
        .map(|c| {
            get_coefficient_for_combination(v.clone(), c[0].name.clone(), c[1].name.clone(), graph)
        })
        .fold((0, 0), |acc: (usize, usize), v: (usize, usize)| {
            (acc.0 + v.0, acc.1 + v.1)
        });
    match potential > 0 {
        true => (v, clustering_v as f64 / potential as f64),
        false => (v, clustering_v as f64),
    }
}

fn get_coefficient_for_combination<T, A>(v: T, u: T, w: T, graph: &Graph<T, A>) -> (usize, usize)
where
    T: Hash + Eq + Clone + Ord + Display + Send + Sync,
    A: Clone + Send + Sync,
{
    let u_nbrs = gnos(u, graph);
    let w_nbrs = gnos(w.clone(), graph);
    let squares = u_nbrs.intersection(&w_nbrs).collect::<HashSet<&T>>().without(&&v).len();
    let degm = match u_nbrs.contains(&w) {
        false => squares + 1,
        true => squares + 2,
    };
    let potential = (u_nbrs.len() - degm) + (w_nbrs.len() - degm) + squares;
    (squares, potential)
}

/// Returns successor or neighbor node names, as a HashSet.
#[inline]
fn gnos<T, A>(nn: T, graph: &Graph<T, A>) -> HashSet<T>
where
    T: Hash + Eq + Clone + Ord + Display + Send + Sync,
    A: Clone + Send + Sync,
{
    graph
        .get_successors_or_neighbors(nn)
        .into_iter()
        .map(|n| n.name.clone())
        .collect::<HashSet<T>>()
}
