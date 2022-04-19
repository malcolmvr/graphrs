mod directed;
mod directed_weighted;
mod square;
mod undirected;
mod undirected_weighted;
mod utility;
use crate::{Error, Graph};
use directed::get_directed_triangles_and_degrees;
use directed_weighted::get_directed_weighted_triangles_and_degrees;
use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;
use undirected::get_triangles_and_degrees;
use undirected_weighted::get_weighted_triangles_and_degrees;

/**
Returns the average clustering coefficient for nodes in a graph.

# Arguments

* `graph`: a [Graph](../../struct.Graph.html) instance
* `node_names`: node names that optionally define a subset of the graph to work with

# Examples

```
use graphrs::{algorithms::cluster, generators};
let graph = generators::social::karate_club_graph();
let result = cluster::average_clustering(&graph, false, None, false).unwrap();
```
*/
pub fn average_clustering<T, A>(
    graph: &Graph<T, A>,
    weighted: bool,
    node_names: Option<&[T]>,
    count_zeros: bool,
) -> Result<f64, Error>
where
    T: Hash + Eq + Clone + Ord + Display + Send + Sync,
    A: Clone + Send + Sync,
{
    let c = clustering(graph, weighted, node_names)?;
    let vs = c.values().into_iter().filter(|v| count_zeros || v.abs() > 0.0).collect::<Vec<&f64>>();
    Ok(vs.iter().cloned().sum::<f64>() / vs.len() as f64)
}

/**
Returns the clustering coefficient for nodes in a graph.

# Arguments

* `graph`: a [Graph](../../struct.Graph.html) instance
* `node_names`: node names that optionally define a subset of the graph to work with

# Examples

```
use graphrs::{algorithms::cluster, generators};
let graph = generators::social::karate_club_graph();
let result = cluster::clustering(&graph, false, None);
```
*/
pub fn clustering<T, A>(
    graph: &Graph<T, A>,
    weighted: bool,
    node_names: Option<&[T]>,
) -> Result<HashMap<T, f64>, Error>
where
    T: Hash + Eq + Clone + Ord + Display + Send + Sync,
    A: Clone + Send + Sync,
{
    graph.ensure_not_multi_edges()?;
    match graph.specs.directed {
        true => match weighted {
            true => {
                graph.ensure_weighted()?;
                Ok(get_clustering_directed_weighted(graph, node_names))
            }
            false => Ok(get_clustering_directed(graph, node_names)),
        },
        false => match weighted {
            true => {
                graph.ensure_weighted()?;
                Ok(get_clustering_undirected_weighted(graph, node_names))
            }
            false => Ok(get_clustering_undirected(graph, node_names)),
        },
    }
}

/**
Returns the generalized degree for nodes.
For each node, the generalized degree shows how many edges of given
triangle multiplicity the node is connected to. The triangle multiplicity
of an edge is the number of triangles an edge participates in.

# Arguments

* `graph`: a [Graph](../../struct.Graph.html) instance
* `node_names`: node names that optionally define a subset of the graph to work with

# Examples

```
use graphrs::{algorithms::cluster, generators};
let graph = generators::social::karate_club_graph();
let result = cluster::generalized_degree(&graph, None);
```
*/
pub fn generalized_degree<T, A>(
    graph: &Graph<T, A>,
    node_names: Option<&[T]>,
) -> Result<HashMap<T, HashMap<usize, usize>>, Error>
where
    T: Hash + Eq + Clone + Ord + Display + Send + Sync,
    A: Clone + Send + Sync,
{
    graph.ensure_undirected()?;
    let tads = get_triangles_and_degrees(graph, node_names);
    Ok(tads.into_iter().map(|item| (item.node_name, item.generalized_degree)).collect())
}

pub use square::square_clustering;

/**
Returns a graph's transitivity, the fraction of all possible triangles present.

# Arguments

* `graph`: a [Graph](../../struct.Graph.html) instance

# Examples

```
use graphrs::{algorithms::cluster, generators};
let graph = generators::social::karate_club_graph();
let result = cluster::transitivity(&graph).unwrap();
assert_eq!(result, 0.2556818181818182);
```
*/
pub fn transitivity<T, A>(graph: &Graph<T, A>) -> Result<f64, Error>
where
    T: Hash + Eq + Clone + Ord + Display + Send + Sync,
    A: Clone + Send + Sync,
{
    graph.ensure_undirected()?;
    if graph.get_all_nodes().is_empty() {
        return Ok(0.0);
    }
    let tads = get_triangles_and_degrees(graph, None);
    let triangles = tads.iter().map(|item| (item.number_of_triangles)).sum::<usize>() as f64;
    let contri = tads.iter().map(|item| item.degree * (item.degree - 1)).sum::<usize>() as f64;
    match triangles == 0.0 {
        true => Ok(0.0),
        false => Ok(triangles / contri),
    }
}

