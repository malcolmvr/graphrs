use crate::{Error, ErrorKind, Graph};
use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;

/**
Compute eigenvector centrality for nodes.

Eigenvector centrality computes the centrality for a node based on the
centrality of its neighbors. Eigenvector centrality is commonly described
as a measure of the influence of a node in the graph.

# Arguments

* `graph`: a [Graph](../../../struct.Graph.html) instance
* `weighted`: set to `true` to use edge weights when computing the betweenness centrality
* `max_iter`: the maximum number of iterations in power method; use `None` to use the default
value of `100`
* `tolerance`: the error tolerance used to check convergence in power method iteration; use `None` to
use the default value of `1.0e-6`

# Examples

```
use graphrs::{algorithms::{centrality::{eigenvector}}, generators};
let graph = generators::social::karate_club_graph();
let centralities = eigenvector::eigenvector_centrality(&graph, false, None, None);
```

# References

1. Phillip Bonacich. "Power and Centrality: A Family of Measures."
*American Journal of Sociology* 92(5):1170–1182, 1986
<http://www.leonidzhukov.net/hse/2014/socialnetworks/papers/Bonacich-Centrality.pdf>

2. Mark E. J. Newman.
*Networks: An Introduction.*
Oxford University Press, USA, 2010, pp. 169.
*/
pub fn eigenvector_centrality<T, A>(
    graph: &Graph<T, A>,
    weighted: bool,
    max_iter: Option<u32>,
    tolerance: Option<f64>,
) -> Result<HashMap<T, f64>, Error>
where
    T: Hash + Eq + Clone + Ord + Display + Send + Sync,
    A: Clone + Send + Sync,
{
    let _max_iter = max_iter.unwrap_or(100);
    let _tolerance = tolerance.unwrap_or(1.0e-6);
    let nnodes = graph.get_all_nodes().len();
    let mut x: HashMap<T, f64> = graph
        .get_all_nodes()
        .iter()
        .map(|n| (n.name.clone(), 1.0 / nnodes as f64))
        .collect();
    for _i in 0.._max_iter {
        let xlast = x.clone();
        for n in xlast.keys() {
            for nbr in graph.get_successors_or_neighbors(n.clone()) {
                let edge = graph.get_edge(n.clone(), nbr.name.clone()).unwrap();
                let w = match !weighted || edge.weight.is_nan() {
                    true => 1.0,
                    false => edge.weight,
                };
                *x.get_mut(&nbr.name).unwrap() += xlast.get(n).unwrap() * w;
            }
        }
        let mut norm: f64 = x.values().map(|v| v.powf(2.0)).sum();
        norm = norm.sqrt();
        norm = match norm == 0.0 {
            true => 1.0,
            false => norm,
        };
        x.values_mut().for_each(|v| *v /= norm);
        let y: f64 = x
            .iter()
            .map(|(k, v)| (v - xlast.get(k).unwrap()).abs())
            .sum();
        if y < (nnodes as f64 * _tolerance) {
            return Ok(x);
        }
    }
    Err(Error {
        kind: ErrorKind::PowerIterationFailedConvergence,
        message: "failed to converge to the specified tolerance within the specified number of iterations.".to_string(),
    })
}
