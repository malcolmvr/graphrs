#![allow(non_snake_case)]

use super::fringe_node::{push_fringe_node, FringeNode};
use crate::{Error, Graph};
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
        },
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
    let hm = betweenness.clone()
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
