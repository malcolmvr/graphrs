use crate::{Error, ErrorKind, Graph};
use rayon::prelude::*;
use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;

/**
Compute PageRank centrality for nodes.

PageRank models a random walk over the graph. At each step, the walker follows
an outgoing edge with probability `alpha`, or teleports to a uniformly chosen
node with probability `1 - alpha`. Nodes without outgoing edges distribute
their rank uniformly across all nodes.

When `weighted` is `true`, outgoing edge weights are used as transition
weights. Unweighted edges contribute a weight of `1.0`. Multiple edges
contribute independently to the transition weight.

# Arguments

* `graph`: a [Graph](../../../struct.Graph.html) instance
* `weighted`: set to `true` to use edge weights
* `alpha`: damping factor; use `None` for the default value of `0.85`
* `max_iter`: maximum number of power iterations; use `None` for `100`
* `tolerance`: convergence tolerance; use `None` for `1.0e-6`

The computation uses Rayon automatically for graphs with more than 20 nodes
when more than one Rayon thread is available.
*/
pub fn pagerank<T, A>(
    graph: &Graph<T, A>,
    weighted: bool,
    alpha: Option<f64>,
    max_iter: Option<u32>,
    tolerance: Option<f64>,
) -> Result<HashMap<T, f64>, Error>
where
    T: Hash + Eq + Clone + Ord + Display + Send + Sync,
    A: Clone + Send + Sync,
{
    let alpha = alpha.unwrap_or(0.85);
    let max_iter = max_iter.unwrap_or(100);
    let tolerance = tolerance.unwrap_or(1.0e-6);
    if !(0.0 < alpha && alpha < 1.0) || max_iter == 0 || !(tolerance > 0.0 && tolerance.is_finite())
    {
        return Err(Error {
			kind: ErrorKind::InvalidArgument,
			message: "alpha must be between 0 and 1, max_iter must be greater than 0, and tolerance must be greater than 0.".to_string(),
		});
    }

    let nodes = graph.get_all_nodes();
    let node_count = nodes.len();
    if node_count == 0 {
        return Ok(HashMap::new());
    }

    let mut outgoing_sums = Vec::with_capacity(node_count);
    for source in 0..node_count {
        let edges = graph.get_successor_nodes_by_index(&source);
        if weighted
            && edges.iter().any(|edge| {
                !edge.weight.is_nan() && (!edge.weight.is_finite() || edge.weight < 0.0)
            })
        {
            return Err(Error {
                kind: ErrorKind::InvalidArgument,
                message: "PageRank weights must be finite and non-negative.".to_string(),
            });
        }
        outgoing_sums.push(
            edges
                .iter()
                .map(|edge| edge_weight(edge.weight, weighted))
                .sum::<f64>(),
        );
    }

    let teleport = (1.0 - alpha) / node_count as f64;
    let mut ranks = vec![1.0 / node_count as f64; node_count];
    let parallel = node_count > 20 && rayon::current_num_threads() > 1;

    for _ in 0..max_iter {
        let dangling_rank: f64 = ranks
            .iter()
            .enumerate()
            .filter(|(index, _)| outgoing_sums[*index] == 0.0)
            .map(|(_, rank)| rank)
            .sum();
        let dangling_share = alpha * dangling_rank / node_count as f64;

        let mut next = vec![0.0; node_count];
        let calculate = |(destination, score): (usize, &mut f64)| {
            let incoming = graph
                .get_predecessor_nodes_by_index(&destination)
                .iter()
                .map(|edge| {
                    let weight = edge_weight(edge.weight, weighted);
                    if outgoing_sums[edge.node_index] == 0.0 {
                        0.0
                    } else {
                        ranks[edge.node_index] * weight / outgoing_sums[edge.node_index]
                    }
                })
                .sum::<f64>();
            *score = teleport + dangling_share + alpha * incoming;
        };
        if parallel {
            next.par_iter_mut().enumerate().for_each(calculate);
        } else {
            next.iter_mut().enumerate().for_each(calculate);
        }

        let difference: f64 = next
            .iter()
            .zip(ranks.iter())
            .map(|(new, old)| (new - old).abs())
            .sum();
        ranks = next;
        if difference < node_count as f64 * tolerance {
            return Ok(nodes
                .iter()
                .enumerate()
                .map(|(index, node)| (node.name.clone(), ranks[index]))
                .collect());
        }
    }

    Err(Error {
		kind: ErrorKind::PowerIterationFailedConvergence,
		message: "failed to converge to the specified tolerance within the specified number of iterations.".to_string(),
	})
}

#[inline]
fn edge_weight(weight: f64, weighted: bool) -> f64 {
    if weighted && !weight.is_nan() {
        weight
    } else {
        1.0
    }
}
