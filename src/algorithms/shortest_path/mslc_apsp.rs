use crate::algorithms::shortest_path::ShortestPathInfo;
use crate::Graph;
use nohash::IntMap;
use orx_priority_queue::*;
use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::hash::Hash;

#[rustfmt::skip]
pub fn mlsc_apsp<T, A>(
    graph: &Graph<T, A>,
    weighted: bool,
) -> HashMap<T, HashMap<T, ShortestPathInfo<T>>>
where
    T: Hash + Eq + Clone + Ord + Debug + Display + Send + Sync,
    A: Clone + Send + Sync,
{
    let mut distances = IntMap::<usize, IntMap<usize, f64>>::default();
    let mut priority_queue = DaryHeapWithMap::<usize, f64, 4>::new();

    for node in 0..graph.number_of_nodes() {
        // println!("inserting node {}", node);
        distances.entry(node).or_default().insert(node, 0.0);
        priority_queue.push(node, 0.0);
    }

    while !priority_queue.is_empty() {
        let (v, _) = priority_queue.pop().unwrap();
        for w in graph.get_successor_nodes_by_index(&v) {
            // println!("v={}, w={}", v, w);
            let mut updated: bool = false;
            for node in 0..graph.number_of_nodes() {
                let v_w_edge_weight = get_min_edge_weight(&graph, v, *w, weighted);
                let node_w_distance = get_distance(&distances, node, *w);
                let node_v_distance = get_distance(&distances, node, v);
                // println!("  s={} v={} w={} d(s,w)={} d(s,v)={} l(v,w)={:?}", node, v, w, inf(node_w_distance), inf(&node_v_distance), v_w_edge_weight);
                if node_w_distance > node_v_distance + v_w_edge_weight {
                    // println!("    set d({},{}) to {:?}", node, *w, node_v_distance + v_w_edge_weight);
                    distances.entry(node).or_default().insert(*w, node_v_distance + v_w_edge_weight);
                    // print_distances(&distances);
                    updated = true;
                }
            }
            if updated {
                let distance = get_predecessor_edge_with_min_weight(&graph, *w);
                // print_queue(&priority_queue);
                if priority_queue.contains(w) {
                    priority_queue.remove(w);
                    // print_queue(&priority_queue);
                }
                priority_queue.push(*w, distance);
            }
            // print_queue(&priority_queue);
        }
    }

    let results = distances
        .into_iter()
        .fold(HashMap::new(), |mut acc, (edge_index, distance)| {
            for (target_index, distance) in distance {
                let source_node = graph.get_node_by_index(&edge_index).unwrap().name.clone();
                let target_node = graph.get_node_by_index(&target_index).unwrap().name.clone();
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

fn get_distance(distances: &IntMap<usize, IntMap<usize, f64>>, u: usize, v: usize) -> f64 {
    if !distances.contains_key(&u) {
        return f64::MAX;
    }
    *distances.get(&u).unwrap().get(&v).unwrap_or(&f64::MAX)
}
/*
fn inf(f: &f64) -> String {
    match *f == f64::MAX {
        true => "inf".to_string(),
        false => format!("{:?}", f),
    }
}

fn print_distances(distances: &HashMap<EdgeIndex, f64, BuildNoHashHasher<usize>>) {
    let joined_distances = distances
        .iter()
        .map(|(edge_index, distance)| {
            format!("({}, {}): {:?}", edge_index.u, edge_index.v, distance)
        })
        .collect::<Vec<_>>()
        .join(", ");
    println!("        {{{}}}", joined_distances);
}

fn print_queue(queue: &DaryHeapWithMap<usize, f64, 4>) {
    let joined_queue = queue
        .iter()
        .map(|(node, distance)| format!("({:?}, {})", distance, node))
        .collect::<Vec<_>>()
        .join(", ");
    println!("[{}]", joined_queue);
}
*/

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
        .map(|node| get_min_edge_weight(&graph, *node, node_index, true))
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap()
}
