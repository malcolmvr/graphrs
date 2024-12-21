use crate::{
    algorithms::community::partitions, Edge, EdgeDedupeStrategy, Error, ErrorKind, Graph,
    GraphSpecs, Node,
};
use nohash::{IntMap, IntSet};
use rand::prelude::*;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashSet;
use std::fmt::Display;
use std::hash::Hash;
use std::sync::Arc;

/**
Returns the best partition of a graph, using the Louvain algorithm.

# Arguments

* `graph`: a [Graph](../../../struct.Graph.html) instance
* `weighted`: set to `true` to use edge weights when determining communities
* `resolution`: If less than 1.0 larger communities are favoured. If greater than 1.0 smaller communities are favoured.
* `threshold`: Determines how quickly the algorithms stops trying to find partitions with higher modularity. Higher values cause the algorithm to give up more quickly.
* `seed`: The Louvain algorithm implemented uses random number generators. Setting the `seed` causes consistent behaviour.

# Examples

```
use graphrs::{algorithms::{community}, generators};
let graph = generators::social::karate_club_graph();
let communities = community::louvain::louvain_communities(&graph, false, None, None, Some(1));
assert_eq!(communities.unwrap().len(), 4);
```
*/
pub fn louvain_communities<T, A>(
    graph: &Graph<T, A>,
    weighted: bool,
    resolution: Option<f64>,
    threshold: Option<f64>,
    seed: Option<u64>,
) -> Result<Vec<HashSet<T>>, Error>
where
    T: Hash + Eq + Clone + Ord + Display + Send + Sync,
    A: Clone + Send + Sync,
{
    let mut partitions = louvain_partitions(graph, weighted, resolution, threshold, seed)?;
    match partitions.is_empty() {
        false => Ok(partitions.pop().unwrap()),
        true => Err(Error {
            kind: ErrorKind::NoPartitions,
            message: "No partitions were found.".to_string(),
        }),
    }
}

/**
Returns the best partitions of a graph, using the Louvain algorithm.

# Arguments

* `graph`: a [Graph](../../../struct.Graph.html) instance
* `weighted`: set to `true` to use edge weights when determining communities
* `resolution`: If less than 1.0 larger communities are favoured. If greater than 1.0 smaller communities are favoured.
* `threshold`: Determines how quickly the algorithms stops trying to find partitions with higher modularity. Higher values cause the algorithm to give up more quickly.
* `seed`: The Louvain algorithm implemented uses random number generators. Setting the `seed` causes consistent behaviour.__rust_force_expr!

# Examples

```
use graphrs::{algorithms::{community}, generators};
let graph = generators::social::karate_club_graph();
let partitions = community::louvain::louvain_partitions(&graph, false, None, None, Some(1));
```
*/
pub fn louvain_partitions<T, A>(
    graph: &Graph<T, A>,
    weighted: bool,
    resolution: Option<f64>,
    threshold: Option<f64>,
    seed: Option<u64>,
) -> Result<Vec<Vec<HashSet<T>>>, Error>
where
    T: Hash + Eq + Clone + Ord + Display + Send + Sync,
    A: Clone + Send + Sync,
{
    let _threshold = threshold.unwrap_or(0.0000001);
    let partition: Vec<IntSet<usize>> = (0..graph.number_of_nodes())
        .into_iter()
        .map(|i| {
            let mut set = IntSet::default();
            set.insert(i);
            set
        })
        .collect();
    let mut modularity =
        partitions::modularity_by_indexes(&graph, &partition, weighted, resolution).unwrap();
    let m = graph.size(weighted);
    let mut graph_com = convert_graph(&graph, weighted);
    let (mut partition, mut inner_partition, _improvement) =
        compute_one_level(&graph_com, m, &partition, resolution.unwrap_or(1.0), seed);
    let mut improvement = true;
    let mut partitions: Vec<Vec<IntSet<usize>>> = vec![];
    while improvement {
        partitions.push(partition.to_vec());
        let new_mod =
            partitions::modularity_by_indexes(&graph_com, &inner_partition, weighted, resolution)
                .unwrap();
        if new_mod - modularity <= _threshold {
            return Ok(convert_usize_partitons_to_t(partitions, &graph));
        }
        modularity = new_mod;
        graph_com = generate_graph(&graph_com, inner_partition);
        let z = compute_one_level(&graph_com, m, &partition, resolution.unwrap_or(1.0), seed);
        partition = z.0;
        inner_partition = z.1;
        improvement = z.2;
    }
    Ok(convert_usize_partitons_to_t(partitions, &graph))
}

