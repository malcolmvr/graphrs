use super::Graph;
use crate::{Error, ErrorKind};
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
}
