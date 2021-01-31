use crate::{
    Edge, EdgeDedupeStrategy, EdgeSide, Error, ErrorKind, GraphSpecs, MissingNodeStrategy, Node,
    SelfLoopsFalseStrategy,
};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::hash::Hash;

/**
The `Graph` struct represents a graph of nodes and vertices.
It allows graphs to be created with support for:
* directed and undirected edges
* multiple edges between two nodes
* self-loops
* acyclic enforcement
# Example
```
use graphrs::{Edge, Graph, GraphSpecs, MissingNodeStrategy, Node};

let nodes = vec![
    Node::from_name("n1"),
    Node::from_name("n2"),
    Node::from_name("n3"),
];

let edges = vec![
    Edge::with_weight("n1", "n2", &1.0),
    Edge::with_weight("n2", "n1", &2.0),
    Edge::with_weight("n1", "n3", &3.0),
    Edge::with_weight("n2", "n3", &3.0),
];

let specs = GraphSpecs::directed();

let graph = Graph::<&str, &str, &f64>::new_from_nodes_and_edges(
    nodes,
    edges,
    specs
);
```
**/
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Graph<T, K, V> {
    nodes: HashMap<T, Node<T, K, V>>,
    edges: HashMap<(T, T), Vec<Edge<T, K, V>>>,
    specs: GraphSpecs,
    successors: HashMap<T, HashSet<T>>,
    predecessors: HashMap<T, HashSet<T>>,
}

impl<T: std::fmt::Display, K, V> Graph<T, K, V> {
    /// Adds new edges to a `Graph`, or updates existing edges, or both.
    /// If the new edges reference nodes that don't exist the `missing_node_strategy` argument determines what happens.
    /// The constraints in the graph's `specs` field (e.g. `acyclic`) will be applied to the resulting set of edges.
    /// The graph is not mutated; this method returns a new `Graph` instance.
    pub fn add_or_update_edges(self, new_edges: Vec<Edge<T, K, V>>) -> Result<Graph<T, K, V>, Error>
    where
        T: Hash + Eq + Copy + Ord,
        K: Hash + Eq + Copy,
        V: Copy,
    {
        let nodes = self.nodes.values().into_iter().map(|n| n.clone()).collect();
        let combined_edges: Vec<Edge<T, K, V>> = self
            .edges
            .values()
            .into_iter()
            .flatten()
            .map(|e| e.clone())
            .chain(new_edges)
            .collect();
        for edge in combined_edges.iter() {
            println!("{}", edge);
        }
        Graph::new_from_nodes_and_edges(nodes, combined_edges, self.specs)
    }

