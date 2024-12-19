#![allow(non_snake_case)]

use super::fringe_node::{push_fringe_node, FringeNode};
use crate::{edge, Error, Graph};
use core::f64;
use rand::distributions;
use rayon::iter::*;
use rayon::prelude::ParallelIterator;
use std::collections::{BinaryHeap, HashMap, VecDeque};
use std::fmt::Debug;
use std::fmt::Display;
use std::hash::Hash;

struct SingleSourceResults {
    S: Vec<usize>,
    P: Vec<Vec<usize>>,
    sigma: Vec<f64>,
    source: usize,
}

/**
Compute the shortest-path (Dijkstra) betweenness centrality for nodes.

# Arguments

* `graph`: a [Graph](../../../struct.Graph.html) instance
* `weighted`: set to `true` to use edge weights when computing the betweenness centrality
* `normalized`: set to `true` to normalize the node centrality values
* `parallel`: set to `true` to compute in parallel

# Examples

```
use graphrs::{algorithms::{centrality::{betweenness}}, generators};
let graph = generators::social::karate_club_graph();
let centralities = betweenness::betweenness_centrality(&graph, false, true);
```

# References

1. Ulrik Brandes: A Faster Algorithm for Betweenness Centrality. Journal of Mathematical Sociology 25(2):163-177, 2001.
*/
pub fn betweenness_centrality<T, A>(
    graph: &Graph<T, A>,
    weighted: bool,
    normalized: bool,
) -> Result<HashMap<T, f64>, Error>
where
    T: Hash + Eq + Clone + Ord + Debug + Display + Send + Sync,
    A: Clone + Send + Sync,
{
    let parallel = graph.number_of_nodes() > 20 && rayon::current_num_threads() > 1;
    let betweenness_mutex = std::sync::Mutex::new(vec![0.0; graph.number_of_nodes()]);
    match parallel {
        true => {
            (0..graph.number_of_nodes())
                .into_par_iter()
                .map(|source| match weighted {
                    true => dijkstra(graph, source),
                    false => bfs(graph, source),
                })
                .for_each(|r| {
                    let mut betweenness = betweenness_mutex.lock().unwrap();
                    accumulate_betweenness(&mut betweenness, &r);
                });
        }
        false => {
            for source in 0..graph.number_of_nodes() {
                let source_source_results = match weighted {
                    true => dijkstra(graph, source),
                    false => bfs(graph, source),
                };
                let mut betweenness = betweenness_mutex.lock().unwrap();
                accumulate_betweenness(&mut betweenness, &source_source_results);
            }
        }
    }
    let mut betweenness = betweenness_mutex.lock().unwrap();
    rescale(
        &mut betweenness,
        graph.get_all_nodes().len(),
        normalized,
        graph.specs.directed,
    );
    let hm = betweenness
        .clone()
        .into_iter()
        .enumerate()
        .map(|(i, v)| (graph.get_node_by_index(&i).unwrap().name.clone(), v))
        .collect();
    Ok(hm)
}

fn bfs<T, A>(graph: &Graph<T, A>, source: usize) -> SingleSourceResults
where
    T: Hash + Eq + Clone + Ord + Display + Send + Sync,
    A: Clone,
{
    let mut P: Vec<Vec<usize>> = vec![vec![]; graph.number_of_nodes()];
    let mut D = vec![f64::MAX; graph.number_of_nodes()];
    let mut fringe = VecDeque::<usize>::new();
    let mut sigma = vec![0.0; graph.number_of_nodes()];

    sigma[source] = 1.0;
    D[source] = 0.0;

    let mut S = vec![];

    fringe.push_back(source);

    while let Some(v) = fringe.pop_front() {
        S.push(v);
        let Dv = D[v];
        let sigmav = sigma[v];
        for adj in graph.get_successor_nodes_by_index(&v) {
            let w = adj.node_index;
            let vw_dist = Dv + 1.0;
            if D[w] == f64::MAX {
                D[w] = vw_dist;
                fringe.push_back(w);
            }
            if D[w] == vw_dist {
                sigma[w] += sigmav;
                P[w].push(v);
            }
        }
    }

    SingleSourceResults {
        S,
        P,
        sigma,
        source,
    }
}

