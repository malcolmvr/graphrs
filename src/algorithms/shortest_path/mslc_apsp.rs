use crate::algorithms::components::bfs_equal_size_partitions;
use crate::algorithms::shortest_path::ShortestPathInfo;
use crate::Graph;
use orx_priority_queue::{DaryHeapWithMap, PriorityQueue, PriorityQueueDecKey};
use rayon::iter::IntoParallelIterator;
use rayon::prelude::ParallelIterator;
use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::num::NonZeroUsize;
use std::thread::available_parallelism;

pub fn mlsc_apsp<T, A>(
    graph: &Graph<T, A>,
    sources: Option<&Vec<T>>,
    weighted: bool,
) -> HashMap<T, HashMap<T, ShortestPathInfo<T>>>
where
    T: Hash + Eq + Clone + Ord + Debug + Display + Send + Sync,
    A: Clone + Send + Sync,
{
    let partition_graph = match sources {
        Some(sources) => &graph.get_subgraph(sources),
        None => graph,
    };
    let num_partitions = get_number_of_partitions(&graph);
    match num_partitions == 1 {
        true => {
            return mlsc_apsp_sub(
                &graph,
                &(0..graph.number_of_nodes()).collect::<Vec<usize>>(),
                weighted,
            )
        }
        false => {
            let partitions = get_bfs_partitions_usize(partition_graph, num_partitions);
            partitions
                .into_par_iter()
                .map(|partition| mlsc_apsp_sub(&graph, &partition, weighted))
                .reduce(HashMap::new, |mut acc, subgraph_results| {
                    for (source, target_results) in subgraph_results {
                        for (target, target_result) in target_results {
                            acc.entry(source.clone())
                                .or_insert_with(HashMap::new)
                                .entry(target.clone())
                                .or_insert(target_result);
                        }
                    }
                    acc
                })
        }
    }
}

#[rustfmt::skip]
pub fn mlsc_apsp_sub<T, A>(
    graph: &Graph<T, A>,
    sources: &Vec<usize>,
    weighted: bool,
) -> HashMap<T, HashMap<T, ShortestPathInfo<T>>>
where
    T: Hash + Eq + Clone + Ord + Debug + Display + Send + Sync,
    A: Clone + Send + Sync,
{
    let number_of_nodes: usize = graph.number_of_nodes();
    let number_of_sources = sources.len();
    let capacity = graph.number_of_edges() + (number_of_sources * number_of_nodes);
    let mut distances = vec![f64::MAX; capacity];
    let mut priority_queue: DaryHeapWithMap<usize, f64, 4> =
        DaryHeapWithMap::<usize, f64, 4>::new();

    for (source_index, node) in sources.iter().enumerate() {
        // println!("inserting node {}", node);
        set_distance(&mut distances, source_index, *node, number_of_sources, 0.0);
        // distances.entry(node).or_default().insert(node, 0.0);
        priority_queue.push(*node, 0.0);
    }

    while !priority_queue.is_empty() {
        let (v, _) = priority_queue.pop().unwrap();
        for successor in graph.get_successor_nodes_by_index(&v) {
            let w = successor.node_index;
            // println!("v={}, w={}", v, w);
            let mut updated: bool = false;
            for source_index in 0..sources.len() {
                let v_w_edge_weight = successor.weight;
                let node_w_distance = get_distance(&distances, source_index, w, number_of_sources);
                let node_v_distance = get_distance(&distances, source_index, v, number_of_sources);
                // println!("  s={} v={} w={} d(s,w)={} d(s,v)={} l(v,w)={:?}", source_index, v, w, inf(&node_w_distance), inf(&node_v_distance), v_w_edge_weight);
                if node_w_distance > node_v_distance + v_w_edge_weight {
                    set_distance(
                        &mut distances,
                        source_index,
                        w,
                        number_of_sources,
                        node_v_distance + v_w_edge_weight,
                    );
                    // println!("    set d({},{}) to {:?}", source_index, *w, node_v_distance + v_w_edge_weight);
                    // print_distances(&distances);
                    updated = true;
                }
            }
            if updated {
                let distance = get_predecessor_edge_with_min_weight(&graph, w);
                // print_queue(&priority_queue);
                if priority_queue.contains(&w) {
                    priority_queue.remove(&w);
                    // print_queue(&priority_queue);
                }
                priority_queue.push(w, distance);
            }
            // print_queue(&priority_queue);
        }
    }

    let results = (0..number_of_sources)
        .flat_map(|source_index| (0..number_of_nodes).map(move |v| (source_index, v)))
        .fold(HashMap::new(), |mut acc, (source_index, v)| {
            let u = sources[source_index];
            let distance = get_distance(&distances, source_index, v, number_of_sources);
            if distance < f64::MAX {
                let source_node = graph.get_node_by_index(&u).unwrap().name.clone();
                let target_node = graph.get_node_by_index(&v).unwrap().name.clone();
                let path = vec![source_node.clone(), target_node.clone()];
                acc.entry(source_node.clone())
                    .or_insert_with(HashMap::new)
                    .insert(
                        target_node.clone(),
                        ShortestPathInfo {
                            distance,
                            paths: vec![path],
                        },
                    );
            }
            acc
        });

    results
}