/**
Finds the number of triangles in a graph, optionally just the ones
that include a node as one vertex.

# Arguments

* `graph`: a [Graph](../../struct.Graph.html) instance
* `node_names`: node names that optionally define a subset of the graph to work with

# Examples

```
use graphrs::{algorithms::cluster, generators};
let graph = generators::social::karate_club_graph();
let result = cluster::triangles(&graph, None);
```
*/
pub fn triangles<T, A>(
    graph: &Graph<T, A>,
    node_names: Option<&[T]>,
) -> Result<HashMap<T, usize>, Error>
where
    T: Hash + Eq + Clone + Ord + Display + Send + Sync,
    A: Clone + Send + Sync,
{
    graph.ensure_undirected()?;
    let tads = get_triangles_and_degrees(graph, node_names);
    Ok(tads.into_iter().map(|item| (item.node_name, item.number_of_triangles / 2)).collect())
}

///////////////////////
/// PRIVATE METHODS ///
///////////////////////

fn get_clustering_directed<T, A>(graph: &Graph<T, A>, node_names: Option<&[T]>) -> HashMap<T, f64>
where
    T: Hash + Eq + Clone + Ord + Display + Send + Sync,
    A: Clone + Send + Sync,
{
    let clusterc = get_directed_triangles_and_degrees(graph, node_names);
    clusterc
        .into_iter()
        .map(|o| {
            (
                o.node_name,
                match o.directed_triangles == 0 {
                    true => 0.0,
                    false => {
                        o.directed_triangles as f64
                            / ((o.total_degree as f64 * (o.total_degree as f64 - 1.0)
                                - (2.0 * o.reciprocal_degree as f64))
                                * 2.0)
                    }
                },
            )
        })
        .collect::<HashMap<T, f64>>()
}

fn get_clustering_directed_weighted<T, A>(
    graph: &Graph<T, A>,
    node_names: Option<&[T]>,
) -> HashMap<T, f64>
where
    T: Hash + Eq + Clone + Ord + Display + Send + Sync,
    A: Clone + Send + Sync,
{
    let clusterc = get_directed_weighted_triangles_and_degrees(graph, node_names);
    clusterc
        .into_iter()
        .map(|o| {
            (
                o.node_name,
                match o.directed_triangles == 0.0 {
                    true => 0.0,
                    false => {
                        o.directed_triangles
                            / ((o.total_degree as f64 * (o.total_degree as f64 - 1.0)
                                - (2.0 * o.reciprocal_degree as f64))
                                * 2.0)
                    }
                },
            )
        })
        .collect::<HashMap<T, f64>>()
}

fn get_clustering_undirected<T, A>(graph: &Graph<T, A>, node_names: Option<&[T]>) -> HashMap<T, f64>
where
    T: Hash + Eq + Clone + Ord + Display + Send + Sync,
    A: Clone + Send + Sync,
{
    let clusterc = get_triangles_and_degrees(graph, node_names);
    clusterc
        .into_iter()
        .map(|o| {
            (
                o.node_name,
                match o.number_of_triangles == 0 {
                    true => 0.0,
                    false => {
                        o.number_of_triangles as f64 / (o.degree as f64 * (o.degree as f64 - 1.0))
                    }
                },
            )
        })
        .collect::<HashMap<T, f64>>()
}

fn get_clustering_undirected_weighted<T, A>(
    graph: &Graph<T, A>,
    node_names: Option<&[T]>,
) -> HashMap<T, f64>
where
    T: Hash + Eq + Clone + Ord + Display + Send + Sync,
    A: Clone + Send + Sync,
{
    let clusterc = get_weighted_triangles_and_degrees(graph, node_names);
    clusterc
        .into_iter()
        .map(|o| {
            (
                o.node_name,
                match o.weighted_triangles == 0.0 {
                    true => 0.0,
                    false => {
                        o.weighted_triangles as f64 / (o.degree as f64 * (o.degree as f64 - 1.0))
                    }
                },
            )
        })
        .collect::<HashMap<T, f64>>()
}
