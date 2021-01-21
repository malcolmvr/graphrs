use crate::merge_attributes::{get_node_with_merged_attributes, AttributeMergeStrategy};
use crate::{
    Edge, EdgeSide, Error, ErrorKind, GraphSpecs, MissingNodeStrategy, Node, SelfLoopsFalseStrategy,
};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Graph<T, K, V> {
    nodes: HashMap<T, Node<T, K, V>>,
    edges: HashMap<(T, T), Vec<Edge<T, K, V>>>,
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
            .flatten()
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
        self.edges
            .values()
            .into_iter()
            .flatten()
            .collect::<Vec<&Edge<T, K, V>>>()
    }

    pub fn get_all_nodes(&self) -> Vec<&Node<T, K, V>> {
        self.nodes.values().collect::<Vec<&Node<T, K, V>>>()
    }

    pub fn get_edge(&self, u: T, v: T) -> Result<&Edge<T, K, V>, Error>
    where
        T: Hash + Eq + Copy + Ord,
    {
        if self.specs.multi_edges {
            return Err(Error {
                kind: ErrorKind::WrongMethod,
                message: "use the `get_edges` method when `multi_edges` is true".to_string(),
            });
        }

        let edges = self.edges.get(&(u, v));
        match edges {
            None => Err(Error {
                kind: ErrorKind::NoEdge,
                message: "the requested edge does not exist".to_string(),
            }),
            Some(e) => Ok(&e[0]),
        }
    }

    pub fn get_edges(&self, u: T, v: T) -> Result<&Vec<Edge<T, K, V>>, Error>
    where
        T: Hash + Eq + Copy + Ord,
    {
        if !self.specs.multi_edges {
            return Err(Error {
                kind: ErrorKind::WrongMethod,
                message: "use the `get_edge` method when `multi_edges` is false".to_string(),
            });
        }

        let edges = self.edges.get(&(u, v));
        match edges {
            None => Err(Error {
                kind: ErrorKind::NoEdge,
                message: "the requested edge does not exist".to_string(),
            }),
            Some(e) => Ok(&e),
        }
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

        let (successors, predecessors) = match specs.directed {
            true => get_directed_successors_predecessors(&edges),
            false => get_undirected_successors_predecessors(&edges),
        };

        let edges_map = match get_edges_map_for_specs(edges, &specs) {
            Err(e) => {
                return Err(e);
            }
            Ok(em) => em,
        };

        let missing_nodes = edges_map
            .values()
            .into_iter()
            .flatten()
            .map(|e| vec![e.u, e.v])
            .flatten()
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

fn get_edges_map_for_specs<T: std::cmp::Ord, K, V>(
    edges: Vec<Edge<T, K, V>>,
    specs: &GraphSpecs,
) -> Result<HashMap<(T, T), Vec<Edge<T, K, V>>>, Error>
where
    T: Hash + Eq + Copy + Ord,
    K: Hash + Eq + Copy,
    V: Copy,
{
    let edges_len = edges.len();

    let u_v_orderer = match specs.directed {
        true => |e| e,
        false => |e: Edge<T, K, V>| match e.u > e.v {
            true => e.reversed(),
            false => e,
        },
    };

    // if specs.self_loops is false:
    //   filter the edges if specs.self_loops_false_strategy is Drop
    //   return Err if self loops detected

    let sorted_edges = edges
        .into_iter()
        .map(u_v_orderer)
        .sorted_by(|e1, e2| Ord::cmp(&e1.u, &e2.u));

    let x =
        match specs.self_loops && specs.self_loops_false_strategy == SelfLoopsFalseStrategy::Drop {
            true => sorted_edges.collect::<Vec<Edge<T, K, V>>>(),
            false => sorted_edges
                .filter(|e| e.u == e.v)
                .into_iter()
                .collect::<Vec<Edge<T, K, V>>>(),
        };

    if x.len() < edges_len {
        return Err(Error {
            kind: ErrorKind::SelfLoopsFound,
            message: "edges contain self-loops and `specs.self_loops` is false".to_string(),
        });
    }

    let deduped = match specs.multi_edges {
        true => x,
        false => x.into_iter().dedup().collect::<Vec<Edge<T, K, V>>>(),
    };

    let grouped = deduped
        .into_iter()
        .group_by(|e| (e.u, e.v))
        .into_iter()
        .map(|(k, g)| (k, g.collect::<Vec<Edge<T, K, V>>>()))
        .collect::<HashMap<(T, T), Vec<Edge<T, K, V>>>>();

    Ok(grouped)
}

fn get_directed_successors_predecessors<T, K, V>(
    edges: &Vec<Edge<T, K, V>>,
) -> (HashMap<T, HashSet<T>>, HashMap<T, HashSet<T>>)
where
    T: Hash + Eq + Copy + Ord,
    K: Hash + Eq + Copy,
    V: Copy,
{
    let successors = dedupe_and_group_edges(edges, EdgeSide::U);
    let predecessors = dedupe_and_group_edges(edges, EdgeSide::V);
    (successors, predecessors)
}

fn get_undirected_successors_predecessors<T, K, V>(
    edges: &Vec<Edge<T, K, V>>,
) -> (HashMap<T, HashSet<T>>, HashMap<T, HashSet<T>>)
where
    T: Hash + Eq + Copy + Ord,
    K: Hash + Eq + Copy,
    V: Copy,
{
    let neighbors = edges
        .into_iter()
        .map(|e| (e.u, e.v))
        .chain(edges.into_iter().map(|e| (e.v, e.u)))
        .sorted_by(|a, b| Ord::cmp(&a.0, &b.0))
        .group_by(|t| t.0)
        .into_iter()
        .map(|(k, g)| (k, g.map(|t| t.1).collect::<HashSet<T>>()))
        .collect::<HashMap<T, HashSet<T>>>();

    // (1,2)
    // (1,3)
    // (4,1)
    // (4,2)

    // 1: 2, 3, 4
    // 2: 1, 5
    // 3: 1
    // 4: 1, 2

    (neighbors, HashMap::new())
}
