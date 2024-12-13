use super::Graph;
use crate::{Edge, Error, ErrorKind, GraphSpecs};
use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;
use std::sync::Arc;

pub enum ToUndirectedCollapseEdgeWeightsStrategy {
    Max,
    Min,
    Sum,
}

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
            .map(|edge| edge.clone().reversed().into())
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
                let mut new_edge: Arc<Edge<T, A>> = edge.clone().into();
                let new_edge_mut = Arc::make_mut(&mut new_edge);
                new_edge_mut.weight = weight;
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
        let new_nodes = self.nodes_vec.clone();
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

    /**
    Converts a directed graph to an undirected graph.

    # Examples

    ```
    use graphrs::{generators};
    let graph = generators::random::fast_gnp_random_graph(25, 0.25, true, Some(1)).unwrap();
    let new_graph = graph.to_undirected(None).unwrap();
    assert_eq!(new_graph.number_of_nodes(), 25);
    assert_eq!(new_graph.number_of_edges(), 132);
    ```
    */
    pub fn to_undirected(
        &self,
        collapse_edge_weights_strategy: Option<ToUndirectedCollapseEdgeWeightsStrategy>,
    ) -> Result<Graph<T, A>, Error>
    where
        T: Hash + Eq + Clone + Ord,
        A: Clone,
    {
        if !self.specs.directed {
            return Err(Error {
                kind: ErrorKind::WrongMethod,
                message: "The `to_undirected` method is not applicable to undirected graphs."
                    .to_string(),
            });
        }
        let new_nodes = self.get_all_nodes().into_iter().cloned().collect();
        let default_weight = match collapse_edge_weights_strategy {
            Some(ToUndirectedCollapseEdgeWeightsStrategy::Max) => f64::MIN,
            Some(ToUndirectedCollapseEdgeWeightsStrategy::Min) => f64::MAX,
            Some(ToUndirectedCollapseEdgeWeightsStrategy::Sum) => 0.0,
            None => f64::NAN,
        };
        let mut edge_map: HashMap<T, HashMap<T, f64>> = HashMap::new();
        for edge in self.get_all_edges() {
            let ordered = edge.ordered();
            let existing_weight = edge_map
                .get(&ordered.u)
                .and_then(|m| m.get(&ordered.v))
                .unwrap_or(&default_weight);
            let new_weight = match collapse_edge_weights_strategy {
                Some(ToUndirectedCollapseEdgeWeightsStrategy::Max) => {
                    edge.weight.max(*existing_weight)
                }
                Some(ToUndirectedCollapseEdgeWeightsStrategy::Min) => {
                    edge.weight.min(*existing_weight)
                }
                Some(ToUndirectedCollapseEdgeWeightsStrategy::Sum) => edge.weight + existing_weight,
                None => f64::NAN,
            };
            edge_map
                .entry(ordered.u)
                .or_insert_with(HashMap::new)
                .insert(ordered.v, new_weight);
        }
        let new_edges = edge_map
            .into_iter()
            .flat_map(|(u, m)| {
                m.into_iter()
                    .map(move |(v, w)| Edge::with_weight(u.clone(), v, w))
            })
            .collect();
        Graph::new_from_nodes_and_edges(
            new_nodes,
            new_edges,
            GraphSpecs {
                directed: false,
                ..self.specs
            },
        )
    }
}

fn collapse_edges<T, A>(tuple: (&(T, T), &Vec<Arc<Edge<T, A>>>)) -> Arc<Edge<T, A>>
where
    T: Eq + Clone + PartialOrd + Ord + Hash + Send + Sync + Display,
    A: Clone,
{
    let (k, v) = tuple;
    let sum_weight = v.iter().map(|e| e.weight).sum();
    Edge::with_weight(k.0.clone(), k.1.clone(), sum_weight)
}
