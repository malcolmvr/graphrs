use crate::{Error, ErrorKind, Graph};
use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::hash::Hash;

/**
Determines if the union of communities contains all nodes in the graph.

# Arguments

* `graph`: a [Graph](../../../struct.Graph.html) instance
* `communities`: a `Vec` of `HashSet`s of node names

# Examples

```
use graphrs::{algorithms::community::partitions, Edge, Graph, GraphSpecs};
use std::collections::HashSet;
let edges = vec![
    Edge::new("n1", "n2"),
    Edge::new("n3", "n4"),
];
let graph: Graph<&str, ()> =
    Graph::new_from_nodes_and_edges(vec![], edges, GraphSpecs::undirected_create_missing())
        .unwrap();
let hs1: HashSet<&str> = vec!["n1", "n3"].into_iter().collect();
let hs2: HashSet<&str> = vec!["n2", "n4"].into_iter().collect();
assert!(partitions::is_partition(&graph, &vec![hs1, hs2]));
```
*/
pub fn is_partition<T, A>(graph: &Graph<T, A>, communities: &[HashSet<T>]) -> bool
where
    T: Hash + Eq + Clone + Ord + Display + Send + Sync,
    A: Clone + Send + Sync,
{
    let node_names_count = communities
        .iter()
        .flatten()
        .filter(|n| graph.get_node((*n).clone()).is_some())
        .count();
    let sum_names = communities.iter().map(|hs| hs.len()).sum::<usize>();
    let all_nodes_len = graph.get_all_nodes().len();
    node_names_count == all_nodes_len && sum_names == all_nodes_len
}

/**
Compute the modularity of the given graph partitions.

# Arguments

* `graph`: a [Graph](../../../struct.Graph.html) instance
* `communities`: a `Vec` of `HashSet`s of node names
* `weighted`: determines if edge weights are used to compute the modularity
* `resolution`: if resolution is less than 1, modularity favors larger communities;
greater than 1 favors smaller communities.

# Examples

```
use graphrs::{algorithms::community::partitions, generators};
let graph = generators::social::karate_club_graph();
let communities = vec![
    vec![0, 1, 2, 3, 4, 5].into_iter().collect(),
    vec![6, 7, 8, 9, 10, 11].into_iter().collect(),
    vec![12, 13, 14, 15, 16, 17].into_iter().collect(),
    vec![18, 19, 20, 21, 22, 23].into_iter().collect(),
    vec![24, 25, 26, 27, 28, 29].into_iter().collect(),
    vec![30, 31, 32, 33].into_iter().collect(),
];
let modularity = partitions::modularity(&graph, &communities, false, None).unwrap();
```
*/
pub fn modularity<T, A>(
    graph: &Graph<T, A>,
    communities: &[HashSet<T>],
    weighted: bool,
    resolution: Option<f64>,
) -> Result<f64, Error>
where
    T: Hash + Eq + Clone + Ord + Display + Send + Sync,
    A: Clone + Send + Sync,
{
    if !is_partition(graph, communities) {
        return Err(Error {
            kind: ErrorKind::NotAPartition,
            message: "The specified communities did not form a partition of a Graph.".to_string(),
        });
    }
    // compute four variables depending on whether or not the graph is directed and `weighted` is true/false
    let (out_degree, in_degree, m, norm) = match graph.specs.directed {
        true => {
            let (outd, ind) = match weighted {
                true => (
                    graph.get_weighted_out_degree_for_all_nodes().unwrap(),
                    graph.get_weighted_in_degree_for_all_nodes().unwrap(),
                ),
                false => (
                    convert_values_to_f64::<T, A>(graph.get_out_degree_for_all_nodes().unwrap()),
                    convert_values_to_f64::<T, A>(graph.get_in_degree_for_all_nodes().unwrap()),
                ),
            };
            let m: f64 = outd.values().sum();
            let norm = (1.0 / m).powf(2.0);
            (outd, ind, m, norm)
        }
        false => {
            let deg = match weighted {
                true => graph.get_weighted_degree_for_all_nodes(),
                false => convert_values_to_f64::<T, A>(graph.get_degree_for_all_nodes()),
            };
            let deg_sum: f64 = deg.values().sum();
            let m = deg_sum / 2.0;
            let norm = (1.0 / deg_sum).powf(2.0);
            (deg.clone(), deg, m, norm)
        }
    };
    let community_contribution = |community: &HashSet<T>| {
        let comm_vec: Vec<T> = community.iter().cloned().collect();
        let subgraph = graph.get_subgraph(&comm_vec);
        let subgraph_edges = subgraph.get_all_edges();
        let subgraph_edges_weight = match weighted {
            true => subgraph_edges.iter().map(|e| e.weight).sum(),
            false => subgraph_edges.len() as f64,
        };
        let out_degree_sum: f64 = community.iter().map(|n| out_degree.get(n).unwrap()).sum();
        let in_degree_sum = match graph.specs.directed {
            true => community.iter().map(|n| in_degree.get(n).unwrap()).sum(),
            false => out_degree_sum,
        };
        subgraph_edges_weight / m
            - resolution.unwrap_or(1.0) * out_degree_sum * in_degree_sum * norm
    };
    Ok(communities.iter().map(community_contribution).sum())
}

fn convert_values_to_f64<T, A>(hashmap: HashMap<T, usize>) -> HashMap<T, f64>
where
    T: Eq + Hash,
{
    hashmap.into_iter().map(|(k, v)| (k, v as f64)).collect()
}
