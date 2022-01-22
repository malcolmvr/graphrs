use crate::{Error, ErrorKind, Graph, GraphSpecs, Node};
use rand::{Rng, RngCore, SeedableRng};
use rand_chacha::ChaCha20Rng;

/**
Returns an Erdos-Renyi or binomial random graph.

# Arguments

* `num_nodes`: The number of nodes to generate.
* `edge_probability`: The probability for edge creation.
* `directed`: If `true` generates a directed graph, otherwise it generates an undirected graph.

# Examples

```
use graphrs::{generators};
let graph = generators::random::fast_gnp_random_graph(250, 0.25, true, None);
assert!(graph.is_ok());
```
*/
pub fn fast_gnp_random_graph(
    num_nodes: i32,
    edge_probability: f64,
    directed: bool,
    seed: Option<u64>,
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
    let mut rng = get_random_number_generator(seed);
    match directed {
        true => fast_gnp_random_graph_directed(num_nodes, edge_probability, &mut rng),
        false => fast_gnp_random_graph_undirected(num_nodes, edge_probability, &mut rng),
    }
}

fn fast_gnp_random_graph_directed(
    num_nodes: i32,
    edge_probability: f64,
    rng: &mut Box<dyn RngCore>,
) -> Result<Graph<i32, ()>, Error> {
    let mut graph = Graph::new(GraphSpecs::directed_create_missing());
    for i in 0..num_nodes {
        graph.add_node(Node::from_name(i));
    }
    let mut w: i32 = -1;
    let lp = (1.0 - edge_probability).ln();
    let mut v = 0;
    let mut edges = vec![];
    while v < num_nodes {
        let lr: f64 = (1.0_f64 - rng.gen::<f64>()).ln();
        w = w + 1 + ((lr / lp) as i32);
        if v == w {
            w += 1;
        }
        while v < num_nodes && num_nodes <= w {
            w -= num_nodes;
            v += 1;
            if v == w {
                w += 1;
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
    rng: &mut Box<dyn RngCore>,
) -> Result<Graph<i32, ()>, Error> {
    let mut graph = Graph::new(GraphSpecs::undirected_create_missing());
    for i in 0..num_nodes {
        graph.add_node(Node::from_name(i));
    }
    let mut w: i32 = -1;
    let lp = (1.0 - edge_probability).ln();
    let mut v = 1;
    let mut edges = vec![];
    while v < num_nodes {
        let lr: f64 = (1.0_f64 - rng.gen::<f64>()).ln();
        w = w + 1 + ((lr / lp) as i32);
        while w >= v && v < num_nodes {
            w += v;
            v += 1;
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

fn get_random_number_generator(seed: Option<u64>) -> Box<dyn RngCore> {
    match seed {
        None => Box::new(rand::thread_rng()),
        Some(s) => Box::new(ChaCha20Rng::seed_from_u64(s)),
    }
}
