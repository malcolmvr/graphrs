use crate::merge_attributes::{get_node_with_merged_attributes, AttributeMergeStrategy};
use crate::{Edge, EdgeSide, Error, ErrorKind, GraphSpecs, MissingNodeStrategy, Node};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Graph<T, K, V> {
    nodes: HashMap<T, Node<T, K, V>>,
    edges: HashMap<(T, T), Edge<T, K, V>>,
    specs: GraphSpecs,
    successors: HashMap<T, HashSet<T>>,
    predecessors: HashMap<T, HashSet<T>>,
}

impl<T, K, V> Graph<T, K, V> {
    pub fn add_or_update_edges(
        self,
        new_edges: Vec<Edge<T, K, V>>,
        missing_node_strategy: MissingNodeStrategy,
    ) -> Result<Graph<T, K, V>, Error>
    where
        T: Hash + Eq + Copy + Ord,
        K: Hash + Eq + Copy,
        V: Copy,
    {
        let nodes = self.nodes.values().into_iter().map(|n| n.clone()).collect();
        let combined_edges = self
            .edges
            .values()
            .into_iter()
            .map(|n| n.clone())
            .chain(new_edges)
            .collect();

        Graph::new_from_nodes_and_edges(nodes, combined_edges, self.specs, missing_node_strategy)
    }

    /// Adds nodes to the Graph or updates the attributes of existing nodes.
    /// `merge_strategy` describes how existing and new attributes are to be merged.
    pub fn add_or_update_nodes(
        self,
        nodes: Vec<Node<T, K, V>>,
        merge_strategy: AttributeMergeStrategy,
    ) -> Result<Graph<T, K, V>, Error>
    where
        T: Hash + Eq + Copy + Ord,
        K: Hash + Eq + Copy,
        V: Copy,
    {
        let (existing, new): (Vec<Node<T, K, V>>, Vec<Node<T, K, V>>) = nodes
            .into_iter()
            .partition(|n| self.nodes.contains_key(&n.name));
        let merged = existing.iter().map(|n| {
            get_node_with_merged_attributes(self.nodes.get(&n.name).unwrap(), &n, &merge_strategy)
        });
        let new_nodes = self
            .nodes
            .values()
            .clone()
            .map(|n| (n.name, n.clone()))
            .chain(new.into_iter().clone().map(|n| (n.name, n)))
            .chain(merged.map(|n| (n.name, n)))
            .collect::<HashMap<T, Node<T, K, V>>>();

        Ok(Graph {
            nodes: new_nodes,
            edges: self.edges,
            specs: self.specs,
            predecessors: self.predecessors,
            successors: self.successors,
        })
    }

    pub fn get_all_edges(&self) -> Vec<&Edge<T, K, V>> {
        self.edges.values().collect::<Vec<&Edge<T, K, V>>>()
    }

    pub fn get_all_nodes(&self) -> Vec<&Node<T, K, V>> {
        self.nodes.values().collect::<Vec<&Node<T, K, V>>>()
    }

    pub fn get_edge(&self, u: T, v: T) -> Option<&Edge<T, K, V>>
    where
        T: Hash + Eq + Copy + Ord,
    {
        self.edges.get(&(u, v))
    }

    pub fn get_node(&self, name: T) -> Option<&Node<T, K, V>>
    where
        T: Hash + Eq + Copy + Ord,
    {
        self.nodes.get(&name)
    }

    pub fn get_predecessor_nodes(&self, node_name: T) -> Option<Vec<&Node<T, K, V>>>
    where
        T: Hash + Eq + Copy + Ord,
    {
        let pred = self.predecessors.get(&node_name);
        match pred {
            None => None,
            Some(hashset) => Some(self.get_nodes_for_names(&hashset)),
        }
    }

    pub fn get_predecessors_map(&self) -> &HashMap<T, HashSet<T>> {
        &self.predecessors
    }

    pub fn get_successor_nodes(&self, node_name: T) -> Option<Vec<&Node<T, K, V>>>
    where
        T: Hash + Eq + Copy + Ord,
    {
        let pred = self.successors.get(&node_name);
        match pred {
            None => None,
            Some(hashset) => Some(self.get_nodes_for_names(&hashset)),
        }
    }

    pub fn get_successors_map(&self) -> &HashMap<T, HashSet<T>> {
        &self.successors
    }

    pub fn new_from_nodes_and_edges(
        nodes: Vec<Node<T, K, V>>,
        edges: Vec<Edge<T, K, V>>,
        specs: GraphSpecs,
        missing_node_strategy: MissingNodeStrategy,
    ) -> Result<Graph<T, K, V>, Error>
    where
        T: Hash + Eq + Copy + Ord,
        K: Hash + Eq + Copy,
        V: Copy,
    {
        let node_names = nodes.iter().map(|n| n.name).collect::<HashSet<T>>();

        let successors = dedupe_and_group_edges(&edges, EdgeSide::U);
        let predecessors = dedupe_and_group_edges(&edges, EdgeSide::V);

        let edges_map = edges
            .into_iter()
            .map(|e| ((e.u, e.v), e))
            .collect::<HashMap<(T, T), Edge<T, K, V>>>();

        let missing_nodes = edges_map
            .iter()
            .flat_map(|(_k, e)| vec![e.u, e.v])
            .filter(|name| !node_names.contains(name))
            .map(|name| Node::<T, K, V>::from_name(name))
            .collect::<Vec<Node<T, K, V>>>();

        if missing_node_strategy == MissingNodeStrategy::Error && missing_nodes.len() > 0 {
            return Err(Error {
                kind: ErrorKind::NodeMissing,
                message: "missing node".to_string(),
            });
        }

        // missing_node_strategy == MissingNodeStrategy::Create

        let nodes_map = nodes
            .into_iter()
            .chain(missing_nodes)
            .map(|n| (n.name, n))
            .collect::<HashMap<T, Node<T, K, V>>>();

        Ok(Graph {
            nodes: nodes_map,
            edges: edges_map,
            specs,
            successors,
            predecessors,
        })
    }

    // PRIVATE METHODS

    fn get_nodes_for_names(&self, names: &HashSet<T>) -> Vec<&Node<T, K, V>>
    where
        T: Hash + Eq + Copy + Ord,
    {
        names
            .into_iter()
            .map(|n| self.nodes.get(n).unwrap())
            .collect::<Vec<&Node<T, K, V>>>()
    }
}

fn dedupe_and_group_edges<T, K, V>(
    edges: &Vec<Edge<T, K, V>>,
    by: EdgeSide,
) -> HashMap<T, HashSet<T>>
where
    T: Hash + Eq + Copy + Ord,
    K: Hash + Eq + Copy,
    V: Copy,
{
    let key_val = match by {
        EdgeSide::U => |e: &Edge<T, K, V>| (e.u, e.v),
        EdgeSide::V => |e: &Edge<T, K, V>| (e.v, e.u),
    };
    edges
        .into_iter()
        .map(key_val)
        .sorted_by(|a, b| Ord::cmp(&a.0, &b.0))
        .group_by(|t| t.0)
        .into_iter()
        .map(|(k, g)| (k, g.map(|t| t.1).collect::<HashSet<T>>()))
        .collect::<HashMap<T, HashSet<T>>>()
}
