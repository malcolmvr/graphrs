use super::Graph;
use std::fmt::Display;
use std::hash::Hash;

impl<T, A> Graph<T, A>
where
    T: Eq + Clone + PartialOrd + Ord + Hash + Send + Sync + Display,
    A: Clone,
{
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
        match self.get_edges_for_node(node_name) {
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
        match self.get_edges_for_node(node_name) {
            Err(_) => None,
            Ok(edges) => Some(edges.iter().map(|e| e.weight).sum()),
        }
    }
}
