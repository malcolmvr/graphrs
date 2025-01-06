use super::Partition;
use crate::{Edge, Graph, Node};
use nohash::IntSet;
use std::fmt::Display;
use std::hash::Hash;
use std::sync::Arc;

pub(crate) struct AggregateGraph<'a, T, A>
where
    T: Hash + Eq + Clone + Ord + Display + Send + Sync + PartialOrd,
    A: Clone + Send + Sync,
{
    pub graph: Graph<T, A>,
    pub node_nodes: Option<Vec<IntSet<usize>>>,
    pub node_weights: Option<Vec<f64>>,
    pub parent_graph: Option<&'a Graph<T, A>>,
    pub parent_partition: Option<&'a Partition>,
}

impl<'a, T, A> AggregateGraph<'a, T, A>
where
    T: Hash + Eq + Clone + Ord + Display + Send + Sync + PartialOrd,
    A: Clone + Send + Sync,
{
    pub fn initial(graph: &Graph<T, A>, weighted: bool) -> Self {
        let nodes: Vec<Arc<Node<T, A>>> = graph
            .get_all_node_names()
            .into_iter()
            .map(|name| Node::from_name(name.clone()))
            .collect();
        let edges: Vec<Arc<Edge<T, A>>> = graph
            .get_all_edges()
            .into_iter()
            .map(|edge| match weighted {
                true => edge.clone(),
                false => Edge::with_weight(edge.u.clone(), edge.v.clone(), 1.0),
            })
            .collect();
        let weighted_graph =
            Graph::<T, A>::new_from_nodes_and_edges(nodes, edges, graph.specs.clone()).unwrap();

        AggregateGraph {
            graph: weighted_graph,
            node_nodes: None,
            node_weights: None,
            parent_graph: None,
            parent_partition: None,
        }
    }

    pub fn node_total(&self, community: &IntSet<usize>) -> f64 {
        if self.node_weights.is_none() {
            return community.len() as f64;
        }
        community
            .iter()
            .map(|node| self.node_weights.as_ref().unwrap()[*node])
            .sum()
    }
}
