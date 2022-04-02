use super::Graph;
use crate::{Error, ErrorKind};
use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;

impl<T, A> Graph<T, A>
where
    T: Eq + Clone + PartialOrd + Ord + Hash + Send + Sync + Display,
    A: Clone,
{
    /**
    Compute the degree for all nodes in the graph.

    # Examples

    ```
    use graphrs::{Edge, Graph, GraphSpecs};
    let edges = vec![
        Edge::new("n1", "n2"),
        Edge::new("n1", "n3"),
        Edge::new("n1", "n4"),
        Edge::new("n4", "n5"),
    ];
    let graph: Graph<&str, ()> =
        Graph::new_from_nodes_and_edges(vec![], edges, GraphSpecs::directed_create_missing())
            .unwrap();
    let result = graph.get_degree_for_all_nodes();
    assert_eq!(result.get("n1").unwrap(), &3);
    ```
    */
    pub fn get_degree_for_all_nodes(&self) -> HashMap<T, usize> {
        self.get_all_nodes()
            .iter()
            .map(|n| {
                (
                    n.name.clone(),
                    self.get_node_degree(n.name.clone()).unwrap(),
                )
            })
            .collect()
    }

    /**
    Compute the in-degree for all nodes in the graph.

    # Examples

    ```
    use graphrs::{Edge, Graph, GraphSpecs};
    let edges = vec![
        Edge::new("n1", "n2"),
        Edge::new("n1", "n3"),
        Edge::new("n1", "n5"),
        Edge::new("n4", "n5"),
    ];
    let graph: Graph<&str, ()> =
        Graph::new_from_nodes_and_edges(vec![], edges, GraphSpecs::directed_create_missing())
            .unwrap();
    let result = graph.get_in_degree_for_all_nodes().unwrap();
    assert_eq!(result.get("n5").unwrap(), &2);
    ```
    */
    pub fn get_in_degree_for_all_nodes(&self) -> Result<HashMap<T, usize>, Error> {
        if !self.specs.directed {
            return Err(Error {
                kind: ErrorKind::WrongMethod,
                message: "Use the `get_degree_for_all_nodes` method when `directed` is `false`"
                    .to_string(),
            });
        }
        Ok(self
            .get_all_nodes()
            .iter()
            .map(|n| {
                (
                    n.name.clone(),
                    self.get_node_in_degree(n.name.clone()).unwrap(),
                )
            })
            .collect())
    }

    /**
    Compute the out-degree for all nodes in the graph.

    # Examples

    ```
    use graphrs::{Edge, Graph, GraphSpecs};
    let edges = vec![
        Edge::new("n1", "n2"),
        Edge::new("n1", "n3"),
        Edge::new("n1", "n5"),
        Edge::new("n4", "n5"),
    ];
    let graph: Graph<&str, ()> =
        Graph::new_from_nodes_and_edges(vec![], edges, GraphSpecs::directed_create_missing())
            .unwrap();
    let result = graph.get_out_degree_for_all_nodes().unwrap();
    assert_eq!(result.get("n1").unwrap(), &3);
    ```
    */
    pub fn get_out_degree_for_all_nodes(&self) -> Result<HashMap<T, usize>, Error> {
        if !self.specs.directed {
            return Err(Error {
                kind: ErrorKind::WrongMethod,
                message: "Use the `get_degree_for_all_nodes` method when `directed` is `false`"
                    .to_string(),
            });
        }
        Ok(self
            .get_all_nodes()
            .iter()
            .map(|n| {
                (
                    n.name.clone(),
                    self.get_node_out_degree(n.name.clone()).unwrap(),
                )
            })
            .collect())
    }

    /**
    Computes the degree of a given node.
    The node degree is the number of edges adjacent to the node.

    # Arguments

    * `node_name`: the name of the node to get the degree of

    # Examples

    ```
    use graphrs::{generators};
    let graph = generators::social::karate_club_graph();
    assert_eq!(graph.get_node_degree(25).unwrap(), 3);
    ```
    */
    pub fn get_node_degree(&self, node_name: T) -> Option<usize> {
        match self.get_edges_for_node(node_name.clone()) {
            Err(_) => None,
            Ok(edges) => {
                let total_count = edges.len();
                // self-loops are double-counted: https://en.wikipedia.org/wiki/Loop_(graph_theory)
                let self_loops_count = edges
                    .iter()
                    .filter(|e| e.u == node_name && e.v == node_name)
                    .count();
                Some(total_count + self_loops_count)
            }
        }
    }

    /**
    Computes the in-degree of a given node.
    The node in-degree is the number of edges (u, v) where v is the node.

    # Arguments

    * `node_name`: the name of the node to get the in-degree of

    # Examples

    ```
    use graphrs::{Edge, Graph, GraphSpecs};
    let edges = vec![
        Edge::new("n2", "n1"),
        Edge::new("n3", "n1"),
        Edge::new("n4", "n1"),
        Edge::new("n1", "n4"),
    ];
    let graph: Graph<&str, ()> =
        Graph::new_from_nodes_and_edges(vec![], edges, GraphSpecs::directed_create_missing())
            .unwrap();
    assert_eq!(graph.get_node_in_degree("n1").unwrap(), 3);
    ```
    */
    pub fn get_node_in_degree(&self, node_name: T) -> Option<usize> {
        match self.get_in_edges_for_node(node_name) {
            Err(_) => None,
            Ok(edges) => Some(edges.len()),
        }
    }

    /**
    Computes the out-degree of a given node.
    The node out-degree is the number of edges (u, v) where u is the node.

    # Arguments

    * `node_name`: the name of the node to get the out-degree of

    # Examples

    ```
    use graphrs::{Edge, Graph, GraphSpecs};
    let edges = vec![
        Edge::new("n1", "n2"),
        Edge::new("n3", "n1"),
        Edge::new("n4", "n1"),
        Edge::new("n1", "n4"),
    ];
    let graph: Graph<&str, ()> =
        Graph::new_from_nodes_and_edges(vec![], edges, GraphSpecs::directed_create_missing())
            .unwrap();
    assert_eq!(graph.get_node_out_degree("n1").unwrap(), 2);
    ```
    */
    pub fn get_node_out_degree(&self, node_name: T) -> Option<usize> {
        match self.get_out_edges_for_node(node_name) {
            Err(_) => None,
            Ok(edges) => Some(edges.len()),
        }
    }

    /**
    Computes the weighted degree of a given node.
    The weighted degree is sum of the weights of edges adjacent to the node.

    # Arguments

    * `node_name`: the name of the node to get the weighted degree of

    # Examples

    ```
    use graphrs::{Edge, Graph, GraphSpecs};
    let edges = vec![
        Edge::with_weight("n2", "n1", 1.0),
        Edge::with_weight("n3", "n1", 2.0),
        Edge::with_weight("n4", "n1", 3.0),
        Edge::with_weight("n1", "n4", 4.0),
    ];
    let graph: Graph<&str, ()> =
        Graph::new_from_nodes_and_edges(vec![], edges, GraphSpecs::directed_create_missing())
            .unwrap();
    assert_eq!(graph.get_node_weighted_degree("n1").unwrap(), 10.0);
    ```
    */
    pub fn get_node_weighted_degree(&self, node_name: T) -> Option<f64> {
        match self.get_edges_for_node(node_name.clone()) {
            Err(_) => None,
            Ok(edges) => {
                let total_weight: f64 = edges.iter().map(|e| e.weight).sum();
                // self-loops are double-counted: https://en.wikipedia.org/wiki/Loop_(graph_theory)
                let self_loops_weight: f64 = edges
                    .iter()
                    .filter(|e| e.u == node_name && e.v == node_name)
                    .map(|e| e.weight)
                    .sum();
                Some(total_weight + self_loops_weight)
            }
        }
    }

    /**
    Computes the weighted in-degree of a given node.
    The weighted in-degree is sum of the weights of edges into to the node.

    # Arguments

    * `node_name`: the name of the node to get the weighted in-degree of

    # Examples

    ```
    use graphrs::{Edge, Graph, GraphSpecs};
    let edges = vec![
        Edge::with_weight("n2", "n1", 1.0),
        Edge::with_weight("n3", "n1", 2.0),
        Edge::with_weight("n4", "n1", 3.0),
        Edge::with_weight("n1", "n4", 4.0),
    ];
    let graph: Graph<&str, ()> =
        Graph::new_from_nodes_and_edges(vec![], edges, GraphSpecs::directed_create_missing())
            .unwrap();
    assert_eq!(graph.get_node_weighted_in_degree("n1").unwrap(), 6.0);
    ```
    */
    pub fn get_node_weighted_in_degree(&self, node_name: T) -> Option<f64> {
        match self.get_in_edges_for_node(node_name) {
            Err(_) => None,
            Ok(edges) => Some(edges.iter().map(|e| e.weight).sum()),
        }
    }

    /**
    Computes the weighted out-degree of a given node.
    The weighted out-degree is sum of the weights of edges coming from the node.

    # Arguments

    * `node_name`: the name of the node to get the weighted out-degree of

    # Examples

    ```
    use graphrs::{Edge, Graph, GraphSpecs};
    let edges = vec![
        Edge::with_weight("n1", "n2", 1.0),
        Edge::with_weight("n3", "n1", 2.0),
        Edge::with_weight("n4", "n1", 3.0),
        Edge::with_weight("n1", "n4", 4.0),
    ];
    let graph: Graph<&str, ()> =
        Graph::new_from_nodes_and_edges(vec![], edges, GraphSpecs::directed_create_missing())
            .unwrap();
    assert_eq!(graph.get_node_weighted_out_degree("n1").unwrap(), 5.0);
    ```
    */
    pub fn get_node_weighted_out_degree(&self, node_name: T) -> Option<f64> {
        match self.get_out_edges_for_node(node_name) {
            Err(_) => None,
            Ok(edges) => Some(edges.iter().map(|e| e.weight).sum()),
        }
    }

    /**
    Compute the weighted degree for all nodes in the graph.

    # Examples

    ```
    use graphrs::{Edge, Graph, GraphSpecs};
    let edges = vec![
        Edge::with_weight("n1", "n2", 1.0),
        Edge::with_weight("n1", "n3", 2.0),
        Edge::with_weight("n1", "n4", 3.0),
        Edge::with_weight("n4", "n5", 4.0),
    ];
    let graph: Graph<&str, ()> =
        Graph::new_from_nodes_and_edges(vec![], edges, GraphSpecs::directed_create_missing())
            .unwrap();
    let result = graph.get_weighted_degree_for_all_nodes();
    assert_eq!(result.get("n1").unwrap(), &6.0);
    ```
    */
    pub fn get_weighted_degree_for_all_nodes(&self) -> HashMap<T, f64> {
        self.get_all_nodes()
            .iter()
            .map(|n| {
                (
                    n.name.clone(),
                    self.get_node_weighted_degree(n.name.clone()).unwrap(),
                )
            })
            .collect()
    }

    /**
    Compute the weighted in-degree for all nodes in the graph.

    # Examples

    ```
    use graphrs::{Edge, Graph, GraphSpecs};
    let edges = vec![
        Edge::with_weight("n1", "n2", 1.0),
        Edge::with_weight("n1", "n3", 2.0),
        Edge::with_weight("n1", "n5", 3.0),
        Edge::with_weight("n4", "n5", 4.0),
    ];
    let graph: Graph<&str, ()> =
        Graph::new_from_nodes_and_edges(vec![], edges, GraphSpecs::directed_create_missing())
            .unwrap();
    let result = graph.get_weighted_in_degree_for_all_nodes().unwrap();
    assert_eq!(result.get("n5").unwrap(), &7.0);
    ```
    */
    pub fn get_weighted_in_degree_for_all_nodes(&self) -> Result<HashMap<T, f64>, Error> {
        if !self.specs.directed {
            return Err(Error {
                kind: ErrorKind::WrongMethod,
                message:
                    "Use the `get_weighted_degree_for_all_nodes` method when `directed` is `false`"
                        .to_string(),
            });
        }
        Ok(self
            .get_all_nodes()
            .iter()
            .map(|n| {
                (
                    n.name.clone(),
                    self.get_node_weighted_in_degree(n.name.clone()).unwrap(),
                )
            })
            .collect())
    }

    /**
    Compute the weighted out-degree for all nodes in the graph.

    # Examples

    ```
    use graphrs::{Edge, Graph, GraphSpecs};
    let edges = vec![
        Edge::with_weight("n1", "n2", 1.0),
        Edge::with_weight("n1", "n3", 2.0),
        Edge::with_weight("n1", "n5", 3.0),
        Edge::with_weight("n4", "n5", 4.0),
    ];
    let graph: Graph<&str, ()> =
        Graph::new_from_nodes_and_edges(vec![], edges, GraphSpecs::directed_create_missing())
            .unwrap();
    let result = graph.get_weighted_out_degree_for_all_nodes().unwrap();
    assert_eq!(result.get("n1").unwrap(), &6.0);
    ```
    */
    pub fn get_weighted_out_degree_for_all_nodes(&self) -> Result<HashMap<T, f64>, Error> {
        if !self.specs.directed {
            return Err(Error {
                kind: ErrorKind::WrongMethod,
                message:
                    "Use the `get_weighted_degree_for_all_nodes` method when `directed` is `false`"
                        .to_string(),
            });
        }
        Ok(self
            .get_all_nodes()
            .iter()
            .map(|n| {
                (
                    n.name.clone(),
                    self.get_node_weighted_out_degree(n.name.clone()).unwrap(),
                )
            })
            .collect())
    }
}
