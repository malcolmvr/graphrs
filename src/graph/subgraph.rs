use super::Graph;
use crate::{Edge, Node};
use std::collections::HashSet;
use std::fmt::Display;
use std::hash::Hash;

impl<T, A> Graph<T, A>
where
    T: Eq + Clone + PartialOrd + Ord + Hash + Send + Sync + Display,
    A: Clone,
{
    /**
    Returns an induced subgraph that contains only the specified nodes
    and the edges between those nodes.

    # Arguments

    * `nodes`: The nodes the subgraph must contain.

    # Examples

    ```
    use graphrs::generators;
    let graph = generators::social::karate_club_graph();
    let subgraph = graph.get_subgraph(&vec![4, 5, 6, 10, 16]);
    assert_eq!(subgraph.get_all_nodes().len(), 5);
    ```
    */
    pub fn get_subgraph(&self, nodes: &[T]) -> Graph<T, A> {
        let nodes_set: HashSet<T> = nodes.iter().cloned().collect();
        let new_nodes = self
            .get_all_nodes()
            .into_iter()
            .filter(|n| nodes_set.contains(&n.name))
            .cloned()
            .collect::<Vec<Node<T, A>>>();
        let new_edges = self
            .get_all_edges()
            .into_iter()
            .filter(|e| nodes_set.contains(&e.u) && nodes_set.contains(&e.v))
            .cloned()
            .collect::<Vec<Edge<T, A>>>();
        Graph::new_from_nodes_and_edges(new_nodes, new_edges, self.specs.clone()).unwrap()
    }
}