    /// Adds nodes to the Graph or updates the attributes of existing nodes.
    /// `merge_strategy` describes how existing and new attributes are to be merged.
    pub fn add_or_update_nodes(self, nodes: Vec<Node<T, K, V>>) -> Result<Graph<T, K, V>, Error>
    where
        T: Hash + Eq + Copy + Ord,
        K: Hash + Eq + Copy,
        V: Copy,
    {
        let (existing, new): (Vec<Node<T, K, V>>, Vec<Node<T, K, V>>) = nodes
            .into_iter()
            .partition(|n| self.nodes.contains_key(&n.name));
        let new_nodes = self
            .nodes
            .values()
            .clone()
            .map(|n| (n.name, n.clone()))
            .chain(new.into_iter().clone().map(|n| (n.name, n)))
            .chain(existing.into_iter().map(|n| (n.name, n)))
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

        let ordered = match !self.specs.directed && u > v {
            false => (u, v),
            true => (v, u),
        };

        let edges = self.edges.get(&ordered);
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

    pub fn get_neighbor_nodes(&self, node_name: T) -> Result<Vec<&Node<T, K, V>>, Error>
    where
        T: Hash + Eq + Copy + Ord,
    {
        if !self.nodes.contains_key(&node_name) {
            return Err(Error {
                kind: ErrorKind::NodeNotFound,
                message: format!("node '{}' not found in the graph", node_name),
            });
        }

        let pred_nodes = self._get_predecessor_nodes(node_name).unwrap();
        let succ_nodes = self._get_successor_nodes(node_name).unwrap();

        Ok(pred_nodes.into_iter().chain(succ_nodes).collect())
    }

    pub fn get_node(&self, name: T) -> Option<&Node<T, K, V>>
    where
        T: Hash + Eq + Copy + Ord,
    {
        self.nodes.get(&name)
    }

    pub fn get_predecessor_nodes(&self, node_name: T) -> Result<Vec<&Node<T, K, V>>, Error>
    where
        T: Hash + Eq + Copy + Ord,
    {
        if !self.specs.directed {
            return Err(Error {
                kind: ErrorKind::WrongMethod,
                message: "for undirected graphs use the `get_neighbor_nodes` method instead of `get_predecessor_nodes`".to_string(),
            });
        }

        self._get_predecessor_nodes(node_name)
    }

    pub fn _get_predecessor_nodes(&self, node_name: T) -> Result<Vec<&Node<T, K, V>>, Error>
    where
        T: Hash + Eq + Copy + Ord,
    {
        if !self.nodes.contains_key(&node_name) {
            return Err(Error {
                kind: ErrorKind::NodeNotFound,
                message: format!("node '{}' not found in the graph", node_name),
            });
        }
        let pred = self.predecessors.get(&node_name);
        match pred {
            None => Ok(vec![]),
            Some(hashset) => Ok(self.get_nodes_for_names(&hashset)),
        }
    }

    pub fn get_predecessors_map(&self) -> &HashMap<T, HashSet<T>> {
        &self.predecessors
    }

    pub fn get_successor_nodes(&self, node_name: T) -> Result<Vec<&Node<T, K, V>>, Error>
    where
        T: Hash + Eq + Copy + Ord,
    {
        if !self.specs.directed {
            return Err(Error {
                kind: ErrorKind::WrongMethod,
                message: "for undirected graphs use the `get_neighbor_nodes` method instead of `get_successor_nodes`".to_string(),
            });
        }

        self._get_successor_nodes(node_name)
    }

    pub fn _get_successor_nodes(&self, node_name: T) -> Result<Vec<&Node<T, K, V>>, Error>
    where
        T: Hash + Eq + Copy + Ord,
    {
        if !self.nodes.contains_key(&node_name) {
            return Err(Error {
                kind: ErrorKind::NodeNotFound,
                message: format!("node '{}' not found in the graph", node_name),
            });
        }
        let succ = self.successors.get(&node_name);
        match succ {
            None => Ok(vec![]),
            Some(hashset) => Ok(self.get_nodes_for_names(&hashset)),
        }
    }

    pub fn get_successors_map(&self) -> &HashMap<T, HashSet<T>> {
        &self.successors
    }

    pub fn new_from_nodes_and_edges(
        nodes: Vec<Node<T, K, V>>,
        edges: Vec<Edge<T, K, V>>,
        specs: GraphSpecs,
    ) -> Result<Graph<T, K, V>, Error>
    where
        T: Hash + Eq + Copy + Ord,
        K: Hash + Eq + Copy,
        V: Copy,
    {
        let node_names = nodes.iter().map(|n| n.name).collect::<HashSet<T>>();

        let edges_map = match get_edges_map_for_specs(edges, &specs) {
            Err(e) => return Err(e),
            Ok(em) => em,
        };

        let edges_for_specs = &edges_map
            .values()
            .into_iter()
            .flatten()
            .collect::<Vec<&Edge<T, K, V>>>();

        let (successors, predecessors) = match specs.directed {
            true => get_directed_successors_predecessors(edges_for_specs),
            false => get_undirected_successors_predecessors(edges_for_specs),
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

        if specs.missing_node_strategy == MissingNodeStrategy::Error && missing_nodes.len() > 0 {
            return Err(Error {
                kind: ErrorKind::NodeNotFound,
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
    edges: &Vec<&Edge<T, K, V>>,
    by: EdgeSide,
) -> HashMap<T, HashSet<T>>
where
    T: Hash + Eq + Copy + Ord,
    K: Hash + Eq + Copy,
    V: Copy,
{
    let key_val = match by {
        EdgeSide::U => |e: &&Edge<T, K, V>| (e.u, e.v),
        EdgeSide::V => |e: &&Edge<T, K, V>| (e.v, e.u),
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
    T: Hash + Eq + Copy + Ord + Display,
    K: Hash + Eq + Copy,
    V: Copy,
{
    let deduped_result = match specs.multi_edges {
        true => Ok(edges),
        false => get_deduped_edges(edges, &specs.edge_dedupe_strategy, &specs.directed),
    };

    match &deduped_result {
        Err(e) => return Err(e.clone()),
        Ok(_r) => {}
    }

    let deduped = deduped_result.unwrap();
    let edges_len = deduped.len();

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

    let sorted_edges = deduped
        .into_iter()
        .map(u_v_orderer)
        .sorted_by(|e1, e2| Ord::cmp(&e1.u, &e2.u));

    let processed_for_self_loops =
        match specs.self_loops && specs.self_loops_false_strategy == SelfLoopsFalseStrategy::Drop {
            true => sorted_edges.collect::<Vec<Edge<T, K, V>>>(),
            false => sorted_edges
                .filter(|e| e.u != e.v)
                .into_iter()
                .collect::<Vec<Edge<T, K, V>>>(),
        };

    if processed_for_self_loops.len() < edges_len {
        return Err(Error {
            kind: ErrorKind::SelfLoopsFound,
            message: "edges contain self-loops and `specs.self_loops` is false".to_string(),
        });
    }

    let grouped = processed_for_self_loops
        .into_iter()
        .group_by(|e| (e.u, e.v))
        .into_iter()
        .map(|(k, g)| (k, g.collect::<Vec<Edge<T, K, V>>>()))
        .collect::<HashMap<(T, T), Vec<Edge<T, K, V>>>>();

    Ok(grouped)
}

fn get_directed_successors_predecessors<T, K, V>(
    edges: &Vec<&Edge<T, K, V>>,
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
    edges: &Vec<&Edge<T, K, V>>,
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
    (neighbors, HashMap::new())
}

fn get_deduped_edges<T, K, V>(
    edges: Vec<Edge<T, K, V>>,
    edge_dedupe_strategy: &EdgeDedupeStrategy,
    directed: &bool,
) -> Result<Vec<Edge<T, K, V>>, Error>
where
    T: Hash + Eq + Copy + Ord + Display,
    K: Hash + Eq + Copy,
    V: Copy,
{
    let mut hash_set = HashSet::<Edge<T, K, V>>::new();
    for edge in edges {
        let ordered = match directed {
            true => edge,
            false => edge.ordered(),
        };
        let existing_edge_option = hash_set.get(&ordered);
        if existing_edge_option.is_some() {
            match edge_dedupe_strategy {
                EdgeDedupeStrategy::Error => {
                    return Err(Error {
                        kind: ErrorKind::DuplicateEdge,
                        message: format!("duplicate edge found: {}", &ordered),
                    })
                }
                EdgeDedupeStrategy::KeepFirst => {}
                EdgeDedupeStrategy::KeepLast => {
                    hash_set.remove(&ordered);
                    hash_set.insert(ordered);
                }
            }
        } else {
            hash_set.insert(ordered);
        }
    }
    Ok(hash_set.into_iter().collect::<Vec<Edge<T, K, V>>>())
}
