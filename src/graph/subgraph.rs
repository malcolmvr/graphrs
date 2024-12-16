use super::Graph;
use crate::{Edge, Error, ErrorKind, Node};
use std::collections::HashSet;
use std::fmt::Display;
use std::hash::Hash;
use std::sync::Arc;

impl<T, A> Graph<T, A>
where
    T: Eq + Clone + PartialOrd + Ord + Hash + Send + Sync + Display,
    A: Clone,
{
    /**
    Returns the 1st-degree ego network for a specific node.

    # Arguments

    * `node`: The node to get the ego network for.

    # Examples

    ```
    use graphrs::generators;
    let graph = generators::social::karate_club_graph();
    let subgraph = graph.get_ego_graph(&4).unwrap();
    assert_eq!(subgraph.get_all_nodes().len(), 4);
    ```
    */
    pub fn get_ego_graph(&self, node: &T) -> Result<Graph<T, A>, Error> {
        if !self.has_node(node) {
            return Err(Error {
                kind: ErrorKind::NodeNotFound,
                message: "The node was not found in graph".to_string(),
            });
        }
        let neighbors: Vec<&T> = match self.specs.directed {
            true => self
                .get_successor_node_names(node.clone())
                .unwrap()
                .into_iter()
                .chain(
                    self.get_predecessor_node_names(node.clone())
                        .unwrap()
                        .into_iter(),
                )
                .collect(),
            false => self
                .get_neighbor_nodes(node.clone())
                .unwrap()
                .into_iter()
                .map(|n| &n.name)
                .collect(),
        };
        let mut neighbors: Vec<T> = neighbors.into_iter().cloned().collect();
        neighbors.push(node.clone());
        self.get_subgraph(&neighbors)
    }

    /**
    Returns an induced subgraph that contains only the specified nodes
    and the edges between those nodes.

    # Arguments

    * `nodes`: The nodes the subgraph must contain.

    # Examples

    ```
    use graphrs::generators;
    let graph = generators::social::karate_club_graph();
    let subgraph = graph.get_subgraph(&vec![4, 5, 6, 10, 16]).unwrap();
    assert_eq!(subgraph.get_all_nodes().len(), 5);
    ```
    */
    pub fn get_subgraph(&self, nodes: &[T]) -> Result<Graph<T, A>, Error> {
        if !self.has_nodes(nodes) {
            return Err(Error {
                kind: ErrorKind::NodeNotFound,
                message: "The node was not found in graph".to_string(),
            });
        }
        Ok(_get_subgraph(self, nodes))
    }
}

fn _get_subgraph<T, A>(graph: &Graph<T, A>, nodes: &[T]) -> Graph<T, A>
where
    T: Eq + Clone + PartialOrd + Ord + Hash + Send + Sync + Display,
    A: Clone,
{
    let nodes_set: HashSet<T> = nodes.iter().cloned().collect();
    let new_nodes = graph
        .get_all_nodes()
        .into_iter()
        .filter(|n| nodes_set.contains(&n.name))
        .cloned()
        .collect::<Vec<Arc<Node<T, A>>>>();
    let new_edges = graph
        .get_all_edges()
        .into_iter()
        .filter(|e| nodes_set.contains(&e.u) && nodes_set.contains(&e.v))
        .cloned()
        .collect::<Vec<Arc<Edge<T, A>>>>();
    Graph::new_from_nodes_and_edges(new_nodes, new_edges, graph.specs.clone()).unwrap()
}
