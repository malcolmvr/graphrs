use crate::{Error, ErrorKind, Graph};
use nohash::IntSet;
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

pub(crate) fn is_partition_of_indexes<T, A>(
    graph: &Graph<T, A>,
    communities: &[IntSet<usize>],
) -> bool
where
    T: Hash + Eq + Clone + Ord + Display + Send + Sync,
    A: Clone + Send + Sync,
{
    let num_nodes = graph.number_of_nodes();
    let node_indexes_count = communities
        .iter()
        .flatten()
        .copied()
        .filter(|n| *n < num_nodes)
        .collect::<IntSet<usize>>()
        .len();
    node_indexes_count == num_nodes
}

pub(crate) fn get_singleton_partition<T, A>(graph: &Graph<T, A>) -> Vec<IntSet<usize>>
where
    T: Hash + Eq + Clone + Ord + Display + Send + Sync,
    A: Clone,
{
    let partition: Vec<IntSet<usize>> = (0..graph.number_of_nodes())
        .into_iter()
        .map(|i| {
            let mut set = IntSet::default();
            set.insert(i);
            set
        })
        .collect();
    partition
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
                    convert_values_to_f64::<T>(graph.get_out_degree_for_all_nodes().unwrap()),
                    convert_values_to_f64::<T>(graph.get_in_degree_for_all_nodes().unwrap()),
                ),
            };
            let m: f64 = outd.values().sum();
            let norm = (1.0 / m).powf(2.0);
            (outd, ind, m, norm)
        }
        false => {
            let deg = match weighted {
                true => graph.get_weighted_degree_for_all_nodes(),
                false => convert_values_to_f64::<T>(graph.get_degree_for_all_nodes()),
            };
            let deg_sum: f64 = deg.values().sum();
            let m = deg_sum / 2.0;
            let norm = (1.0 / deg_sum).powf(2.0);
            (deg.clone(), deg, m, norm)
        }
    };
    let community_contribution = |community: &HashSet<T>| {
        let comm_vec: Vec<T> = community.iter().cloned().collect();
        let subgraph = graph.get_subgraph(&comm_vec).unwrap();
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

pub(crate) fn modularity_by_indexes<T, A>(
    graph: &Graph<T, A>,
    communities: &[IntSet<usize>],
    weighted: bool,
    resolution: Option<f64>,
) -> Result<f64, Error>
where
    T: Hash + Eq + Clone + Ord + Display + Send + Sync,
    A: Clone + Send + Sync,
{
    if !is_partition_of_indexes(graph, communities) {
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
                    graph.get_weighted_out_degree_for_all_node_indexes(),
                    graph.get_weighted_in_degree_for_all_node_indexes(),
                ),
                false => (
                    convert_values_to_f64_vec(graph.get_out_degree_for_all_node_indexes()),
                    convert_values_to_f64_vec(graph.get_in_degree_for_all_node_indexes()),
                ),
            };
            let m: f64 = outd.iter().sum();
            let norm = (1.0 / m).powf(2.0);
            (outd, ind, m, norm)
        }
        false => {
            let deg = match weighted {
                true => graph.get_weighted_degree_for_all_node_indexes(),
                false => convert_values_to_f64_vec(graph.get_degree_for_all_node_indexes()),
            };
            let deg_sum: f64 = deg.iter().sum();
            let m = deg_sum / 2.0;
            let norm = (1.0 / deg_sum).powf(2.0);
            (deg.clone(), deg, m, norm)
        }
    };
    let community_contribution = |community: &IntSet<usize>| {
        let comm_vec: Vec<usize> = community.iter().cloned().collect();
        let subgraph = graph.get_subgraph_by_indexes(&comm_vec).unwrap();
        let subgraph_edges = subgraph.get_all_edges();
        let subgraph_edges_weight = match weighted {
            true => subgraph_edges.iter().map(|e| e.weight).sum(),
            false => subgraph_edges.len() as f64,
        };
        let out_degree_sum: f64 = community.iter().map(|n| out_degree[*n]).sum();
        let in_degree_sum = match graph.specs.directed {
            true => community.iter().map(|n| in_degree[*n]).sum(),
            false => out_degree_sum,
        };
        subgraph_edges_weight / m
            - resolution.unwrap_or(1.0) * out_degree_sum * in_degree_sum * norm
    };
    Ok(communities.iter().map(community_contribution).sum())
}

