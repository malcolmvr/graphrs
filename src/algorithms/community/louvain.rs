use crate::{
    algorithms::community::partitions, Edge, EdgeDedupeStrategy, Error, ErrorKind, Graph,
    GraphSpecs, Node,
};
use itertools::Itertools;
use rand::prelude::*;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::hash::Hash;

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
    let node_map = graph
        .get_all_nodes()
        .iter()
        .map(|n| n.name.clone())
        .sorted()
        .enumerate()
        .map(|(i, n)| (n, i))
        .collect::<HashMap<T, usize>>();
    let mut graphu = convert_graph(graph, weighted, &node_map);
    let partition = map_node_names_to_hashsets(&graphu);
    let mut modularity = partitions::modularity(&graphu, &partition, weighted, resolution).unwrap();
    let m = graphu.size(weighted);
    let (mut partition, mut inner_partition, _improvement) =
        compute_one_level(&graphu, m, &partition, resolution.unwrap_or(1.0), seed);
    let mut improvement = true;
    let mut partitions: Vec<Vec<HashSet<usize>>> = vec![];
    while improvement {
        partitions.push(partition.to_vec());
        let new_mod =
            partitions::modularity(&graphu, &inner_partition, weighted, resolution).unwrap();
        if new_mod - modularity <= _threshold {
            return Ok(convert_usize_partitons_to_t(partitions, &node_map));
        }
        modularity = new_mod;
        graphu = generate_graph(&graphu, inner_partition);
        let z = compute_one_level(&graphu, m, &partition, resolution.unwrap_or(1.0), seed);
        partition = z.0;
        inner_partition = z.1;
        improvement = z.2;
    }
    Ok(convert_usize_partitons_to_t(partitions, &node_map))
}

/// Converts a graph partition of usize replacements of node names T to
/// a partition using the node names T.
fn convert_usize_partitons_to_t<T>(
    partition: Vec<Vec<HashSet<usize>>>,
    node_map: &HashMap<T, usize>,
) -> Vec<Vec<HashSet<T>>>
where
    T: Hash + Eq + Clone + Ord + Display + Send + Sync,
{
    let reverse_node_map = node_map
        .iter()
        .map(|(k, v)| (*v, k.clone()))
        .collect::<HashMap<usize, T>>();
    partition
        .into_iter()
        .map(|v| {
            v.into_iter()
                .map(|hs| {
                    hs.into_iter()
                        .map(|u| reverse_node_map.get(&u).unwrap().clone())
                        .collect::<HashSet<T>>()
                })
                .collect::<Vec<HashSet<T>>>()
        })
        .collect()
}