/// Converts a graph partition of usize replacements of node names T to
/// a partition using the node names T.
fn convert_usize_partitons_to_t<T, A>(
    partition: Vec<Vec<IntSet<usize>>>,
    graph: &Graph<T, A>,
) -> Vec<Vec<HashSet<T>>>
where
    T: Hash + Eq + Clone + Ord + Display + Send + Sync,
    A: Clone + Send + Sync,
{
    partition
        .into_iter()
        .map(|v| {
            v.into_iter()
                .map(|hs| {
                    hs.into_iter()
                        .map(|u| graph.get_node_by_index(&u).unwrap().name.clone())
                        .collect::<HashSet<T>>()
                })
                .collect::<Vec<HashSet<T>>>()
        })
        .collect()
}

/// Calculate one level of the Louvain partitions tree.
#[allow(clippy::ptr_arg)]
fn compute_one_level(
    graph: &Graph<usize, IntSet<usize>>,
    m: f64,
    partition: &Vec<IntSet<usize>>,
    resolution: f64,
    seed: Option<u64>,
) -> (Vec<IntSet<usize>>, Vec<IntSet<usize>>, bool) {
    let mut node2com: Vec<usize> = (0..graph.number_of_nodes()).collect();
    let mut _partition = partition.clone();
    let mut inner_partition = map_node_indexes_to_hashsets(graph);
    let mut deg_info = get_degree_information(graph, partition);
    let shuffled_indexes = get_shuffled_node_indexes(graph, seed);
    // let shuffled_indexes: Vec<usize> = (0..graph.number_of_nodes()).collect();
    let mut nb_moves = 1;
    let mut improvement = false;
    while nb_moves > 0 {
        nb_moves = 0;
        for u in &shuffled_indexes {
            let mut best_mod = 0.0;
            let mut best_com: usize = node2com[*u];
            let weights2com = get_neighbor_weights(graph, u, &node2com);
            subtract_degree_from_best_com(
                best_com,
                u,
                &weights2com,
                m,
                resolution,
                &mut deg_info,
                graph.specs.directed,
            );
            #[rustfmt::skip]
            update_best_com(&mut best_com, &mut best_mod, weights2com, &deg_info, m, resolution, graph.specs.directed);
            add_degree_to_best_com(best_com, &mut deg_info, graph.specs.directed);
            if best_com != node2com[*u] {
                let node_hs = vec![u].into_iter().copied().collect::<IntSet<usize>>();
                let com = graph
                    .get_node_by_index(u)
                    .unwrap()
                    .attributes
                    .clone()
                    .unwrap_or(node_hs);
                let n2c = node2com[*u];
                _partition[n2c] = _partition[n2c].difference(&com).cloned().collect();
                inner_partition[n2c].remove(u);
                _partition[best_com] = _partition[best_com].union(&com).cloned().collect();
                inner_partition[best_com].insert(*u);
                node2com[*u] = best_com;
                improvement = true;
                nb_moves += 1;
            }
        }
    }
    let new_partition: Vec<IntSet<usize>> = _partition
        .into_iter()
        .filter(|part| !part.is_empty())
        .collect();
    let new_inner_partition: Vec<IntSet<usize>> = inner_partition
        .into_iter()
        .filter(|part| !part.is_empty())
        .collect();
    (new_partition, new_inner_partition, improvement)
}

/// Returns a random number generator (RNG), optionally seeded.
fn get_rng(seed: Option<u64>) -> StdRng {
    match seed {
        None => {
            let mut trng = thread_rng();
            StdRng::seed_from_u64(trng.next_u64())
        }
        Some(s) => StdRng::seed_from_u64(s),
    }
}

/// Returns all the node indexes in `graph`, shuffled randomly.
fn get_shuffled_node_indexes<T, A>(graph: &Graph<T, A>, seed: Option<u64>) -> Vec<usize>
where
    T: Hash + Eq + Clone + Ord + Display + Send + Sync,
    A: Clone + Send + Sync,
{
    let mut rng = get_rng(seed);
    let mut indexes: Vec<usize> = (0..graph.number_of_nodes()).collect();
    indexes.shuffle(&mut rng);
    indexes
}

#[inline]
fn add_degree_to_best_com(best_com: usize, deg_info: &mut DegreeInfo, directed: bool) {
    match directed {
        true => {
            deg_info.stot_in[best_com] += deg_info.in_degree;
            deg_info.stot_out[best_com] += deg_info.out_degree;
        }
        false => {
            deg_info.stot[best_com] += deg_info.degree;
        }
    }
}