fn dijkstra<T, A>(graph: &Graph<T, A>, source: usize) -> SingleSourceResults
where
    T: Hash + Eq + Clone + Ord + Display + Send + Sync,
    A: Clone,
{
    // println!("source: {:?}", source);
    let mut P: Vec<Vec<usize>> = vec![vec![]; graph.number_of_nodes()];
    let mut D = vec![f64::MAX; graph.number_of_nodes()];
    let mut seen = vec![f64::MAX; graph.number_of_nodes()];
    let mut fringe = BinaryHeap::<FringeNode>::new();
    let mut sigma = vec![0.0; graph.number_of_nodes()];

    sigma[source] = 1.0;
    seen[source] = 0.0;

    let mut S = vec![];

    fringe.push(FringeNode {
        distance: -0.0,
        pred: source,
        v: source,
    });

    while let Some(fringe_item) = fringe.pop() {
        let dist = -fringe_item.distance;
        let v = fringe_item.v;
        let pred = fringe_item.pred;
        // println!("    v: {}", v);
        if D[v] != f64::MAX {
            continue;
        }
        sigma[v] += sigma[pred];
        S.push(v);
        D[v] = dist;
        for adj in graph.get_successor_nodes_by_index(&v) {
            let w = adj.node_index;
            // println!("        u: {}", u);
            let cost = adj.weight;
            // println!("            cost: {}", cost);
            let vw_dist = dist + cost;
            // println!("            vu_dist: {}", vu_dist);
            if D[w] == f64::MAX && (seen[w] == f64::MAX || vw_dist < seen[w]) {
                // println!("            vu_dist < seen[u]");
                seen[w] = vw_dist;
                push_fringe_node(&mut fringe, v, w, vw_dist);
                sigma[w] = 0.0;
                P[w] = vec![v];
            } else if vw_dist == seen[w] {
                sigma[w] += sigma[v];
                P[w].push(v);
            }
        }
    }

    SingleSourceResults {
        S,
        P,
        sigma,
        source,
    }
}

pub fn brandes_unweighted<T, A>(graph: &Graph<T, A>) -> Result<HashMap<T, f64>, Error>
where
    T: Hash + Eq + Clone + Ord + Debug + Display + Send + Sync,
    A: Clone + Send + Sync,
{
    let mut CB = vec![0.0; graph.number_of_nodes()];
    let mut sigma = vec![0.0; graph.number_of_nodes()];
    let mut delta = vec![0.0; graph.number_of_nodes()];
    let mut prev = vec![vec![]; graph.number_of_nodes()];
    let mut dist = vec![f64::NAN; graph.number_of_nodes()];
    for s in 0..graph.number_of_nodes() {
        println!("s: {}", s);
        for v in 0..graph.number_of_nodes() {
            delta[v] = 0.0;
            prev[v] = vec![];
            sigma[v] = 0.0;
            dist[v] = f64::NAN;
        }
        sigma[s] = 1.0;
        dist[s] = 0.0;
        let mut Q = VecDeque::<usize>::new();
        Q.push_back(s);
        let mut stack = Vec::<usize>::new();
        while let Some(u) = Q.pop_front() {
            println!("    u: {}", u);
            stack.push(u);
            for adj in graph.get_successor_nodes_by_index(&u) {
                let v = adj.node_index;
                if dist[v].is_nan() {
                    dist[v] = dist[u] + 1.0;
                    Q.push_back(v);
                }
                if dist[v] == dist[u] + 1.0 {
                    sigma[v] += sigma[u];
                    prev[v].push(u);
                }
            }
        }
        while let Some(v) = stack.pop() {
            for u in &prev[v] {
                delta[*u] += (sigma[*u] / sigma[v]) * (1.0 + delta[v]);
            }
            if v != s {
                CB[v] += delta[v];
            }
        }
    }
    let hm = CB
        .clone()
        .into_iter()
        .enumerate()
        .map(|(i, v)| (graph.get_node_by_index(&i).unwrap().name.clone(), v / 2.0))
        .collect();
    Ok(hm)
}