/// Calculate one level of the Louvain partitions tree.
#[allow(clippy::ptr_arg)]
fn compute_one_level(
    graph: &Graph<usize, HashSet<usize>>,
    m: f64,
    partition: &Vec<HashSet<usize>>,
    resolution: f64,
    seed: Option<u64>,
) -> (Vec<HashSet<usize>>, Vec<HashSet<usize>>, bool) {
    let mut _partition = partition.clone();
    let mut node2com: HashMap<usize, usize> = graph
        .get_all_nodes()
        .iter()
        .map(|n| n.name)
        .sorted()
        .map(|n| (n, n))
        .collect();
    let mut inner_partition = map_node_names_to_hashsets(graph);
    let mut deg_info = get_degree_information(graph, partition);
    let nbrs = graph.get_successors_map();
    let shuffled_nodes = get_shuffled_node_names(graph, seed);
    let mut nb_moves = 1;
    let mut improvement = false;
    while nb_moves > 0 {
        nb_moves = 0;
        for u in &shuffled_nodes {
            let mut best_mod = 0.0;
            let mut best_com: usize = *node2com.get(u).unwrap();
            let weights2com = get_neighbor_weights(graph, u, nbrs, &node2com);
            subtract_degree_from_best_com(best_com, u, &mut deg_info, graph.specs.directed);
            #[rustfmt::skip]
            update_best_com(&mut best_com, &mut best_mod, weights2com, &deg_info, m, resolution, graph.specs.directed);
            add_degree_to_best_com(best_com, &mut deg_info, graph.specs.directed);
            if best_com != *node2com.get(u).unwrap() {
                let node_hs = vec![u].into_iter().copied().collect::<HashSet<usize>>();
                let com = graph
                    .get_node(*u)
                    .unwrap()
                    .attributes
                    .clone()
                    .unwrap_or(node_hs);
                let n2c = *node2com.get(u).unwrap();
                _partition[n2c] = _partition[n2c].difference(&com).cloned().collect();
                inner_partition[n2c].remove(u);
                _partition[best_com] = _partition[best_com].union(&com).cloned().collect();
                inner_partition[best_com].insert(*u);
                *node2com.entry(*u).or_default() = best_com;
                improvement = true;
                nb_moves += 1;
            }
        }
    }
    let new_partition: Vec<HashSet<usize>> = _partition
        .into_iter()
        .filter(|part| !part.is_empty())
        .collect();
    let new_inner_partition: Vec<HashSet<usize>> = inner_partition
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

/// Returns all the node names in `graph`, shuffled randomly.
fn get_shuffled_node_names(graph: &Graph<usize, HashSet<usize>>, seed: Option<u64>) -> Vec<usize> {
    let mut rng = get_rng(seed);
    let mut shuffled_nodes: Vec<usize> = graph.get_all_nodes().iter().map(|n| n.name).collect();
    shuffled_nodes.shuffle(&mut rng);
    shuffled_nodes
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
    weights2com: HashMap<usize, f64>,
    deg_info: &DegreeInfo,
    m: f64,
    resolution: f64,
    directed: bool,
) {
    for (nbr_com, wt) in weights2com {
        let gain = match directed {
            true => {
                wt - resolution
                    * (deg_info.out_degree * deg_info.stot_in[nbr_com]
                        + deg_info.in_degree * deg_info.stot_out[nbr_com])
                    / m
            }
            false => 2.0 * wt - resolution * (deg_info.stot[nbr_com] * deg_info.degree) / m,
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
    deg_info: &mut DegreeInfo,
    directed: bool,
) {
    match directed {
        true => {
            deg_info.in_degree = *deg_info.in_degrees.get(u).unwrap();
            deg_info.out_degree = *deg_info.out_degrees.get(u).unwrap();
            deg_info.stot_in[best_com] -= deg_info.in_degree;
            deg_info.stot_out[best_com] -= deg_info.out_degree;
        }
        false => {
            deg_info.degree = *deg_info.degrees.get(u).unwrap();
            deg_info.stot[best_com] -= deg_info.degree;
        }
    }
}

/// Holds information about the degrees of nodes of a graph.
struct DegreeInfo {
    pub in_degrees: HashMap<usize, f64>,
    pub out_degrees: HashMap<usize, f64>,
    pub stot_in: Vec<f64>,
    pub stot_out: Vec<f64>,
    pub degrees: HashMap<usize, f64>,
    pub stot: Vec<f64>,
    pub degree: f64,
    pub in_degree: f64,
    pub out_degree: f64,
}

/// Gets information about the degrees of nodes of a graph.
fn get_degree_information(
    graph: &Graph<usize, HashSet<usize>>,
    partition: &[HashSet<usize>],
) -> DegreeInfo {
    let mut in_degrees: HashMap<usize, f64> = HashMap::new();
    let mut out_degrees: HashMap<usize, f64> = HashMap::new();
    let mut stot_in: Vec<f64> = vec![];
    let mut stot_out: Vec<f64> = vec![];
    let mut degrees: HashMap<usize, f64> = HashMap::new();
    let mut stot: Vec<f64> = vec![];

    if graph.specs.directed {
        // the `get_weighted_*` methods can be used here, whether or not the original graph
        // was weighted because `set_all_edge_weights` has been called in `louvain_partitions`
        in_degrees = graph.get_weighted_in_degree_for_all_nodes().unwrap();
        out_degrees = graph.get_weighted_out_degree_for_all_nodes().unwrap();
        stot_in = (0..partition.len())
            .into_iter()
            .map(|i| *in_degrees.get(&i).unwrap())
            .collect();
        stot_out = (0..partition.len())
            .into_iter()
            .map(|i| *out_degrees.get(&i).unwrap())
            .collect();
    } else {
        degrees = graph.get_weighted_degree_for_all_nodes();
        stot = (0..partition.len())
            .into_iter()
            .map(|i| *degrees.get(&i).unwrap())
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
    }
}

/// Converts a graph of node named by `T` to a graph where nodes are named
/// with `usize` values instead. It uses the `node_map` to perform the T to
/// usize replacements.
fn convert_graph<T, A>(
    graph: &Graph<T, A>,
    weighted: bool,
    node_map: &HashMap<T, usize>,
) -> Graph<usize, HashSet<usize>>
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
            let u = *node_map.get(&n.name.clone()).unwrap();
            Node::from_name_and_attributes(u, vec![u].into_iter().collect::<HashSet<usize>>())
        })
        .collect();
    let edges = graph
        .get_all_edges()
        .iter()
        .map(|e| Edge {
            u: *node_map.get(&e.u.clone()).unwrap(),
            v: *node_map.get(&e.v.clone()).unwrap(),
            weight: e.weight,
            attributes: None,
        })
        .collect();
    Graph::new_from_nodes_and_edges(nodes, edges, graph.specs.clone()).unwrap()
}

