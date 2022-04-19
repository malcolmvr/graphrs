use super::utility::{get_adjacent_nodes_without, get_normalized_edge_weight};
use crate::Graph;
use std::collections::HashSet;
use std::fmt::Display;
use std::hash::Hash;

pub struct DirectedWeightedTrianglesAndDegree<T> {
    pub node_name: T,
    pub total_degree: usize,
    pub reciprocal_degree: usize,
    pub directed_triangles: f64,
}

pub fn get_directed_weighted_triangles_and_degrees<T, A>(
    graph: &Graph<T, A>,
    node_names: Option<&[T]>,
) -> Vec<DirectedWeightedTrianglesAndDegree<T>>
where
    T: Hash + Eq + Clone + Ord + Display + Send + Sync,
    A: Clone + Send + Sync,
{
    let edges = graph.get_all_edges();
    let max_weight = match edges.is_empty() {
        true => 1.0,
        false => edges.iter().map(|e| e.weight).reduce(f64::max).unwrap(),
    };
    let ns: Vec<T> = match node_names {
        None => graph.get_all_nodes().into_iter().map(|n| n.name.clone()).collect(),
        Some(names) => names.to_vec(),
    };
    ns.into_iter()
        .map(|i| {
            let ipreds = get_adjacent_nodes_without(graph, &i, true);
            let isuccs = get_adjacent_nodes_without(graph, &i, false);
            let directed_triangles_ipreds =
                get_all_directed_triangles(&i, &ipreds, &isuccs, true, graph, &max_weight);
            let directed_triangles_isuccs =
                get_all_directed_triangles(&i, &ipreds, &isuccs, false, graph, &max_weight);
            let total_degree = ipreds.len() + isuccs.len();
            let reciprocal_degree = ipreds.intersection(&isuccs).count();
            DirectedWeightedTrianglesAndDegree {
                node_name: i,
                total_degree,
                reciprocal_degree,
                directed_triangles: directed_triangles_ipreds + directed_triangles_isuccs,
            }
        })
        .collect()
}

#[inline]
#[rustfmt::skip]
fn get_all_directed_triangles<T, A>(
    i: &T,
    ipreds: &HashSet<T>,
    isuccs: &HashSet<T>,
    iter_preds: bool,
    graph: &Graph<T, A>,
    max_weight: &f64,
) -> f64
where
    T: Hash + Eq + Clone + Ord + Display + Send + Sync,
    A: Clone + Send + Sync,
{
    let to_iter = match iter_preds { true => ipreds, false => isuccs };
    let wt = |u: &T, v: &T| get_normalized_edge_weight(u, v, max_weight, graph);
    to_iter
        .iter()
        .map(|j: &T| {
            let (_i, _j) = match iter_preds { true => (j, i), false => (i, j) };
            let jpreds = get_adjacent_nodes_without(graph, j, true);
            let jsuccs = get_adjacent_nodes_without(graph, j, false);
            let directed_triangles: f64 = 
                ipreds.intersection(&jpreds).into_iter().map(|k| (wt(_i, _j) * wt(k, i) * wt(k, j)).cbrt()).sum::<f64>() +
                ipreds.intersection(&jsuccs).into_iter().map(|k| (wt(_i, _j) * wt(k, i) * wt(j, k)).cbrt()).sum::<f64>() +
                isuccs.intersection(&jpreds).into_iter().map(|k| (wt(_i, _j) * wt(i, k) * wt(k, j)).cbrt()).sum::<f64>() +
                isuccs.intersection(&jsuccs).into_iter().map(|k| (wt(_i, _j) * wt(i, k) * wt(j, k)).cbrt()).sum::<f64>();
            directed_triangles
        })
        .sum::<f64>()
}
