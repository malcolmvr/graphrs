use nohash::BuildNoHashHasher;

use super::Graph;
use crate::{
    Edge, EdgeDedupeStrategy, EdgeIndex, Error, ErrorKind, GraphSpecs, MissingNodeStrategy, Node,
    SelfLoopsFalseStrategy,
};
use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::hash::Hash;
use std::sync::Arc;

impl<T, A> Graph<T, A>
where
    T: Eq + Clone + PartialOrd + Ord + Hash + Send + Sync + Display,
    A: Clone,
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
    pub fn add_edge(&mut self, edge: Arc<Edge<T, A>>) -> Result<(), Error>
    where
        T: Hash + Eq + Clone + Ord + Display,
        A: Clone,
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

        // check for missing nodes
        if self.specs.missing_node_strategy == MissingNodeStrategy::Error
            && (!self.nodes_map.contains_key(&edge.u) || !self.nodes_map.contains_key(&edge.v))
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

        // check for duplicate edges
        if self.specs.edge_dedupe_strategy == EdgeDedupeStrategy::Error
            && self.get_edge(edge.u.clone(), edge.v.clone()).is_ok()
        {
            return Err(Error {
                kind: ErrorKind::DuplicateEdge,
                message: format!(
                    "A duplicate edge was found: {}. \
                    Set the `GraphSpecs.edge_dedupe_strategy` if a different
                    behavior is desired.",
                    edge
                ),
            });
        }

        // add or insert nodes
        if !self.nodes_map.contains_key(&edge.u) {
            self.add_node(Node::from_name(edge.u.clone()).into());
        }
        if !self.nodes_map.contains_key(&edge.v) {
            self.add_node(Node::from_name(edge.v.clone()).into());
        }

        // add successors and predecessors
        let u_node_index = *self.nodes_map.get(&edge.u).unwrap();
        let v_node_index = *self.nodes_map.get(&edge.v).unwrap();
        self.successors
            .entry(edge.u.clone())
            .or_default()
            .insert(edge.v.clone());
        self.successors_map
            .entry(u_node_index)
            .or_default()
            .insert(v_node_index);
        match self.specs.directed {
            true => {
                self.predecessors
                    .entry(edge.v.clone())
                    .or_default()
                    .insert(edge.u.clone());
                self.predecessors_map
                    .entry(v_node_index)
                    .or_default()
                    .insert(u_node_index);
            }
            false => {
                self.successors
                    .entry(edge.v.clone())
                    .or_default()
                    .insert(edge.u.clone());
                self.successors_map
                    .entry(v_node_index)
                    .or_default()
                    .insert(u_node_index);
            }
        }

        // if undirected, order the edge as that it can be easily queried for
        let ordered = match self.specs.directed {
            false => edge.clone().ordered().into(),
            true => Arc::clone(&edge),
        };
        let ordered_edge_index = match !self.specs.directed && u_node_index > v_node_index {
            false => EdgeIndex::new(u_node_index, v_node_index),
            true => EdgeIndex::new(v_node_index, u_node_index),
        };

        // add edge
        match self.specs.multi_edges {
            true => {
                self.edges
                    .entry((ordered.u.clone(), ordered.v.clone()))
                    .or_default()
                    .push(ordered.clone());
                self.edges_map
                    .entry(ordered_edge_index)
                    .or_default()
                    .push(ordered.clone());
            }
            false => match self.get_edge(ordered.u.clone(), ordered.v.clone()).is_ok() {
                false => {
                    self.edges.insert(
                        (ordered.u.clone(), ordered.v.clone()),
                        vec![ordered.clone()],
                    );
                    self.edges_map
                        .insert(ordered_edge_index, vec![ordered.clone()]);
                }
                true => match self.specs.edge_dedupe_strategy {
                    EdgeDedupeStrategy::KeepLast => {
                        self.edges
                            .insert((ordered.u.clone(), ordered.v.clone()), vec![ordered.into()]);
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
        T: Hash + Eq + Clone + Ord + Display,
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
    pub fn add_edges(&mut self, edges: Vec<Arc<Edge<T, A>>>) -> Result<(), Error>
    where
        T: Hash + Eq + Clone + Ord + Display,
        A: Clone,
    {
        for edge in edges {
            self.add_edge(edge)?;
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
        T: Hash + Eq + Clone + Ord + Display,
        A: Clone,
    {
        for edge in edges {
            self.add_edge(Edge::new(edge.0, edge.1))?;
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
    pub fn add_node(&mut self, node: Arc<Node<T, A>>)
    where
        T: Hash + Eq + Clone + Ord,
        A: Clone,
    {
        match self.nodes_map.contains_key(&node.name) {
            true => {
                let node_index = self.get_node_index(&node.name);
                self.nodes_vec[node_index] = node.clone();
                self.nodes_map_rev.insert(node_index, node.clone());
            }
            false => {
                let node_rc = node.clone();
                let node_index = self.nodes_vec.len();
                if !self.nodes_map.contains_key(&node.name) {
                    self.nodes_map.insert(node.name.clone(), node_index);
                }
                if !self.nodes_map_rev.contains_key(&node_index) {
                    self.nodes_map_rev.insert(node_index, Arc::clone(&node_rc));
                }
                self.nodes_vec.push(Arc::clone(&node_rc));
                self.successors_map.insert(
                    node_index,
                    HashSet::<usize, BuildNoHashHasher<usize>>::default(),
                );
                self.predecessors_map.insert(
                    node_index,
                    HashSet::<usize, BuildNoHashHasher<usize>>::default(),
                );
            }
        }
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
    pub fn add_nodes(&mut self, nodes: Vec<Arc<Node<T, A>>>)
    where
        T: Hash + Eq + Clone + Ord,
        A: Clone,
    {
        for node in nodes {
            self.add_node(node);
        }
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
            nodes_map: HashMap::<T, usize>::new(),
            nodes_map_rev: HashMap::<usize, Arc<Node<T, A>>>::new(),
            nodes_vec: Vec::<Arc<Node<T, A>>>::new(),
            edges: HashMap::<(T, T), Vec<Arc<Edge<T, A>>>>::new(),
            edges_map:
                HashMap::<EdgeIndex, Vec<Arc<Edge<T, A>>>, BuildNoHashHasher<usize>>::default(),
            specs,
            successors: HashMap::<T, HashSet<T>>::new(),
            successors_map: HashMap::<usize, HashSet<usize, BuildNoHashHasher<usize>>>::default(),
            predecessors: HashMap::<T, HashSet<T>>::new(),
            predecessors_map: HashMap::<usize, HashSet<usize, BuildNoHashHasher<usize>>>::default(),
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
        nodes: Vec<Arc<Node<T, A>>>,
        edges: Vec<Arc<Edge<T, A>>>,
        specs: GraphSpecs,
    ) -> Result<Graph<T, A>, Error>
    where
        T: Hash + Eq + Clone + Ord,
        A: Clone,
    {
        let mut graph = Graph::new(specs);
        graph.add_nodes(nodes);
        let result = graph.add_edges(edges);
        match result {
            Err(e) => Err(e),
            Ok(_) => Ok(graph),
        }
    }
}
