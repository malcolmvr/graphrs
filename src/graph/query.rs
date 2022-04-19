use super::Graph;
use crate::{Edge, Error, ErrorKind, Node};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::hash::Hash;

impl<T, A> Graph<T, A>
where
    T: Eq + Clone + PartialOrd + Ord + Hash + Send + Sync + Display,
    A: Clone,
{
    /**
    Determines if all edges have a weight value.

    # Returns

    `true` if all edges have a `weight` value and the value isn't NAN, false otherwise.
    */
    pub fn edges_have_weight(&self) -> bool
    where
        T: Hash + Eq + Clone + Ord,
        A: Clone,
    {
        for edge in self.get_all_edges() {
            if edge.weight.is_nan() {
                return false;
            }
        }
        true
    }

    /**
    Gets a `Vec` of all the edges in the graph.

    # Examples

    ```
    use graphrs::{Edge, Graph, GraphSpecs};

    let mut graph: Graph<&str, ()> = Graph::new(GraphSpecs::directed_create_missing());
    let result = graph.add_edges(vec![
        Edge::new("n1", "n2"),
        Edge::new("n2", "n3"),
    ]);
    let all_edges = graph.get_all_edges();
    assert_eq!(all_edges.len(), 2);
    ```
    **/
    pub fn get_all_edges(&self) -> Vec<&Edge<T, A>>
    where
        T: Hash + Eq + Clone + Ord,
        A: Clone,
    {
        self.edges.values().into_iter().flatten().collect::<Vec<&Edge<T, A>>>()
    }

    /**
    Gets a `Vec` of all the nodes in the graph.

    # Examples

    ```
    use graphrs::{Node, Graph, GraphSpecs};

    let mut graph: Graph<&str, ()> = Graph::new(GraphSpecs::multi_undirected());
    graph.add_nodes(vec![
        Node::from_name("n1"),
        Node::from_name("n2"),
    ]);
    let all_nodes = graph.get_all_nodes();
    assert_eq!(all_nodes.len(), 2);
    ```
    */
    pub fn get_all_nodes(&self) -> Vec<&Node<T, A>>
    where
        T: Hash + Eq + Clone + Ord,
        A: Clone,
    {
        self.nodes.values().collect::<Vec<&Node<T, A>>>()
    }

    /**
    Gets a `Vec` of all the nodes in the graph.

    # Examples

    ```
    use graphrs::{Node, Graph, GraphSpecs};

    let mut graph: Graph<&str, ()> = Graph::new(GraphSpecs::multi_undirected());
    graph.add_nodes(vec![
        Node::from_name("n1"),
        Node::from_name("n2"),
    ]);
    let all_nodes = graph.get_all_node_names();
    assert_eq!(all_nodes.len(), 2);
    ```
    */
    pub fn get_all_node_names(&self) -> Vec<&T>
    where
        T: Hash + Eq + Clone + Ord,
        A: Clone,
    {
        self.nodes.keys().collect::<Vec<&T>>()
    }

    /**
    Gets the `Edge` between `u` and `v` nodes.

    If `specs.multi_edges` is true then the `get_edges` method should be used instead.

    # Arguments

    `u`: The name of the first node of the edge.
    `v`: The name of the second node of the edge.

    # Returns

    If no edge exists between `u` and `v`, `Err` is returned.

    # Examples

    ```
    use graphrs::{generators};
    let graph = generators::social::karate_club_graph();
    let edge = graph.get_edge(0, 1);
    assert!(edge.is_ok());
    ```
    */
    pub fn get_edge(&self, u: T, v: T) -> Result<&Edge<T, A>, Error>
    where
        T: Hash + Eq + Clone + Ord,
        A: Clone,
    {
        if self.specs.multi_edges {
            return Err(Error {
                kind: ErrorKind::WrongMethod,
                message: "Use the `get_edges` method when `GraphSpecs.multi_edges` is `true`."
                    .to_string(),
            });
        }

        let ordered = match !self.specs.directed && u > v {
            false => (u.clone(), v.clone()),
            true => (v.clone(), u.clone()),
        };

        let edges = self.edges.get(&ordered);
        match edges {
            None => Err(Error {
                kind: ErrorKind::EdgeNotFound,
                message: format!("The requested edge ({}, {}) does not exist.", u, v),
            }),
            Some(e) => Ok(&e[0]),
        }
    }

    /**
    Gets the edges between `u` and `v` nodes.

    If `specs.multi_edges` is false then the `get_edge` method should be used instead.

    # Arguments

    `u`: The name of the first node of the edge.
    `v`: The name of the second node of the edge.

    # Returns

    If no edge exists between `u` and `v`, `Err` is returned.

    # Examples

    ```
    use graphrs::{Edge, Graph, GraphSpecs, MissingNodeStrategy};

    let mut graph: Graph<&str, ()> = Graph::new(GraphSpecs {
        missing_node_strategy: MissingNodeStrategy::Create,
        ..GraphSpecs::multi_undirected()
    });
    let result = graph.add_edges(vec![
        Edge::new("n1", "n2"),
        Edge::new("n2", "n1"),
    ]);
    let edges = graph.get_edges("n1", "n2");
    assert_eq!(edges.unwrap().len(), 2);
    ```
    */
    pub fn get_edges(&self, u: T, v: T) -> Result<Vec<&Edge<T, A>>, Error>
    where
        T: Hash + Eq + Clone + Ord,
        A: Clone,
    {
        if !self.specs.multi_edges {
            return Err(Error {
                kind: ErrorKind::WrongMethod,
                message: "Use the `get_edge` method when `multi_edges` is `false`".to_string(),
            });
        }

        let ordered = match !self.specs.directed && u > v {
            false => (u.clone(), v.clone()),
            true => (v.clone(), u.clone()),
        };

        let edges = self.edges.get(&ordered);
        match edges {
            None => Err(Error {
                kind: ErrorKind::EdgeNotFound,
                message: format!("No edges found for the requested ({}, {})", u, v),
            }),
            Some(e) => Ok(e.iter().collect::<Vec<&Edge<T, A>>>()),
        }
    }

    /**
    Returns all edges that connect to a specified node.

    # Arguments

    * `name`: the node to get all adjacent edges for

    # Examples

    ```
    use graphrs::{Edge, Graph, GraphSpecs};

    let mut graph: Graph<&str, ()> = Graph::new(GraphSpecs::undirected_create_missing());
    let result = graph.add_edges(vec![
        Edge::new("n1", "n2"),
        Edge::new("n3", "n2"),
    ]);
    let n2_edges = graph.get_edges_for_node("n2").unwrap();
    assert_eq!(n2_edges.len(), 2);
    ```
    */
    pub fn get_edges_for_node(&self, name: T) -> Result<Vec<&Edge<T, A>>, Error>
    where
        T: Hash + Eq + Clone + Ord,
        A: Clone,
    {
        if self.get_node(name.clone()).is_none() {
            return Err(Error {
                kind: ErrorKind::NodeNotFound,
                message: format!("Requested node '{}' was not found in the graph.", name),
            });
        }
        let empty_set = HashSet::new();
        let pred_node_names = self.predecessors.get(&name.clone()).unwrap_or(&empty_set);
        let succ_node_names = self.successors.get(&name.clone()).unwrap_or(&empty_set);
        let pred_edges = pred_node_names
            .iter()
            .flat_map(|pnn| self.edges.get(&(pnn.clone(), name.clone())).unwrap());
        let succ_edges: Vec<&Edge<T, A>> = succ_node_names
            .iter()
            .flat_map(|snn| {
                let ordered = match !self.specs.directed && name > snn.clone() {
                    false => (name.clone(), snn.clone()),
                    true => (snn.clone(), name.clone()),
                };
                self.edges.get(&ordered).unwrap()
            })
            .collect();
        Ok(pred_edges.into_iter().chain(succ_edges).collect())
    }

    /**
    Returns all edges that connect to any node in a `Vec` of nodes.

    # Arguments

    * `names`: the nodes to get all edges for

    # Examples

    ```
    use graphrs::{Edge, Graph, GraphSpecs};

    let mut graph: Graph<&str, ()> = Graph::new(GraphSpecs::undirected_create_missing());
    let result = graph.add_edges(vec![
        Edge::new("n1", "n3"),
        Edge::new("n2", "n4"),
        Edge::new("n3", "n4"),
    ]);
    let n2_edges = graph.get_edges_for_nodes(&["n1", "n2"]).unwrap();
    assert_eq!(n2_edges.len(), 2);
    ```
    */
    pub fn get_edges_for_nodes(&self, names: &[T]) -> Result<Vec<&Edge<T, A>>, Error>
    where
        T: Hash + Eq + Clone + Ord,
        A: Clone,
    {
        if !self.has_nodes(names) {
            return Err(Error {
                kind: ErrorKind::NodeNotFound,
                message: "One or more of the specified found nodes were not found in the graph."
                    .to_string(),
            });
        }
        let names_set: HashSet<&T> = names.iter().collect();
        Ok(self
            .get_all_edges()
            .into_iter()
            .filter(|e| names_set.contains(&e.u) || names_set.contains(&e.v))
            .collect())
    }

    /**
    Returns all edges (u, v) where v is `name`.
    Returns an `Error` if `graph.specs.directed` is `false`; use the `get_edges_for_node`
    for an undirected graph.

    # Arguments

    * `name`: the node to get all in-edges for

    # Examples

    ```
    use graphrs::{Edge, Graph, GraphSpecs};

    let mut graph: Graph<&str, ()> = Graph::new(GraphSpecs::directed_create_missing());
    let result = graph.add_edges(vec![
        Edge::new("n1", "n2"),
        Edge::new("n3", "n2"),
    ]);
    let n2_in_edges = graph.get_in_edges_for_node("n2").unwrap();
    assert_eq!(n2_in_edges.len(), 2);
    ```
    */
    pub fn get_in_edges_for_node(&self, name: T) -> Result<Vec<&Edge<T, A>>, Error>
    where
        T: Hash + Eq + Clone + Ord,
        A: Clone,
    {
        if !self.specs.directed {
            return Err(Error {
                kind: ErrorKind::WrongMethod,
                message: "Use the `get_edges_for_node` method when `directed` is `false`"
                    .to_string(),
            });
        }
        if self.get_node(name.clone()).is_none() {
            return Err(Error {
                kind: ErrorKind::NodeNotFound,
                message: format!("Requested node '{}' was not found in the graph.", name),
            });
        }
        let empty = HashSet::new();
        let pred_node_names = self.predecessors.get(&name).unwrap_or(&empty);
        Ok(pred_node_names
            .iter()
            .flat_map(|pnn| self.edges.get(&(pnn.clone(), name.clone())).unwrap())
            .collect())
    }

    /**
    Returns all edges that connect into to any node in a `Vec` of nodes.

    # Arguments

    * `names`: the nodes to get all in-edges for

    # Examples

    ```
    use graphrs::{Edge, Graph, GraphSpecs};

    let mut graph: Graph<&str, ()> = Graph::new(GraphSpecs::directed_create_missing());
    let result = graph.add_edges(vec![
        Edge::new("n1", "n2"),
        Edge::new("n2", "n1"),
        Edge::new("n2", "n3"),
    ]);
    let n2_in_edges = graph.get_in_edges_for_nodes(&["n1", "n2"]).unwrap();
    assert_eq!(n2_in_edges.len(), 2);
    ```
    */
    pub fn get_in_edges_for_nodes(&self, names: &[T]) -> Result<Vec<&Edge<T, A>>, Error>
    where
        T: Hash + Eq + Clone + Ord,
        A: Clone,
    {
        if !self.specs.directed {
            return Err(Error {
                kind: ErrorKind::WrongMethod,
                message: "Use the `get_edges_for_nodes` method when `directed` is `false`"
                    .to_string(),
            });
        }
        if !self.has_nodes(names) {
            return Err(Error {
                kind: ErrorKind::NodeNotFound,
                message: "One or more of the specified found nodes were not found in the graph."
                    .to_string(),
            });
        }
        let names_set: HashSet<&T> = names.iter().collect();
        Ok(self.get_all_edges().into_iter().filter(|e| names_set.contains(&e.v)).collect())
    }

    /**
    Returns all edges (u, v) where u is `name`.
    Returns an `Error` if `graph.specs.directed` is `false`; use the `get_edges_for_node`
    for an undirected graph.

    # Arguments

    * `name`: the node to get all out-edges for

    # Examples

    ```
    use graphrs::{Edge, Graph, GraphSpecs};

    let mut graph: Graph<&str, ()> = Graph::new(GraphSpecs::directed_create_missing());
    let result = graph.add_edges(vec![
        Edge::new("n2", "n1"),
        Edge::new("n2", "n3"),
    ]);
    let n2_out_edges = graph.get_out_edges_for_node("n2").unwrap();
    assert_eq!(n2_out_edges.len(), 2);
    ```
    */
    pub fn get_out_edges_for_node(&self, name: T) -> Result<Vec<&Edge<T, A>>, Error>
    where
        T: Hash + Eq + Clone + Ord,
        A: Clone,
    {
        if !self.specs.directed {
            return Err(Error {
                kind: ErrorKind::WrongMethod,
                message: "Use the `get_edges_for_node` method when `directed` is `false`"
                    .to_string(),
            });
        }
        if self.get_node(name.clone()).is_none() {
            return Err(Error {
                kind: ErrorKind::NodeNotFound,
                message: format!("Requested node '{}' was not found in the graph.", name),
            });
        }
        let empty = HashSet::new();
        let succ_node_names = self.successors.get(&name).unwrap_or(&empty);
        Ok(succ_node_names
            .iter()
            .flat_map(|snn| self.edges.get(&(name.clone(), snn.clone())).unwrap())
            .collect())
    }

    /**
    Returns all edges that come out of any node in a `Vec` of nodes.

    # Arguments

    * `names`: the nodes to get all out-edges for

    # Examples

    ```
    use graphrs::{Edge, Graph, GraphSpecs};

    let mut graph: Graph<&str, ()> = Graph::new(GraphSpecs::directed_create_missing());
    let result = graph.add_edges(vec![
        Edge::new("n1", "n2"),
        Edge::new("n2", "n3"),
        Edge::new("n3", "n2"),
    ]);
    let n1_out_edges = graph.get_out_edges_for_nodes(&["n2", "n3"]).unwrap();
    assert_eq!(n1_out_edges.len(), 2);
    ```
    */
    pub fn get_out_edges_for_nodes(&self, names: &[T]) -> Result<Vec<&Edge<T, A>>, Error>
    where
        T: Hash + Eq + Clone + Ord,
        A: Clone,
    {
        if !self.specs.directed {
            return Err(Error {
                kind: ErrorKind::WrongMethod,
                message: "Use the `get_edges_for_nodes` method when `directed` is `false`"
                    .to_string(),
            });
        }
        if !self.has_nodes(names) {
            return Err(Error {
                kind: ErrorKind::NodeNotFound,
                message: "One or more of the specified found nodes were not found in the graph."
                    .to_string(),
            });
        }
        let names_set: HashSet<&T> = names.iter().collect();
        Ok(self.get_all_edges().into_iter().filter(|e| names_set.contains(&e.u)).collect())
    }

    /**
    Returns all the nodes that connect to `node_name`.

    # Arguments

    * `node_name`: The name of the node to find neighbors for.

    # Returns

    For an undirected graph adjacent nodes are returned.
    For a directed graph predecessor and successor nodes are returned.

    # Examples

    ```
    use graphrs::{Edge, Graph, GraphSpecs};

    let mut graph: Graph<&str, ()> = Graph::new(GraphSpecs::directed_create_missing());
    let result = graph.add_edges(vec![
        Edge::new("n1", "n2"),
        Edge::new("n2", "n3"),
    ]);
    assert!(result.is_ok());
    let neighbors = graph.get_neighbor_nodes("n2");
    assert_eq!(neighbors.unwrap().len(), 2);
    ```
    */
    pub fn get_neighbor_nodes(&self, node_name: T) -> Result<Vec<&Node<T, A>>, Error>
    where
        T: Hash + Eq + Clone + Ord,
        A: Clone,
    {
        if !self.nodes.contains_key(&node_name.clone()) {
            return Err(Error {
                kind: ErrorKind::NodeNotFound,
                message: format!("Requested node '{}' was not found in the graph.", node_name),
            });
        }

        let pred_nodes = self._get_predecessor_nodes(node_name.clone()).unwrap();
        let succ_nodes = self._get_successor_nodes(node_name).unwrap();

        let all_nodes = pred_nodes
            .into_iter()
            .chain(succ_nodes)
            .sorted_by(|a, b| Ord::cmp(&a, &b))
            .dedup_by(|a, b| a == b)
            .collect();

        Ok(all_nodes)
    }

    /**
    Gets the `Node` for the specified node `name`.

    # Arguments

    * `name`: The name of the [Node](./struct.Node.html) to return.

    # Examples

    ```
    use graphrs::{Node, Graph, GraphSpecs};

    let mut graph: Graph<&str, i32> = Graph::new(GraphSpecs::directed());
    graph.add_node(Node::from_name_and_attributes("n1", 99));
    let node = graph.get_node("n1");
    assert_eq!(node.unwrap().attributes.unwrap(), 99);
    ```
    */
    pub fn get_node(&self, name: T) -> Option<&Node<T, A>>
    where
        T: Hash + Eq + Clone + Ord,
        A: Clone,
    {
        self.nodes.get(&name)
    }

    /**
    Gets all u for (u, v) edges where `node_name` is v.

    # Arguments

    * `name`: The name of the [Node](./struct.Node.html) to return predecessors for.

    # Returns

    Returns an error if called on an undirected graph. Use `get_neighbor_nodes` for
    undirected graphs.

    # Examples

    ```
    use graphrs::{Edge, Graph, GraphSpecs};

    let mut graph: Graph<&str, ()> = Graph::new(GraphSpecs::directed_create_missing());
    let result = graph.add_edges(vec![
        Edge::new("n1", "n3"),
        Edge::new("n2", "n3"),
    ]);
    assert!(result.is_ok());
    let predecessors = graph.get_predecessor_nodes("n3");
    assert_eq!(predecessors.unwrap().len(), 2);
    ```
    */
    pub fn get_predecessor_nodes(&self, node_name: T) -> Result<Vec<&Node<T, A>>, Error>
    where
        T: Hash + Eq + Clone + Ord,
        A: Clone,
    {
        if !self.specs.directed {
            return Err(Error {
                kind: ErrorKind::WrongMethod,
                message: "For undirected graphs use the `get_neighbor_nodes` method instead \
                of `get_predecessor_nodes`"
                    .to_string(),
            });
        }

        self._get_predecessor_nodes(node_name)
    }

    /**
    Gets all the names of u for (u, v) edges where `node_name` is v.

    # Arguments

    * `name`: The name of the [Node](./struct.Node.html) to return predecessors for.

    # Returns

    Returns an error if called on an undirected graph. Use `get_neighbor_nodes` for
    undirected graphs.

    # Examples

    ```
    use graphrs::{Edge, Graph, GraphSpecs};

    let mut graph: Graph<&str, ()> = Graph::new(GraphSpecs::directed_create_missing());
    let result = graph.add_edges(vec![
        Edge::new("n1", "n3"),
        Edge::new("n2", "n3"),
    ]);
    assert!(result.is_ok());
    let predecessor_names = graph.get_predecessor_node_names("n3");
    assert_eq!(predecessor_names.unwrap().len(), 2);
    ```
    */
    pub fn get_predecessor_node_names(&self, node_name: T) -> Result<Vec<&T>, Error>
    where
        T: Hash + Eq + Clone + Ord,
        A: Clone,
    {
        let nodes = self.get_predecessor_nodes(node_name)?;
        Ok(nodes.into_iter().map(|n| &n.name).collect())
    }

    fn _get_predecessor_nodes(&self, node_name: T) -> Result<Vec<&Node<T, A>>, Error>
    where
        T: Hash + Eq + Clone + Ord,
        A: Clone,
    {
        if !self.nodes.contains_key(&node_name) {
            return Err(Error {
                kind: ErrorKind::NodeNotFound,
                message: format!("Requested node '{}' was not found in the graph", node_name),
            });
        }
        let pred = self.predecessors.get(&node_name);
        match pred {
            None => Ok(vec![]),
            Some(hashset) => Ok(self.get_nodes_for_names(hashset)),
        }
    }

    /// Gets a `HashMap` of all the predecessor edges.
    pub fn get_predecessors_map(&self) -> &HashMap<T, HashSet<T>>
    where
        T: Hash + Eq + Clone + Ord,
        A: Clone,
    {
        &self.predecessors
    }

    /**
    Gets all v for (u, v) edges where `node_name` is u.

    # Arguments

    * `name`: The name of the [Node](./struct.Node.html) to return predecessors for.

    # Returns

    Returns an error if called on an undirected graph. Use `get_neighbor_nodes` for
    undirected graphs.

    # Examples

    ```
    use graphrs::{Edge, Graph, GraphSpecs};

    let mut graph: Graph<&str, ()> = Graph::new(GraphSpecs::directed_create_missing());
    let result = graph.add_edges(vec![
        Edge::new("n1", "n2"),
        Edge::new("n1", "n3"),
    ]);
    assert!(result.is_ok());
    let successors = graph.get_successor_nodes("n1");
    assert_eq!(successors.unwrap().len(), 2);
    ```
    */
    pub fn get_successor_nodes(&self, node_name: T) -> Result<Vec<&Node<T, A>>, Error>
    where
        T: Hash + Eq + Clone + Ord,
        A: Clone,
    {
        if !self.specs.directed {
            return Err(Error {
                kind: ErrorKind::WrongMethod,
                message: "For undirected graphs use the `get_neighbor_nodes` method instead \
                of `get_successor_nodes`"
                    .to_string(),
            });
        }

        self._get_successor_nodes(node_name)
    }

    /**
    Gets all the names of v for (u, v) edges where `node_name` is u.

    # Arguments

    * `name`: The name of the [Node](./struct.Node.html) to return successors for.

    # Returns

    Returns an error if called on an undirected graph. Use `get_neighbor_nodes` for
    undirected graphs.

    # Examples

    ```
    use graphrs::{Edge, Graph, GraphSpecs};

    let mut graph: Graph<&str, ()> = Graph::new(GraphSpecs::directed_create_missing());
    let result = graph.add_edges(vec![
        Edge::new("n1", "n2"),
        Edge::new("n1", "n3"),
    ]);
    assert!(result.is_ok());
    let successor_names = graph.get_successor_node_names("n1");
    assert_eq!(successor_names.unwrap().len(), 2);
    ```
    */
    pub fn get_successor_node_names(&self, node_name: T) -> Result<Vec<&T>, Error>
    where
        T: Hash + Eq + Clone + Ord,
        A: Clone,
    {
        let nodes = self.get_successor_nodes(node_name)?;
        Ok(nodes.into_iter().map(|n| &n.name).collect())
    }

    fn _get_successor_nodes(&self, node_name: T) -> Result<Vec<&Node<T, A>>, Error>
    where
        T: Hash + Eq + Clone + Ord,
        A: Clone,
    {
        if !self.nodes.contains_key(&node_name) {
            return Err(Error {
                kind: ErrorKind::NodeNotFound,
                message: format!("Requested node '{}' was not found in the graph.", node_name),
            });
        }
        let succ = self.successors.get(&node_name);
        match succ {
            None => Ok(vec![]),
            Some(hashset) => Ok(self.get_nodes_for_names(hashset)),
        }
    }

    /// Gets a `HashMap` of all the successor edges.
    pub fn get_successors_map(&self) -> &HashMap<T, HashSet<T>>
    where
        T: Hash + Eq + Clone + Ord,
        A: Clone,
    {
        &self.successors
    }

    /**
    Returns successors of a node if the `graph` is directed.

    Returns neighbors of a node if the `graph` is undirected.
    */
    pub fn get_successors_or_neighbors(&self, node_name: T) -> Vec<&Node<T, A>>
    where
        T: Hash + Eq + Clone + Ord + Display + Send + Sync,
        A: Clone,
    {
        match self.specs.directed {
            true => self.get_successor_nodes(node_name).unwrap(),
            false => self.get_neighbor_nodes(node_name).unwrap(),
        }
    }

    /**
    Returns `true` if the graph contains a given node, `false` otherwise.

    # Arguments

    * `node_name`: the name of the node to query for

    # Examples

    ```
    use graphrs::{Graph, GraphSpecs, Node};
    let mut graph: Graph<&str, ()> = Graph::new(GraphSpecs::directed_create_missing());
    graph.add_node(Node::from_name("n1"));
    assert!(graph.has_node(&"n1"));
    ```
    */
    pub fn has_node(&self, node_name: &T) -> bool {
        self.get_node(node_name.clone()).is_some()
    }

    /**
    Returns `true` if the graph contains all given nodes, `false` otherwise.

    # Arguments

    * `node_names`: the names of the nodes to query for

    # Examples

    ```
    use graphrs::{Graph, GraphSpecs, Node};
    let mut graph: Graph<&str, ()> = Graph::new(GraphSpecs::directed_create_missing());
    graph.add_node(Node::from_name("n1"));
    assert!(!graph.has_nodes(&vec!["n1", "n2"]));
    ```
    */
    pub fn has_nodes(&self, node_names: &[T]) -> bool {
        for node_name in node_names {
            if !self.has_node(node_name) {
                return false;
            }
        }
        true
    }

    /**
    Returns the number of edges or sum of all edge weights.

    # Arguments

    * `weighted`: if `true` returns the sum of all edge weights, if `false` the number of edges

    # Examples

    ```
    use graphrs::{generators};
    let graph = generators::social::karate_club_graph();
    assert_eq!(graph.size(false), 78.0);
    ```
    */
    pub fn size(&self, weighted: bool) -> f64 {
        match weighted {
            false => self.get_all_edges().len() as f64,
            true => self.get_all_edges().iter().map(|e| e.weight).sum(),
        }
    }

    // PRIVATE METHODS

    fn get_nodes_for_names(&self, names: &HashSet<T>) -> Vec<&Node<T, A>>
    where
        T: Hash + Eq + Clone + Ord,
        A: Clone,
    {
        names.iter().map(|n| self.nodes.get(n).unwrap()).collect::<Vec<&Node<T, A>>>()
    }
}