pub fn brandes_weighted<T, A>(graph: &Graph<T, A>) -> Result<HashMap<T, f64>, Error>
where
    T: Hash + Eq + Clone + Ord + Debug + Display + Send + Sync,
    A: Clone + Send + Sync,
{
    let mut CB = vec![0.0; graph.number_of_nodes()];
    let mut sigma = vec![0.0; graph.number_of_nodes()];
    let mut delta = vec![0.0; graph.number_of_nodes()];
    let mut Pred: Vec<Vec<usize>> = vec![vec![]; graph.number_of_nodes()];
    let mut dist = vec![f64::INFINITY; graph.number_of_nodes()];
    let mut S = Vec::<usize>::new();
    for s in 0..graph.number_of_nodes() {
        for w in 0..graph.number_of_nodes() {
            delta[w] = 0.0;
            Pred[w] = vec![];
            sigma[w] = 0.0;
            dist[w] = f64::INFINITY;
        }
        sigma[s] = 1.0;
        dist[s] = 0.0;
        let mut Q = BinaryHeap::<FringeNode>::new();
        Q.push(FringeNode {
            distance: -0.0,
            pred: s,
            v: s,
        });
        while let Some(fringe_item) = Q.pop() {
            let v = fringe_item.v;
            S.push(v);
            for adj in graph.get_successor_nodes_by_index(&v) {
                let w = adj.node_index;
                let edge_weight = adj.weight;
                let vw_dist = dist[v] + edge_weight;
                if dist[w] > vw_dist {
                    dist[w] = vw_dist;
                    push_fringe_node(&mut Q, v, w, vw_dist);
                    sigma[w] = 0.0;
                    Pred[w] = vec![];
                }
                if dist[w] == vw_dist {
                    sigma[w] += sigma[v];
                    Pred[w].push(v);
                }
            }
        }
        while let Some(w) = S.pop() {
            for v in &Pred[w] {
                delta[*v] += (sigma[*v] / sigma[w]) * (1.0 + delta[w]);
            }
            if w != s {
                CB[w] += delta[w];
            }
        }
    }
    // rescale(
    //     &mut CB,
    //     graph.get_all_nodes().len(),
    //     true,
    //     graph.specs.directed,
    // );
    let hm = CB
        .clone()
        .into_iter()
        .enumerate()
        .map(|(i, v)| {
            (
                graph.get_node_by_index(&i).unwrap().name.clone(),
                match graph.specs.directed {
                    true => v,
                    false => v / 2.0,
                },
            )
        })
        .collect();

    Ok(hm)
}

fn accumulate_betweenness(betweenness: &mut Vec<f64>, result: &SingleSourceResults) {
    let mut delta = vec![0.0; betweenness.len()];
    let mut S = result.S.iter().rev();
    while let Some(w) = S.next() {
        let coeff = (1.0 + delta[*w]) / result.sigma[*w];
        for v in result.P[*w].iter() {
            delta[*v] += result.sigma[*v] * coeff;
        }
        if *w != result.source {
            betweenness[*w] += delta[*w];
        }
    }
}

fn rescale(betweeneess: &mut Vec<f64>, num_nodes: usize, normalized: bool, directed: bool) {
    let scale = get_scale(num_nodes, normalized, directed);
    if scale.is_some() {
        let scale = scale.unwrap();
        for i in 0..num_nodes {
            betweeneess[i] *= scale;
        }
    }
}

#[inline]
fn get_scale(num_nodes: usize, normalized: bool, directed: bool) -> Option<f64> {
    match normalized {
        true => match num_nodes <= 2 {
            true => None,
            false => Some(1.0 / ((num_nodes as f64 - 1.0) * (num_nodes as f64 - 2.0))),
        },
        false => match directed {
            true => None,
            false => Some(0.5),
        },
    }
}

// tests for private methods only; other tests are in:
// tests/test_algorithms_centrality_betweenness
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_get_scale_1() {
        let result = get_scale(10, true, true).unwrap();
        assert_eq!(result, 1.0 / 72.0);
    }

    #[test]
    fn test_get_scale_2() {
        let result = get_scale(2, true, true);
        assert!(result.is_none());
    }

    #[test]
    fn test_get_scale_3() {
        let result = get_scale(2, false, true);
        assert!(result.is_none());
    }

    #[test]
    fn test_get_scale_4() {
        let result = get_scale(10, true, false).unwrap();
        assert_eq!(result, 1.0 / 72.0);
    }

    #[test]
    fn test_get_scale_5() {
        let result = get_scale(10, false, false).unwrap();
        assert_eq!(result, 0.5);
    }
}