pub(crate) fn partition_is_singleton(partition: &[IntSet<usize>], num_nodes: usize) -> bool {
    let len = partition.len();
    let flattened_len = partition.into_iter().flatten().count();
    flattened_len == len && len == num_nodes
}

pub(crate) fn partitions_eq(
    partition1: &Vec<IntSet<usize>>,
    partition2: &Vec<IntSet<usize>>,
) -> bool {
    let first_of_each_set1: Vec<&usize> = partition1
        .iter()
        .map(|hs| hs.iter().next().unwrap())
        .collect();
    let matching_partition2_indexes: Vec<usize> = first_of_each_set1
        .iter()
        .map(|i| partition2.iter().position(|hs| hs.contains(i)).unwrap())
        .collect();
    partition1
        .into_iter()
        .zip(matching_partition2_indexes)
        .all(|(hs1, i)| hs1 == &partition2[i])
}

/// Converts a graph partition of usize replacements of node names T to
/// a partition using the node names T.
pub(crate) fn convert_usize_partitions_vec_to_t<T, A>(
    partitions_vec: Vec<Vec<IntSet<usize>>>,
    graph: &Graph<T, A>,
) -> Vec<Vec<HashSet<T>>>
where
    T: Hash + Eq + Clone + Ord + Display + Send + Sync,
    A: Clone + Send + Sync,
{
    partitions_vec
        .into_iter()
        .map(|v| convert_usize_partitions_to_t(v, graph))
        .collect()
}

/// Converts a graph partition of usize replacements of node names T to
/// a partition using the node names T.
pub(crate) fn convert_usize_partitions_to_t<T, A>(
    partitions: Vec<IntSet<usize>>,
    graph: &Graph<T, A>,
) -> Vec<HashSet<T>>
where
    T: Hash + Eq + Clone + Ord + Display + Send + Sync,
    A: Clone + Send + Sync,
{
    partitions
        .into_iter()
        .map(|hs| {
            hs.into_iter()
                .map(|u| graph.get_node_by_index(&u).unwrap().name.clone())
                .collect::<HashSet<T>>()
        })
        .collect::<Vec<HashSet<T>>>()
}

fn convert_values_to_f64<T>(hashmap: HashMap<T, usize>) -> HashMap<T, f64>
where
    T: Eq + Hash,
{
    hashmap.into_iter().map(|(k, v)| (k, v as f64)).collect()
}