fn update_best_com(
    best_com: &mut usize,
    best_mod: &mut f64,
    weights2com: IntMap<usize, f64>,
    deg_info: &DegreeInfo,
    m: f64,
    resolution: f64,
    directed: bool,
) {
    for (nbr_com, wt) in weights2com {
        let gain = match directed {
            true => {
                deg_info.remove_cost + wt / m
                    - resolution
                        * (deg_info.out_degree * deg_info.stot_in[nbr_com]
                            + deg_info.in_degree * deg_info.stot_out[nbr_com])
                        / m.powf(2.0)
            }
            false => {
                deg_info.remove_cost + wt / m
                    - resolution * (deg_info.stot[nbr_com] * deg_info.degree) / (2.0 * m.powf(2.0))
            }
        };
        if gain > *best_mod {
            *best_mod = gain;
            *best_com = nbr_com;
        }
    }
}

#[inline]
fn subtract_degree_from_best_com(
    best_com: usize,
    u: &usize,
    weights2com: &IntMap<usize, f64>,
    m: f64,
    resolution: f64,
    deg_info: &mut DegreeInfo,
    directed: bool,
) {
    match directed {
        true => {
            deg_info.in_degree = deg_info.in_degrees[*u];
            deg_info.out_degree = deg_info.out_degrees[*u];
            deg_info.stot_in[best_com] -= deg_info.in_degree;
            deg_info.stot_out[best_com] -= deg_info.out_degree;
            deg_info.remove_cost = -weights2com.get(&best_com).unwrap_or(&0.0) / m
                + resolution
                    * (deg_info.out_degree * deg_info.stot_in[best_com]
                        + deg_info.in_degree * deg_info.stot_out[best_com])
                    / m.powf(2.0);
        }
        false => {
            deg_info.degree = deg_info.degrees[*u];
            deg_info.stot[best_com] -= deg_info.degree;
            deg_info.remove_cost = -weights2com.get(&best_com).unwrap_or(&0.0) / m
                + resolution * (deg_info.stot[best_com] * deg_info.degree) / (2.0 * m.powf(2.0));
        }
    }
}

/// Holds information about the degrees of nodes of a graph.
#[derive(Debug)]
struct DegreeInfo {
    pub in_degrees: Vec<f64>,
    pub out_degrees: Vec<f64>,
    pub stot_in: Vec<f64>,
    pub stot_out: Vec<f64>,
    pub degrees: Vec<f64>,
    pub stot: Vec<f64>,
    pub degree: f64,
    pub in_degree: f64,
    pub out_degree: f64,
    pub remove_cost: f64,
}

/// Gets information about the degrees of nodes of a graph.
fn get_degree_information<T, A>(graph: &Graph<T, A>, partition: &[IntSet<usize>]) -> DegreeInfo
where
    T: Hash + Eq + Clone + Ord + Display + Send + Sync,
    A: Clone + Send + Sync,
{
    let mut in_degrees = vec![];
    let mut out_degrees = vec![];
    let mut stot_in: Vec<f64> = vec![];
    let mut stot_out: Vec<f64> = vec![];
    let mut degrees = vec![];
    let mut stot: Vec<f64> = vec![];

    if graph.specs.directed {
        // the `get_weighted_*` methods can be used here, whether or not the original graph
        // was weighted because `set_all_edge_weights` has been called in `louvain_partitions`
        in_degrees = graph.get_weighted_in_degree_for_all_node_indexes();
        out_degrees = graph.get_weighted_out_degree_for_all_node_indexes();
        stot_in = (0..partition.len())
            .into_iter()
            .map(|i| in_degrees[i])
            .collect();
        stot_out = (0..partition.len())
            .into_iter()
            .map(|i| out_degrees[i])
            .collect();
    } else {
        degrees = graph.get_weighted_degree_for_all_node_indexes();
        stot = (0..partition.len())
            .into_iter()
            .map(|i| degrees[i])
            .collect();
    }

    DegreeInfo {
        in_degrees,
        out_degrees,
        stot_in,
        stot_out,
        degrees,
        stot,
        degree: 0.0,
        in_degree: 0.0,
        out_degree: 0.0,
        remove_cost: 0.0,
    }
}

