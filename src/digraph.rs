use crate::merge_attributes::{get_node_with_merged_attributes, AttributeMergeStrategy};
use crate::{Edge, EdgeSide, Error, ErrorKind, Node};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

pub enum MissingNodeStrategy {
    Create,
    Error,
}

pub struct DiGraph<T, K, V> {
    nodes: HashMap<T, Node<T, K, V>>,
    edges: HashMap<(T, T), Edge<T, K, V>>,
    successors: HashMap<T, HashSet<T>>,
    predecessors: HashMap<T, HashSet<T>>,
}

impl<T, K, V> DiGraph<T, K, V> {
    pub fn add_or_update_edges(
        mut self,
        edges: Vec<Edge<T, K, V>>,
    ) -> Result<DiGraph<T, K, V>, Error>
    where
        T: Hash + Eq + Copy + Ord,
        K: Hash + Eq + Copy,
        V: Copy,
    {
        self.successors =
            add_edges_to_successors_or_predecessors(self.successors, &edges, EdgeSide::U);
        self.predecessors =
            add_edges_to_successors_or_predecessors(self.predecessors, &edges, EdgeSide::V);

        self.edges
            .extend(edges.into_iter().map(|e| ((e.u, e.v), e)));

        Ok(DiGraph {
            nodes: self.nodes,
            edges: self.edges,
            predecessors: self.predecessors,
            successors: self.successors,
        })
    }

    /// Adds nodes to the DiGraph or updates the attributes of existing nodes.
    /// `merge_strategy` describes how existing and new attributes are to be merged.
    pub fn add_or_update_nodes(
        mut self,
        nodes: Vec<Node<T, K, V>>,
        merge_strategy: AttributeMergeStrategy,
    ) -> Result<DiGraph<T, K, V>, Error>
    where
        T: Hash + Eq + Copy + Ord,
        K: Hash + Eq + Copy,
        V: Copy,
    {
        let (existing, new): (Vec<Node<T, K, V>>, Vec<Node<T, K, V>>) = nodes
            .into_iter()
            .partition(|n| self.nodes.contains_key(&n.name));
        let nm = self.nodes.clone();
        self.nodes.extend(new.into_iter().map(|n| (n.name, n)));
        self.nodes.extend(existing.iter().map(|n| {
            (
                n.name,
                get_node_with_merged_attributes(nm.get(&n.name).unwrap(), &n, &merge_strategy),
            )
        }));

        Ok(DiGraph {
            nodes: self.nodes,
            edges: self.edges,
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

    pub fn get_node(&self, name: &T) -> Option<&Node<T, K, V>>
    where
        T: Hash + Eq + Copy + Ord,
    {
        self.nodes.get(name)
    }

    pub fn get_predecessors_map(&self) -> &HashMap<T, HashSet<T>> {
        &self.predecessors
    }

    pub fn get_predecessor_nodes(&self, node_name: &T) -> Option<Vec<&Node<T, K, V>>>
    where
        T: Hash + Eq + Copy + Ord,
    {
        let pred = self.predecessors.get(node_name);
        match pred {
            None => None,
            Some(hashset) => Some(self.get_nodes_for_names(&hashset)),
        }
    }

    pub fn get_successors_map(&self) -> &HashMap<T, HashSet<T>> {
        &self.successors
    }

    pub fn get_successor_nodes(&self, node_name: &T) -> Option<Vec<&Node<T, K, V>>>
    where
        T: Hash + Eq + Copy + Ord,
    {
        let pred = self.successors.get(node_name);
        match pred {
            None => None,
            Some(hashset) => Some(self.get_nodes_for_names(&hashset)),
        }
    }

    pub fn new_from_nodes_and_edges(
        nodes: Vec<Node<T, K, V>>,
        edges: Vec<Edge<T, K, V>>,
        missing_node_strategy: MissingNodeStrategy,
    ) -> Result<DiGraph<T, K, V>, Error>
    where
        T: Hash + Eq + Copy + Ord,
        K: Hash + Eq + Copy,
        V: Copy,
    {
        let mut nodes_map = nodes
            .into_iter()
            .map(|n| (n.name, n))
            .collect::<HashMap<T, Node<T, K, V>>>();

        let successors = dedupe_and_group_edges(&edges, EdgeSide::U);
        let predecessors = dedupe_and_group_edges(&edges, EdgeSide::V);

        let edges_map = edges
            .into_iter()
            .map(|e| ((e.u, e.v), e))
            .collect::<HashMap<(T, T), Edge<T, K, V>>>();

        let missing_nodes = edges_map
            .iter()
            .flat_map(|(_k, e)| vec![e.u, e.v])
            .filter(|name| !nodes_map.contains_key(name))
            .map(|name| (name, Node::<T, K, V>::from_name(name)))
            .collect::<Vec<(T, Node<T, K, V>)>>();

        match missing_node_strategy {
            MissingNodeStrategy::Create => {
                nodes_map.extend(missing_nodes);
            }
            MissingNodeStrategy::Error => {
                if missing_nodes.len() > 0 {
                    return Err(Error {
                        kind: ErrorKind::NodeMissing,
                        message: "missing node".to_string(),
                    });
                }
            }
        }

        Ok(DiGraph {
            nodes: nodes_map,
            edges: edges_map,
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

fn get_adjacency_hashmap<T, I>(tuples: I, side: EdgeSide) -> HashMap<T, HashSet<T>>
where
    I: Iterator<Item = (T, T)>,
    T: Hash + Eq + Copy + Ord,
{
    let get_key_val = match side {
        EdgeSide::U => |t: (T, T)| (t.0, t.1),
        EdgeSide::V => |t: (T, T)| (t.1, t.0),
    };
    tuples
        .map(get_key_val)
        .sorted_by(|a, b| Ord::cmp(&a.0, &b.0))
        .group_by(|t| t.0)
        .into_iter()
        .map(|(k, g)| (k, g.map(|t| t.1).collect::<HashSet<T>>()))
        .collect::<HashMap<T, HashSet<T>>>()
}

fn add_edges_to_successors_or_predecessors<T, K, V>(
    pred_or_succ: HashMap<T, HashSet<T>>,
    edges: &Vec<Edge<T, K, V>>,
    side: EdgeSide,
) -> HashMap<T, HashSet<T>>
where
    T: Hash + Eq + Copy + Ord,
{
    let get_key_val = match side {
        EdgeSide::U => |e: &Edge<T, K, V>| (e.u, e.v),
        EdgeSide::V => |e: &Edge<T, K, V>| (e.v, e.u),
    };
    let x = pred_or_succ
        .into_iter()
        .map(|(k, v)| v.into_iter().map(move |x| (k, x)))
        .flatten()
        .chain(edges.into_iter().map(get_key_val));
    get_adjacency_hashmap(x, side)
}
