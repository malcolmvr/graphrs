use crate::{
    Edge, EdgeDedupeStrategy, Error, ErrorKind, GraphSpecs, MissingNodeStrategy, Node,
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

A `Graph` has two generic arguments:
* `T`: Specifies the type to use for node names.
* `A`: Specifies the type to use for node and edge attributes. Attributes are *optional*
extra data that are associated with a node or an edge. For example, if nodes represent
people and `T` is an `i32` of their employee ID then the node attributes might store
their first and last names.

# Example

```
use graphrs::{Edge, Graph, GraphSpecs, Node};

let nodes = vec![
    Node::from_name("n1"),
    Node::from_name("n2"),
    Node::from_name("n3"),
];

let edges = vec![
    Edge::with_weight("n1", "n2", 1.0),
    Edge::with_weight("n2", "n1", 2.0),
    Edge::with_weight("n1", "n3", 3.0),
    Edge::with_weight("n2", "n3", 3.0),
];

let specs = GraphSpecs::directed();

let graph = Graph::<&str, ()>::new_from_nodes_and_edges(
    nodes,
    edges,
    specs
);
```
*/
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Graph<T: PartialOrd + Send, A: Copy> {
    /// The graph's nodes, stored as a `HashMap` keyed by the node names.
    nodes: HashMap<T, Node<T, A>>,
    /// The graph's edges, stored as a `HashMap` keyed by a tuple of node names.
    edges: HashMap<(T, T), Vec<Edge<T, A>>>,
    /// The [GraphSpecs](./struct.GraphSpecs.html) for the graph.
    pub specs: GraphSpecs,
    /// Stores the successors of nodes. A successor of u is a node v such that there
    /// exists a directed edge from u to v. For an undirected graph `successors` stores
    /// all the adjacent nodes. An adjacent node to u is a node v such that there exists
    /// an edge from u to v *or* from v to u.
    successors: HashMap<T, HashSet<T>>,
    /// Stores the predecessors of nodes. A predecessor of v is a node u such that there
    /// exists a directed edge from u to v. For an undirected graph `precessors` is not used.
    predecessors: HashMap<T, HashSet<T>>,
}

impl<T, A> Graph<T, A>
where
    T: Eq + Copy + PartialOrd + Ord + Hash + Send + Sync + Display,
    A: Copy,
{
    /**
    Adds an `edge` to the `Graph`.

    If the new edge references nodes that don't exist the graph's `specs.missing_node_strategy`
    determines what happens.

    ```
    use graphrs::{Edge, Graph, GraphSpecs};

    let mut graph: Graph<&str, ()> = Graph::new(GraphSpecs::directed_create_missing());
    let result = graph.add_edge(Edge::new("n1", "n2"));
    assert!(result.is_ok());
    ```
    */
    pub fn add_edge(&mut self, edge: Edge<T, A>) -> Result<(), Error>
    where
        T: Hash + Eq + Copy + Ord + Display,
        A: Copy,
    {
        // check for self loops
        if !self.specs.self_loops && edge.u == edge.v {
            match self.specs.self_loops_false_strategy {
                SelfLoopsFalseStrategy::Error => {
                    return Err(Error {
                        kind: ErrorKind::SelfLoopsFound,
                        message: format!(
                            "Edge ({}, {}) is a self-loop and `specs.self_loops` is false.",
                            edge.u, edge.v
                        ),
                    });
                }
                SelfLoopsFalseStrategy::Drop => {
                    return Ok(());
                }
            }
        }

        // add nodes
        if self.specs.missing_node_strategy == MissingNodeStrategy::Error
            && (!self.nodes.contains_key(&edge.u) || !self.nodes.contains_key(&edge.v))
        {
            return Err(Error {
                kind: ErrorKind::NodeNotFound,
                message: format!(
                    "While adding edge ({}, {}) one or both of the nodes was not \
                    found in the graph. Either add the nodes or set \
                    GraphSpecs.missing_node_strategy to `Create`.",
                    edge.u, edge.v
                ),
            });
        }
        self.nodes
            .entry(edge.u)
            .or_insert_with(|| Node::from_name(edge.u));
        self.nodes
            .entry(edge.v)
            .or_insert_with(|| Node::from_name(edge.v));

        // add successors and predecessors
        self.successors.entry(edge.u).or_default().insert(edge.v);
        match self.specs.directed {
            true => {
                self.predecessors.entry(edge.v).or_default().insert(edge.u);
            }
            false => {
                self.successors.entry(edge.v).or_default().insert(edge.u);
            }
        }

        let ordered = match self.specs.directed {
            false => edge.ordered(),
            true => edge,
        };

        // add edge
        match self.specs.multi_edges {
            true => {
                self.edges
                    .entry((ordered.u, ordered.v))
                    .or_default()
                    .push(ordered);
            }
            false => match self.get_edge(ordered.u, ordered.v).is_ok() {
                false => {
                    self.edges.insert((ordered.u, ordered.v), vec![ordered]);
                }
                true => match self.specs.edge_dedupe_strategy {
                    EdgeDedupeStrategy::Error => {
                        return Err(Error {
                            kind: ErrorKind::DuplicateEdge,
                            message: format!(
                                "A duplicate edge was found: {}. \
                                Set the `GraphSpecs.edge_dedupe_strategy` if a different
                                behavior is desired.",
                                ordered
                            ),
                        });
                    }
                    EdgeDedupeStrategy::KeepLast => {
                        self.edges.insert((ordered.u, ordered.v), vec![ordered]);
                    }
                    _ => {}
                },
            },
        }

        Ok(())
    }

    /**
    Adds an edge, as a (u, v) tuple, to the `Graph`.

    If the new edge references nodes that don't exist the graph's `specs.missing_node_strategy`
    determines what happens.

    ```
    use graphrs::{Edge, Graph, GraphSpecs};

    let mut graph: Graph<&str, ()> = Graph::new(GraphSpecs::directed_create_missing());
    let result = graph.add_edge_tuple("n1", "n2");
    assert!(result.is_ok());
    ```
    */
    pub fn add_edge_tuple(&mut self, u: T, v: T) -> Result<(), Error>
    where
        T: Hash + Eq + Copy + Ord + Display,
    {
        self.add_edge(Edge::new(u, v))
    }

    /**
    Adds new edges to a `Graph`, or updates existing edges, or both.

    If the new edges reference nodes that don't exist the graph's `specs.missing_node_strategy`
    determines what happens.

    # Arguments

    * `edges`: the new edges to add to the graph

    ```
    use graphrs::{Edge, Graph, GraphSpecs};

    let mut graph: Graph<&str, ()> = Graph::new(GraphSpecs::directed_create_missing());
    let result = graph.add_edges(vec![
        Edge::new("n1", "n2"),
        Edge::new("n2", "n3"),
    ]);
    assert!(result.is_ok());
    ```
    */
    pub fn add_edges(&mut self, edges: Vec<Edge<T, A>>) -> Result<(), Error>
    where
        T: Hash + Eq + Copy + Ord + Display,
        A: Copy,
    {
        for edge in edges {
            let result = self.add_edge(edge);
            if result.is_err() {
                return result;
            }
        }

        Ok(())
    }

    /**
    Adds new edges to a `Graph`, or updates existing edges, or both.

    If the new edges reference nodes that don't exist the graph's `specs.missing_node_strategy`
    determines what happens.

    # Arguments

    * `edges`: the new edges to add to the graph

    ```
    use graphrs::{Edge, Graph, GraphSpecs};

    let mut graph: Graph<&str, ()> = Graph::new(GraphSpecs::directed_create_missing());
    let result = graph.add_edge_tuples(vec![
        ("n1", "n2"),
        ("n2", "n3"),
    ]);
    assert!(result.is_ok());
    ```
    */
    pub fn add_edge_tuples(&mut self, edges: Vec<(T, T)>) -> Result<(), Error>
    where
        T: Hash + Eq + Copy + Ord + Display,
        A: Copy,
    {
        for edge in edges {
            let result = self.add_edge(Edge::new(edge.0, edge.1));
            if result.is_err() {
                return result;
            }
        }

        Ok(())
    }

    /**
    Adds a node to the graph or updates the node's attributes if the node already exists.

    # Arguments

    `node`: the new node to add to the graph

    ```
    use graphrs::{Node, Graph, GraphSpecs};

    let mut graph: Graph<&str, ()> = Graph::new(GraphSpecs::multi_directed());
    graph.add_node(Node::from_name("n1"));
    ```
    */
    pub fn add_node(&mut self, node: Node<T, A>)
    where
        T: Hash + Eq + Copy + Ord,
        A: Copy,
    {
        self.nodes.insert(node.name, node);
    }

    /**
    Adds a nodes to the graph or updates the nodes' attributes if they already exist.

    # Arguments

    `nodes`: the new nodes to add to the graph

    ```
    use graphrs::{Node, Graph, GraphSpecs};

    let mut graph: Graph<&str, ()> = Graph::new(GraphSpecs::multi_directed());
    graph.add_nodes(vec![
        Node::from_name("n1"),
        Node::from_name("n2"),
    ]);
    ```
    */
    pub fn add_nodes(&mut self, nodes: Vec<Node<T, A>>)
    where
        T: Hash + Eq + Copy + Ord,
        A: Copy,
    {
        for node in nodes {
            self.add_node(node);
        }
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
        T: Hash + Eq + Copy + Ord,
        A: Copy,
    {
        self.edges
            .values()
            .into_iter()
            .flatten()
            .collect::<Vec<&Edge<T, A>>>()
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
        T: Hash + Eq + Copy + Ord,
        A: Copy,
    {
        self.nodes.values().collect::<Vec<&Node<T, A>>>()
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
        T: Hash + Eq + Copy + Ord,
        A: Copy,
    {
        if self.specs.multi_edges {
            return Err(Error {
                kind: ErrorKind::WrongMethod,
                message: "Use the `get_edges` method when `GraphSpecs.multi_edges` is `true`."
                    .to_string(),
            });
        }

        let ordered = match !self.specs.directed && u > v {
            false => (u, v),
            true => (v, u),
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
        T: Hash + Eq + Copy + Ord,
        A: Copy,
    {
        if !self.specs.multi_edges {
            return Err(Error {
                kind: ErrorKind::WrongMethod,
                message: "Use the `get_edge` method when `multi_edges` is `false`".to_string(),
            });
        }

        let ordered = match !self.specs.directed && u > v {
            false => (u, v),
            true => (v, u),
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
        T: Hash + Eq + Copy + Ord,
        A: Copy,
    {
        if !self.nodes.contains_key(&node_name) {
            return Err(Error {
                kind: ErrorKind::NodeNotFound,
                message: format!("Requested node '{}' was not found in the graph.", node_name),
            });
        }

        let pred_nodes = self._get_predecessor_nodes(node_name).unwrap();
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
        T: Hash + Eq + Copy + Ord,
        A: Copy,
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
        T: Hash + Eq + Copy + Ord,
        A: Copy,
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

    fn _get_predecessor_nodes(&self, node_name: T) -> Result<Vec<&Node<T, A>>, Error>
    where
        T: Hash + Eq + Copy + Ord,
        A: Copy,
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
        T: Hash + Eq + Copy + Ord,
        A: Copy,
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
        Edge::new("n1", "n3"),
        Edge::new("n2", "n3"),
    ]);
    assert!(result.is_ok());
    let successors = graph.get_predecessor_nodes("n3");
    assert_eq!(successors.unwrap().len(), 2);
    ```
    */
    pub fn get_successor_nodes(&self, node_name: T) -> Result<Vec<&Node<T, A>>, Error>
    where
        T: Hash + Eq + Copy + Ord,
        A: Copy,
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

    fn _get_successor_nodes(&self, node_name: T) -> Result<Vec<&Node<T, A>>, Error>
    where
        T: Hash + Eq + Copy + Ord,
        A: Copy,
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
        T: Hash + Eq + Copy + Ord,
        A: Copy,
    {
        &self.successors
    }

    /**
    Determines if all edges have a weight value.

    # Returns

    `true` if all edges have a `weight` value and the value isn't NAN, false otherwise.
    */
    pub fn edges_have_weight(&self) -> bool
    where
        T: Hash + Eq + Copy + Ord,
        A: Copy,
    {
        for edge in self.get_all_edges() {
            if edge.weight.is_nan() {
                return false;
            }
        }
        true
    }

    /**
    Creates an empty graph, according to the `specs`.

    # Arguments

    * `specs`: An instance of [GraphSpecs](./struct.GraphSpecs.html) that determines the
    characteristics and constraints of the graph.

    # Examples

    ```
    use graphrs::{Graph, GraphSpecs};
    let mut graph: Graph<&str, ()> = Graph::new(GraphSpecs::directed_create_missing());
    ```
    */
    pub fn new(specs: GraphSpecs) -> Graph<T, A> {
        Graph {
            nodes: HashMap::<T, Node<T, A>>::new(),
            edges: HashMap::<(T, T), Vec<Edge<T, A>>>::new(),
            specs,
            successors: HashMap::<T, HashSet<T>>::new(),
            predecessors: HashMap::<T, HashSet<T>>::new(),
        }
    }

    /**
    Create a new `Graph` from the specified `nodes` and `edges`.

    # Arguments

    * `nodes`: The [Node](./struct.Node.html) objects to add to the graph.
    * `edge`: The [Edge](./struct.Edge.html) objects to add to the graph.
    * `specs`: An instance of [GraphSpecs](./struct.GraphSpecs.html) that determines the
    characteristics and constraints of the graph.

    # Examples

    ```
    use graphrs::{Edge, Graph, GraphSpecs, Node};

    let nodes = vec![
        Node::from_name("n1"),
        Node::from_name("n2"),
        Node::from_name("n3"),
    ];

    let edges = vec![
        Edge::with_weight("n1", "n2", 1.0),
        Edge::with_weight("n2", "n1", 2.0),
        Edge::with_weight("n1", "n3", 3.0),
        Edge::with_weight("n2", "n3", 3.0),
    ];

    let specs = GraphSpecs::directed();

    let graph = Graph::<&str, ()>::new_from_nodes_and_edges(
        nodes,
        edges,
        specs
    );
    ```
    */
    pub fn new_from_nodes_and_edges(
        nodes: Vec<Node<T, A>>,
        edges: Vec<Edge<T, A>>,
        specs: GraphSpecs,
    ) -> Result<Graph<T, A>, Error>
    where
        T: Hash + Eq + Copy + Ord,
        A: Copy,
    {
        let mut graph = Graph::new(specs);
        graph.add_nodes(nodes);
        let result = graph.add_edges(edges);
        match result {
            Err(e) => Err(e),
            Ok(_) => Ok(graph),
        }
    }

    // PRIVATE METHODS

    fn get_nodes_for_names(&self, names: &HashSet<T>) -> Vec<&Node<T, A>>
    where
        T: Hash + Eq + Copy + Ord,
        A: Copy,
    {
        names
            .iter()
            .map(|n| self.nodes.get(n).unwrap())
            .collect::<Vec<&Node<T, A>>>()
    }
}