/// Generates a new graph based on the partitions of a given graph.
fn generate_graph<T>(
    graph: &Graph<T, HashSet<T>>,
    partition: Vec<HashSet<T>>,
) -> Graph<usize, HashSet<T>>
where
    T: Hash + Eq + Clone + Ord + Display + Send + Sync,
{
    let mut new_graph = Graph::new(GraphSpecs {
        self_loops: true,
        edge_dedupe_strategy: EdgeDedupeStrategy::KeepLast,
        ..graph.specs.clone()
    });
    let mut node2com = HashMap::<T, usize>::new();
    partition.iter().enumerate().for_each(|(i, part)| {
        let mut nodes = HashSet::<T>::new();
        for node in part {
            *node2com.entry(node.clone()).or_insert(0) = i;
            let node_object = graph.get_node(node.clone()).unwrap();
            let node_hs = vec![node].into_iter().cloned().collect::<HashSet<T>>();
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
    u: &T,
    nbrs: &HashMap<T, HashSet<T>>,
    node2com: &HashMap<T, usize>,
) -> HashMap<usize, f64>
where
    T: Hash + Eq + Clone + Ord + Display + Send + Sync,
    A: Clone + Send + Sync,
{
    let hm: HashMap<usize, f64> = HashMap::new();
    let empty_hs = HashSet::new();
    let hs = nbrs.get(u).unwrap_or(&empty_hs);
    hs.iter().fold(hm, |mut acc: HashMap<usize, f64>, v: &T| {
        if u == v {
            return acc;
        }
        let edge = graph.get_edge(u.clone(), v.clone()).unwrap();
        *acc.entry(*node2com.get(v).unwrap()).or_insert(0.0) += edge.weight;
        // *acc.get_mut(node2com.get(v).as_ref().unwrap()).unwrap() += edge.weight;
        acc
    })
}

/// Creates the initial mapping of node names in the `graph` to
/// a vector where each item contains a HashSet that contains a single
/// node name.
fn map_node_names_to_hashsets(graph: &Graph<usize, HashSet<usize>>) -> Vec<HashSet<usize>> {
    graph
        .get_all_nodes()
        .iter()
        .map(|n| n.name)
        .sorted()
        .map(|n| {
            let mut hs = HashSet::new();
            hs.insert(n);
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
            Edge {u: "n1", v: "n2", weight: 1.0, attributes: None},
            Edge {u: "n1", v: "n2", weight: 1.1, attributes: None},
            Edge {u: "n2", v: "n1", weight: 1.2, attributes: None},
            Edge {u: "n1", v: "n3", weight: 1.3, attributes: None},
            Edge {u: "n1", v: "n4", weight: 1.4, attributes: None},
            Edge {u: "n4", v: "n3", weight: 1.5, attributes: None},
        ]).expect("couldn't add edges");
        let node_map = vec![("n1", 1), ("n2", 2), ("n3", 3), ("n4", 4)].into_iter().collect();
        let converted_graph = convert_graph(&graph, true, &node_map);
        assert_eq!(converted_graph.get_all_nodes().iter().map(|n| n.name).sorted().collect::<Vec<usize>>(), vec![1, 2, 3, 4]);
        assert_eq!(converted_graph.get_node(1).unwrap().attributes.clone().unwrap().len(), 1);
        assert_eq!(converted_graph.get_all_edges().len(), 5);
        assert_eq!(converted_graph.get_edge(1, 2).unwrap().weight, 2.1);
        assert_eq!(converted_graph.get_edge(2, 1).unwrap().weight, 1.2);
        assert_eq!(converted_graph.get_edge(1, 3).unwrap().weight, 1.3);
        assert_eq!(converted_graph.get_edge(1, 4).unwrap().weight, 1.4);
        assert_eq!(converted_graph.get_edge(4, 3).unwrap().weight, 1.5);
    }

    #[rustfmt::skip]
    #[test]
    fn test_generate_graph() {
        let mut graph = Graph::new(GraphSpecs::directed_create_missing());
        graph.add_edges(vec![
            Edge {u: "n1", v: "n2", weight: 1.1, attributes: None},
            Edge {u: "n2", v: "n1", weight: 1.2, attributes: None},
            Edge {u: "n1", v: "n3", weight: 1.3, attributes: None},
            Edge {u: "n1", v: "n4", weight: 1.4, attributes: None},
            Edge {u: "n4", v: "n3", weight: 1.5, attributes: None},
        ]).expect("couldn't add edges");
        let communities = vec![
            vec!["n1", "n2"].into_iter().collect(),
            vec!["n3", "n4"].into_iter().collect(),
        ];
        let gen_graph = generate_graph(&graph, communities);
        assert_eq!(gen_graph.get_all_nodes().iter().map(|n| n.name).sorted().collect::<Vec<usize>>(), vec![0, 1]);
        assert_eq!(gen_graph.get_node(0).unwrap().attributes.as_ref().unwrap().iter().copied().sorted().collect::<Vec<&str>>(), vec!["n1", "n2"]);
        assert_eq!(gen_graph.get_node(1).unwrap().attributes.as_ref().unwrap().iter().copied().sorted().collect::<Vec<&str>>(), vec!["n3", "n4"]);
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
            Edge {u: "n1", v: "n2", weight: 1.1, attributes: None},
            Edge {u: "n2", v: "n1", weight: 1.2, attributes: None},
            Edge {u: "n1", v: "n3", weight: 1.3, attributes: None},
            Edge {u: "n1", v: "n4", weight: 1.4, attributes: None},
            Edge {u: "n4", v: "n3", weight: 1.5, attributes: None},
        ]).expect("couldn't add edges");
        let mut nbrs = HashMap::new();
        nbrs.insert("n1", vec!["n2", "n3", "n4"].into_iter().collect::<HashSet<&str>>());
        let mut node2com = HashMap::new();
        node2com.insert("n1", 0);
        node2com.insert("n2", 0);
        node2com.insert("n3", 2);
        node2com.insert("n4", 2);
        let weights = get_neighbor_weights(&graph, &"n1", &nbrs, &node2com);
        assert_eq!(weights.len(), 2);
        assert_eq!(weights.get(&0).unwrap(), &1.1);
        assert_eq!(weights.get(&2).unwrap(), &2.7);
    }

    #[rustfmt::skip]
    #[test]
    fn test_map_node_names_to_hashsets() {
        let mut graph: Graph<usize, HashSet<usize>> = Graph::new(GraphSpecs::directed_create_missing());
        graph.add_edges(vec![
            Edge {u: 1, v: 2, weight: 1.1, attributes: None},
            Edge {u: 2, v: 1, weight: 1.2, attributes: None},
            Edge {u: 1, v: 3, weight: 1.3, attributes: None},
            Edge {u: 1, v: 4, weight: 1.4, attributes: None},
            Edge {u: 4, v: 3, weight: 1.5, attributes: None},
        ]).expect("couldn't add edges");
        let names = map_node_names_to_hashsets(&graph);
        assert_eq!(names.iter().map(|hs| hs.len()).collect::<Vec<usize>>(), vec![1, 1, 1, 1]);
        assert_eq!(
            names.into_iter().flat_map(|hs| hs.into_iter().collect::<Vec<usize>>())
                 .sorted().collect::<Vec<usize>>()
            , vec![1, 2, 3, 4]
        );
    }
}
