use super::Graph;
use crate::{Edge, Error, ErrorKind, GraphSpecs};
use std::fmt::Display;
use std::hash::Hash;

impl<T, A> Graph<T, A>
where
    T: Eq + Clone + PartialOrd + Ord + Hash + Send + Sync + Display,
    A: Clone,
{
    /**
    Reverses the edges in a directed graph.

    # Examples

    ```
    use graphrs::{Edge, Graph, GraphSpecs};

    let mut graph: Graph<&str, ()> = Graph::new(GraphSpecs::directed_create_missing());
    let result = graph.add_edges(vec![
        Edge::new("n1", "n3"),
        Edge::new("n2", "n3"),
    ]);
    let graph = graph.reverse().unwrap();
    assert!(graph.get_edge("n3", "n1").is_ok());
    ```
    */
    pub fn reverse(&self) -> Result<Graph<T, A>, Error> {
        if !self.specs.directed {
            return Err(Error {
                kind: ErrorKind::WrongMethod,
                message: "The `reverse` method is not applicable to undirected graphs.".to_string(),
            });
        }
        let new_nodes = self.get_all_nodes().into_iter().cloned().collect();
        let new_edges = self
            .get_all_edges()
            .into_iter()
            .map(|edge| edge.clone().reversed())
            .collect();
        Graph::new_from_nodes_and_edges(new_nodes, new_edges, self.specs.clone())
    }

    /**
    Return a new graph with all the edge weights set to the specified value.

    # Arguments

    * `weight`: the value to set all the edge weights to

    # Examples

    ```
    use graphrs::{Edge, Graph, GraphSpecs};

    let mut graph: Graph<&str, ()> = Graph::new(GraphSpecs::directed_create_missing());
    let result = graph.add_edges(vec![
        Edge::new("n1", "n3"),
        Edge::new("n2", "n3"),
    ]);
    let new_graph = graph.set_all_edge_weights(2.0);
    assert_eq!(new_graph.get_edge("n1", "n3").unwrap().weight, 2.0);
    ```
    */
    pub fn set_all_edge_weights(&self, weight: f64) -> Graph<T, A> {
        let new_nodes = self.get_all_nodes().into_iter().cloned().collect();
        let new_edges = self
            .get_all_edges()
            .into_iter()
            .map(|edge| {
                let mut new_edge = edge.clone();
                new_edge.weight = weight;
                new_edge
            })
            .collect();
        Graph::new_from_nodes_and_edges(new_nodes, new_edges, self.specs.clone()).unwrap()
    }

    /**
    Convert a multi-edge graph to a single-edge graph.

    Edge weights are summed.

    Edge attributes are lost.

    # Examples

    ```
    use graphrs::{Edge, Graph, GraphSpecs, MissingNodeStrategy, Node};
    let mut graph: Graph<&str, ()> = Graph::new(GraphSpecs {
        missing_node_strategy: MissingNodeStrategy::Create,
        multi_edges: true,
        ..GraphSpecs::directed()
    });
    graph.add_edges(vec![
        Edge::with_weight("n1", "n2", 1.1),
        Edge::with_weight("n1", "n2", 2.2),
        Edge::with_weight("n1", "n2", 3.3),
        Edge::with_weight("n1", "n3", 4.4),
    ]);
    let new_graph = graph.to_single_edges().unwrap();
    assert!(!new_graph.specs.multi_edges);
    assert_eq!(new_graph.get_all_edges().len(), 2);
    assert_eq!(new_graph.get_edge("n1", "n2").unwrap().weight, 6.6);
    assert_eq!(new_graph.get_edge("n1", "n3").unwrap().weight, 4.4);
    ```
    */
    pub fn to_single_edges(&self) -> Result<Graph<T, A>, Error> {
        if !self.specs.multi_edges {
            return Err(Error {
                kind: ErrorKind::WrongMethod,
                message: "The `to_single_edges` method is not applicable to graph where `specs.multi_edges` is `false`.".to_string(),
            });
        }
        let new_nodes = self.nodes.values().cloned().collect();
        let new_edges = self.edges.iter().map(collapse_edges).collect();
        Graph::new_from_nodes_and_edges(
            new_nodes,
            new_edges,
            GraphSpecs {
                multi_edges: false,
                ..self.specs.clone()
            },
        )
    }
}

fn collapse_edges<T, A>(tuple: (&(T, T), &Vec<Edge<T, A>>)) -> Edge<T, A>
where
    T: Eq + Clone + PartialOrd + Ord + Hash + Send + Sync + Display,
    A: Clone,
{
    let (k, v) = tuple;
    let sum_weight = v.iter().map(|e| e.weight).sum();
    Edge::with_weight(k.0.clone(), k.1.clone(), sum_weight)
}