fn convert_values_to_f64_vec(values: Vec<usize>) -> Vec<f64> {
    values.into_iter().map(|v| v as f64).collect()
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::{Edge, Graph, GraphSpecs};
    use std::collections::HashMap;

    #[test]
    fn test_convert_values_to_f64() {
        let hashmap: HashMap<&str, usize> = vec![("a", 1), ("b", 2), ("c", 3)]
            .into_iter()
            .collect::<HashMap<&str, usize>>();
        let f64_hashmap = convert_values_to_f64(hashmap);
        assert_eq!(f64_hashmap.get("a").unwrap(), &1.0);
        assert_eq!(f64_hashmap.get("b").unwrap(), &2.0);
        assert_eq!(f64_hashmap.get("c").unwrap(), &3.0);
    }

    #[test]
    fn test_convert_values_to_f64_vec() {
        let values = vec![1, 2, 3];
        let f64_vec = convert_values_to_f64_vec(values);
        assert_eq!(f64_vec, vec![1.0, 2.0, 3.0]);
    }

    #[test]
    fn test_convert_usize_partitions_to_t() {
        let edges = vec![
            Edge::new("n1", "n2"),
            Edge::new("n3", "n4"),
            Edge::new("n5", "n6"),
        ];
        let graph: Graph<&str, ()> =
            Graph::new_from_nodes_and_edges(vec![], edges, GraphSpecs::undirected_create_missing())
                .unwrap();
        let partitions = vec![
            vec![0, 1].into_iter().collect(),
            vec![2, 3].into_iter().collect(),
            vec![4, 5].into_iter().collect(),
        ];
        let converted = convert_usize_partitions_to_t(partitions, &graph);
        let hs1: HashSet<&str> = vec!["n1", "n2"].into_iter().collect();
        let hs2: HashSet<&str> = vec!["n3", "n4"].into_iter().collect();
        let hs3: HashSet<&str> = vec!["n5", "n6"].into_iter().collect();
        assert_eq!(converted[0], hs1);
        assert_eq!(converted[1], hs2);
        assert_eq!(converted[2], hs3);
    }

    #[test]
    fn test_convert_usize_partitions_vec_to_t() {
        let edges = vec![
            Edge::new("n1", "n2"),
            Edge::new("n3", "n4"),
            Edge::new("n5", "n6"),
        ];
        let graph: Graph<&str, ()> =
            Graph::new_from_nodes_and_edges(vec![], edges, GraphSpecs::undirected_create_missing())
                .unwrap();
        let partitions = vec![
            vec![0, 1].into_iter().collect(),
            vec![2, 3].into_iter().collect(),
            vec![4, 5].into_iter().collect(),
        ];
        let converted = convert_usize_partitions_vec_to_t(vec![partitions], &graph);
        let hs1: HashSet<&str> = vec!["n1", "n2"].into_iter().collect();
        let hs2: HashSet<&str> = vec!["n3", "n4"].into_iter().collect();
        let hs3: HashSet<&str> = vec!["n5", "n6"].into_iter().collect();
        assert_eq!(converted[0][0], hs1);
        assert_eq!(converted[0][1], hs2);
        assert_eq!(converted[0][2], hs3);
    }

    #[test]
    fn test_partition_is_singleton() {
        let partition = vec![
            vec![0, 1].into_iter().collect(),
            vec![2, 3].into_iter().collect(),
            vec![4, 5].into_iter().collect(),
        ];
        assert!(!partition_is_singleton(&partition, 6));
        let partition = vec![
            vec![0].into_iter().collect(),
            vec![1].into_iter().collect(),
            vec![2].into_iter().collect(),
        ];
        assert!(partition_is_singleton(&partition, 3));
    }

    #[test]
    fn test_partitions_eq1() {
        let partition1 = vec![
            vec![0, 1].into_iter().collect(),
            vec![2, 3].into_iter().collect(),
            vec![4, 5].into_iter().collect(),
        ];
        let partition2 = vec![
            vec![0, 1].into_iter().collect(),
            vec![2, 3].into_iter().collect(),
            vec![4, 5].into_iter().collect(),
        ];
        assert!(partitions_eq(&partition1, &partition2));
    }

    #[test]
    fn test_partitions_eq2() {
        let partition1 = vec![
            vec![2, 3].into_iter().collect(),
            vec![0, 1].into_iter().collect(),
            vec![4, 5].into_iter().collect(),
        ];
        let partition2 = vec![
            vec![0, 1].into_iter().collect(),
            vec![2, 3].into_iter().collect(),
            vec![4, 5].into_iter().collect(),
        ];
        assert!(partitions_eq(&partition1, &partition2));
    }

    #[test]
    fn test_partitions_eq3() {
        let partition1 = vec![
            vec![0, 1, 2].into_iter().collect(),
            vec![3, 4, 5].into_iter().collect(),
        ];
        let partition2 = vec![
            vec![0, 1].into_iter().collect(),
            vec![2, 3].into_iter().collect(),
            vec![4, 5].into_iter().collect(),
        ];
        assert!(!partitions_eq(&partition1, &partition2));
    }
}
