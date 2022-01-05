use crate::{Error, ErrorKind, Graph, GraphSpecs};
extern crate rand;

/**
Returns an Erdos-Renyi or binomial random graph.

# Arguments

* `num_nodes`: The number of nodes to generate.
* `edge_probability`: The probability for edge creation.
* `directed`: If `true` generates a directed graph, otherwise it generates an undirected graph.

# Examples

```
use graphrs::{generators};
let graph = generators::random::fast_gnp_random_graph(250, 0.25, true);
assert!(graph.is_ok());
```
*/
pub fn fast_gnp_random_graph(
    num_nodes: i32,
    edge_probability: f64,
    directed: bool,
) -> Result<Graph<i32, ()>, Error> {
    if edge_probability <= 0.0 || edge_probability >= 1.0 {
        return Err(Error {
            kind: ErrorKind::InvalidArgument,
            message: format!(
                "`edge_probability was {} but it must be between 0.0 and 1.0, non-inclusive.",
                edge_probability
            ),
        });
    }
    match directed {
        true => fast_gnp_random_graph_directed(num_nodes, edge_probability),
        false => fast_gnp_random_graph_undirected(num_nodes, edge_probability),
    }
}

fn fast_gnp_random_graph_directed(
    num_nodes: i32,
    edge_probability: f64,
) -> Result<Graph<i32, ()>, Error> {
    let mut graph = Graph::new(GraphSpecs::directed_create_missing());
    let mut w: i32 = -1;
    let lp = (1.0 - edge_probability).ln();
    let mut v = 0;
    let mut edges = vec![];
    while v < num_nodes {
        let lr: f64 = (1.0_f64 - rand::random::<f64>()).ln();
        w = w + 1 + ((lr / lp) as i32);
        if v == w {
            w = w + 1;
        }
        while v < num_nodes && num_nodes <= w {
            w = w - num_nodes;
            v = v + 1;
            if v == w {
                w = w + 1;
            }
        }
        if v < num_nodes {
            edges.push((v, w));
        }
    }
    match graph.add_edge_tuples(edges) {
        Err(e) => Err(e),
        Ok(_) => Ok(graph),
    }
}

fn fast_gnp_random_graph_undirected(
    num_nodes: i32,
    edge_probability: f64,
) -> Result<Graph<i32, ()>, Error> {
    let mut graph = Graph::new(GraphSpecs::undirected_create_missing());
    let mut w: i32 = -1;
    let lp = (1.0 - edge_probability).ln();
    let mut v = 1;
    let mut edges = vec![];
    while v < num_nodes {
        let lr: f64 = (1.0_f64 - rand::random::<f64>()).ln();
        w = w + 1 + ((lr / lp) as i32);
        while w >= v && v < num_nodes {
            w = w - v;
            v = v + 1;
        }
        if v < num_nodes {
            edges.push((v, w));
        }
    }
    match graph.add_edge_tuples(edges) {
        Err(e) => Err(e),
        Ok(_) => Ok(graph),
    }
}