/// Converts a graph of node named by `T` to a graph where nodes are named
/// with `usize` values instead. It uses the `node_map` to perform the T to
/// usize replacements.
fn convert_graph<T, A>(graph: &Graph<T, A>, weighted: bool) -> Graph<usize, IntSet<usize>>
where
    T: Hash + Eq + Clone + Ord + Display + Send + Sync,
    A: Clone + Send + Sync,
{
    let mut converted_graph: Graph<T, A>;
    let graph = match graph.specs.multi_edges {
        true => {
            converted_graph = graph.to_single_edges().unwrap();
            &converted_graph
        }
        false => graph,
    };
    let graph = match weighted {
        false => {
            converted_graph = graph.set_all_edge_weights(1.0);
            &converted_graph
        }
        true => graph,
    };
    let nodes = graph
        .get_all_nodes()
        .iter()
        .map(|n| {
            let u = graph.get_node_index(&n.name).unwrap();
            Node::from_name_and_attributes(u, vec![u].into_iter().collect::<IntSet<usize>>())
        })
        .collect::<Vec<_>>();
    let edges = graph
        .get_all_edges()
        .iter()
        .map(|e| {
            Arc::new(Edge {
                u: graph.get_node_index(&e.u).unwrap(),
                v: graph.get_node_index(&e.v).unwrap(),
                weight: e.weight,
                attributes: None,
            })
        })
        .collect();
    Graph::new_from_nodes_and_edges(nodes, edges, graph.specs.clone()).unwrap()
}

/// Generates a new graph based on the partitions of a given graph.
fn generate_graph(
    graph: &Graph<usize, IntSet<usize>>,
    partition: Vec<IntSet<usize>>,
) -> Graph<usize, IntSet<usize>> {
    let mut new_graph = Graph::new(GraphSpecs {
        self_loops: true,
        edge_dedupe_strategy: EdgeDedupeStrategy::KeepLast,
        ..graph.specs.clone()
    });
    let mut node2com = IntMap::<usize, usize>::default();
    partition.iter().enumerate().for_each(|(i, part)| {
        let mut nodes = IntSet::<usize>::default();
        for node in part {
            *node2com.entry(node.clone()).or_insert(0) = i;
            let node_object = graph.get_node(node.clone()).unwrap();
            let node_hs = vec![node].into_iter().cloned().collect::<IntSet<usize>>();
            let to_extend = node_object.attributes.clone().unwrap_or(node_hs);
            nodes.extend(to_extend);
        }
        new_graph.add_node(Node::from_name_and_attributes(i, nodes));
    });
    graph.get_all_edges().iter().for_each(|e| {
        let com1 = node2com.get(&e.u).unwrap();
        let com2 = node2com.get(&e.v).unwrap();
        let new_graph_edge_weight = new_graph
            .get_edge(*com1, *com2)
            .unwrap_or(&Edge::with_weight(*com1, *com2, 0.0))
            .weight;
        new_graph
            .add_edge(Edge::with_weight(
                *com1,
                *com2,
                e.weight + new_graph_edge_weight,
            ))
            .expect("unexpected failure to add edge");
    });
    new_graph
}

/// For a given node `u` returns all the weights of the edges to its neighbors.
fn get_neighbor_weights<T, A>(
    graph: &Graph<T, A>,
    u: &usize,
    node2com: &Vec<usize>,
) -> IntMap<usize, f64>
where
    T: Hash + Eq + Clone + Ord + Display + Send + Sync,
    A: Clone + Send + Sync,
{
    let mut hm: IntMap<usize, f64> = IntMap::default();
    let adjacent = match graph.specs.directed {
        true => &graph
            .get_successor_nodes_by_index(u)
            .into_iter()
            .chain(graph.get_predecessor_nodes_by_index(u).into_iter())
            .collect::<Vec<_>>(),
        false => &graph
            .get_successor_nodes_by_index(u)
            .iter()
            .collect::<Vec<_>>(),
    };
    for adj in adjacent {
        if *u == adj.node_index {
            continue;
        }
        let weight = match adj.weight.is_nan() {
            true => 1.0,
            false => adj.weight,
        };
        *hm.entry(node2com[adj.node_index]).or_default() += weight;
    }
    hm
}