fn get_number_of_partitions<T, A>(graph: &Graph<T, A>) -> usize
where
    T: Hash + Eq + Clone + Ord + Debug + Display + Send + Sync,
    A: Clone + Send + Sync,
{
    match graph.number_of_nodes() < 100 {
        true => 1,
        false => {
            available_parallelism()
                .unwrap_or(NonZeroUsize::new(1).unwrap())
                .get()
                * 12
        }
    }
}

fn get_distance(distances: &Vec<f64>, u: usize, v: usize, number_of_sources: usize) -> f64 {
    return distances[get_distance_index(u, v, number_of_sources)];
}

fn set_distance(
    distances: &mut Vec<f64>,
    u: usize,
    v: usize,
    number_of_sources: usize,
    distance: f64,
) {
    distances[get_distance_index(u, v, number_of_sources)] = distance;
}

fn get_distance_index(u: usize, v: usize, number_of_sources: usize) -> usize {
    return (v * number_of_sources) + u;
}

fn get_min_edge_weight<T, A>(graph: &Graph<T, A>, u: usize, v: usize, weighted: bool) -> f64
where
    T: Hash + Eq + Clone + Ord + Debug + Display + Send + Sync,
    A: Clone + Send + Sync,
{
    match weighted {
        true => match graph.specs.multi_edges {
            true => graph
                .get_edges_by_indexes(u, v)
                .unwrap()
                .into_iter()
                .map(|edge| edge.weight)
                .min_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap(),
            false => graph.get_edge_by_indexes(u, v).unwrap().weight,
        },
        false => 1.0,
    }
}

fn get_predecessor_edge_with_min_weight<T, A>(graph: &Graph<T, A>, node_index: usize) -> f64
where
    T: Hash + Eq + Clone + Ord + Debug + Display + Send + Sync,
    A: Clone + Send + Sync,
{
    graph
        .get_predecessor_nodes_by_index(&node_index)
        .into_iter()
        .map(|node| node.weight)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap()
}

fn get_bfs_partitions_usize<T, A>(graph: &Graph<T, A>, num_partitions: usize) -> Vec<Vec<usize>>
where
    T: Hash + Eq + Clone + Ord + Debug + Display + Send + Sync,
    A: Clone + Send + Sync,
{
    let partitions = bfs_equal_size_partitions(graph, num_partitions);
    partitions
        .iter()
        .map(|partition| {
            partition
                .iter()
                .map(|node_name| graph.get_node_index(node_name).unwrap())
                .collect::<Vec<usize>>()
        })
        .collect()
}
