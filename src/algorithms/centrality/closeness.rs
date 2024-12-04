#![allow(non_snake_case)]

use super::fringe_node::FringeNode;
use crate::{Error, Graph};
use nohash::{IntMap, IntSet};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::hash::Hash;

/**
Compute closeness centrality for nodes.

Closeness centrality of a node u is the reciprocal of the average shortest path
distance to u over all n-1 reachable nodes.

# Arguments

* `graph`: a [Graph](../../../struct.Graph.html) instance
* `weighted`: set to `true` to use edge weights when computing the betweenness centrality
* `wf_improved`: if `true`, scale by the fraction of nodes reachable; this gives the
* `parallel`: set to `true` to compute in parallel
Wasserman and Faust improved formula. For single component graphs it is the same as the
original formula.

# Examples

```
use graphrs::{algorithms::{centrality::{closeness}}, generators};
let graph = generators::social::karate_club_graph();
let centralities = closeness::closeness_centrality(&graph, false, true, false);
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
    parallel: bool,
) -> Result<HashMap<T, f64>, Error>
where
    T: Hash + Eq + Clone + Ord + Debug + Display + Send + Sync,
    A: Clone + Send + Sync,
{
    /* */
    let mut the_graph = graph;
    let x: Graph<T, A>;
    if graph.specs.directed {
        x = graph.reverse().unwrap();
        the_graph = &x;
    }
    let num_nodes = the_graph.number_of_nodes();
    let mut centralities = HashMap::new();
    match parallel {
        true => {
            let results: Vec<(T, f64)> = (0..the_graph.number_of_nodes())
                .into_par_iter()
                .map(|source| {
                    let shortest_paths = match weighted {
                        true => single_source_shortest_path_length_weighted(the_graph, source),
                        false => single_source_shortest_path_length_unweighted(the_graph, source),
                    };
                    let cc = get_node_centrality(&shortest_paths, num_nodes, wf_improved);
                    let node_name = the_graph.get_node_by_index(&source).unwrap().name.clone();
                    (node_name, cc)
                })
                .collect();
            for (node, cc) in results {
                centralities.insert(node, cc);
            }
        }
        false => {
            for source in 0..the_graph.number_of_nodes() {
                let shortest_paths = match weighted {
                    true => single_source_shortest_path_length_weighted(the_graph, source),
                    false => single_source_shortest_path_length_unweighted(the_graph, source),
                };
                let cc = get_node_centrality(&shortest_paths, num_nodes, wf_improved);
                let node_name = the_graph.get_node_by_index(&source).unwrap().name.clone();
                centralities.insert(node_name, cc);
            }
        }
    }
    Ok(centralities)
}

fn single_source_shortest_path_length_unweighted<T, A>(
    graph: &Graph<T, A>,
    source: usize,
) -> Vec<(usize, f64)>
where
    T: Hash + Eq + Clone + Ord + Debug + Display + Send + Sync,
    A: Clone + Send + Sync,
{
    let mut seen = IntMap::default();
    let mut level = 0.0;
    let mut next_level = IntSet::default();
    next_level.insert(source);
    let mut results = vec![];
    let num_nodes = graph.number_of_nodes();
    while next_level.len() > 0 {
        let mut found = vec![];
        for v in next_level.clone() {
            if !seen.contains_key(&v) {
                seen.insert(v, level);
                found.push(v);
                results.push((v, level));
            }
        }
        if seen.len() == num_nodes {
            return results;
        }
        next_level.clear();
        for v in found {
            let adj = graph.get_successor_nodes_by_index(&v);
            for w in adj {
                next_level.insert(w.node_index);
            }
        }
        level += 1.0;
    }
    results
}

fn single_source_shortest_path_length_weighted<T, A>(
    graph: &Graph<T, A>,
    source: usize,
) -> Vec<(usize, f64)>
where
    T: Hash + Eq + Clone + Ord + Debug + Display + Send + Sync,
    A: Clone + Send + Sync,
{
    let mut D = vec![f64::MAX; graph.number_of_nodes()];
    let mut seen = vec![f64::MAX; graph.number_of_nodes()];
    let mut fringe = BinaryHeap::<FringeNode>::new();
    let mut sigma = vec![0.0; graph.number_of_nodes()];

    sigma[source] = 1.0;
    seen[source] = 0.0;

    fringe.push(FringeNode {
        distance: -0.0,
        pred: source,
        v: source,
    });

    while let Some(fringe_item) = fringe.pop() {
        let dist = -fringe_item.distance;
        let v = fringe_item.v;
        let pred = fringe_item.pred;
        if D[v] != f64::MAX {
            continue;
        }
        sigma[v] += sigma[pred];
        D[v] = dist;
        for adj in graph.get_successor_nodes_by_index(&v) {
            let w = adj.node_index;
            let cost = adj.weight;
            let vw_dist = dist + cost;
            if D[w] == f64::MAX && (seen[w] == f64::MAX || vw_dist < seen[w]) {
                seen[w] = vw_dist;
                push_fringe_node(&mut fringe, v, w, vw_dist);
                sigma[w] = 0.0;
            } else if vw_dist == seen[w] {
                sigma[w] += sigma[v];
            }
        }
    }

    D.into_iter()
        .enumerate()
        .filter(|(_, d)| *d != f64::MAX)
        .collect()
}

/**
Pushes a `FringeNode` into the `fringe` `BinaryHeap`.
Increments `count`.
*/
#[inline]
fn push_fringe_node(fringe: &mut BinaryHeap<FringeNode>, v: usize, w: usize, vw_dist: f64) {
    fringe.push(FringeNode {
        distance: -vw_dist, // negative because BinaryHeap is a max heap
        pred: v,
        v: w,
    });
}

#[inline]
fn get_node_centrality(
    shortest_paths: &Vec<(usize, f64)>,
    num_nodes: usize,
    wf_improved: bool,
) -> f64 {
    let totsp = shortest_paths.iter().map(|(_, sp)| sp).sum::<f64>();
    let mut cc = 0.0;
    if totsp > 0.0 && num_nodes > 1 {
        let s = (shortest_paths.len() - 1) as f64;
        cc = s / totsp;
        if wf_improved {
            let s = s / (num_nodes - 1) as f64;
            cc *= s;
        }
    }
    return cc;
}