/// Creates the initial mapping of node names in the `graph` to
/// a vector where each item contains an IntSet that contains a single
/// node index.
fn map_node_indexes_to_hashsets<T, A>(graph: &Graph<T, A>) -> Vec<IntSet<usize>>
where
    T: Hash + Eq + Clone + Ord + Display + Send + Sync,
    A: Clone + Send + Sync,
{
    (0..graph.number_of_nodes())
        .into_iter()
        .map(|i| {
            let mut hs = IntSet::default();
            hs.insert(i);
            hs
        })
        .collect()
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::{Edge, Graph, GraphSpecs};
    use itertools::Itertools;

    #[rustfmt::skip]
    #[test]
    fn test_convert_graph() {
        let mut graph = Graph::<&str, ()>::new(GraphSpecs {multi_edges: true, ..GraphSpecs::directed_create_missing()});
        graph.add_edges(vec![
            Edge::with_weight("n1", "n2", 1.0),
            Edge::with_weight("n1", "n2", 1.1),
            Edge::with_weight("n2", "n1", 1.2),
            Edge::with_weight("n1", "n3", 1.3),
            Edge::with_weight("n1", "n4", 1.4),
            Edge::with_weight("n4", "n3", 1.5),
        ]).expect("couldn't add edges");
        let converted_graph = convert_graph(&graph, true);
        assert_eq!(converted_graph.get_all_nodes().iter().map(|n| n.name).sorted().collect::<Vec<usize>>(), vec![0, 1, 2, 3]);
        assert_eq!(converted_graph.get_node(1).unwrap().attributes.clone().unwrap().len(), 1);
        assert_eq!(converted_graph.get_all_edges().len(), 5);
        assert_eq!(converted_graph.get_edge(0, 1).unwrap().weight, 2.1);
        assert_eq!(converted_graph.get_edge(1, 0).unwrap().weight, 1.2);
        assert_eq!(converted_graph.get_edge(0, 2).unwrap().weight, 1.3);
        assert_eq!(converted_graph.get_edge(0, 3).unwrap().weight, 1.4);
        assert_eq!(converted_graph.get_edge(3, 2).unwrap().weight, 1.5);
    }

    #[rustfmt::skip]
    #[test]
    fn test_generate_graph() {
        let mut graph = Graph::new(GraphSpecs::directed_create_missing());
        graph.add_edges(vec![
            Edge::with_weight(0, 1, 1.1),
            Edge::with_weight(1, 0, 1.2),
            Edge::with_weight(0, 2, 1.3),
            Edge::with_weight(0, 3, 1.4),
            Edge::with_weight(3, 2, 1.5),
        ]).expect("couldn't add edges");
        let communities = vec![
            vec![0, 1].into_iter().collect(),
            vec![2, 3].into_iter().collect(),
        ];
        let gen_graph = generate_graph(&graph, communities);
        assert_eq!(gen_graph.get_all_nodes().iter().map(|n| n.name).sorted().collect::<Vec<usize>>(), vec![0, 1]);
        assert_eq!(gen_graph.get_node(0).unwrap().attributes.as_ref().unwrap().iter().copied().sorted().collect::<Vec<usize>>(), vec![0, 1]);
        assert_eq!(gen_graph.get_node(1).unwrap().attributes.as_ref().unwrap().iter().copied().sorted().collect::<Vec<usize>>(), vec![2, 3]);
        assert_eq!(gen_graph.get_all_edges().len(), 3);
        assert_eq!(gen_graph.get_edge(0, 0).unwrap().weight, 2.3);
        assert_eq!(gen_graph.get_edge(0, 1).unwrap().weight, 2.7);
        assert_eq!(gen_graph.get_edge(1, 1).unwrap().weight, 1.5);
    }

    #[rustfmt::skip]
    #[test]
    fn test_get_neighbor_weights() {
        let mut graph: Graph<&str, ()> = Graph::new(GraphSpecs::directed_create_missing());
        graph.add_edges(vec![
            Edge::with_weight("n1", "n2", 1.1),
            Edge::with_weight("n2", "n1", 1.2),
            Edge::with_weight("n1", "n3", 1.3),
            Edge::with_weight("n1", "n4", 1.4),
            Edge::with_weight("n4", "n3", 1.5),
        ]).expect("couldn't add edges");
        let node2com = vec![0, 0, 2, 2];
        let weights = get_neighbor_weights(&graph, &0, &node2com);
        assert_eq!(weights.len(), 2);
        assert_eq!(weights.get(&0).unwrap(), &2.3);
        assert_eq!(weights.get(&2).unwrap(), &2.7);
    }

    #[rustfmt::skip]
    #[test]
    fn test_map_node_indexes_to_hashsets() {
        let mut graph: Graph<usize, HashSet<usize>> = Graph::new(GraphSpecs::directed_create_missing());
        graph.add_edges(vec![
            Edge::with_weight(1, 2, 1.1),
            Edge::with_weight(2, 1, 1.2),
            Edge::with_weight(1, 3, 1.3),
            Edge::with_weight(1, 4, 1.4),
            Edge::with_weight(4, 3, 1.5),
        ]).expect("couldn't add edges");
        let names = map_node_indexes_to_hashsets(&graph);
        assert_eq!(names.iter().map(|hs| hs.len()).collect::<Vec<usize>>(), vec![1, 1, 1, 1]);
        assert_eq!(
            names.into_iter().flat_map(|hs| hs.into_iter().collect::<Vec<usize>>())
                 .sorted().collect::<Vec<usize>>()
            , vec![0, 1, 2, 3]
        );
    }
}
