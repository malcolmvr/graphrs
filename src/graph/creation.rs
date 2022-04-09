use super::Graph;
use crate::{
    Edge, EdgeDedupeStrategy, Error, ErrorKind, GraphSpecs, MissingNodeStrategy, Node,
    SelfLoopsFalseStrategy,
};
use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::hash::Hash;

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
    pub fn add_edge(&mut self, edge: Edge<T, A>) -> Result<(), Error>
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
            .entry(edge.u.clone())
            .or_insert_with(|| Node::from_name(edge.u.clone()));
        self.nodes
            .entry(edge.v.clone())
            .or_insert_with(|| Node::from_name(edge.v.clone()));

        // add successors and predecessors
        self.successors
            .entry(edge.u.clone())
            .or_default()
            .insert(edge.v.clone());
        match self.specs.directed {
            true => {
                self.predecessors
                    .entry(edge.v.clone())
                    .or_default()
                    .insert(edge.u.clone());
            }
            false => {
                self.successors
                    .entry(edge.v.clone())
                    .or_default()
                    .insert(edge.u.clone());
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
                    .entry((ordered.u.clone(), ordered.v.clone()))
                    .or_default()
                    .push(ordered);
            }
            false => match self.get_edge(ordered.u.clone(), ordered.v.clone()).is_ok() {
                false => {
                    self.edges
                        .insert((ordered.u.clone(), ordered.v.clone()), vec![ordered]);
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
                        self.edges
                            .insert((ordered.u.clone(), ordered.v.clone()), vec![ordered]);
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
    pub fn add_edges(&mut self, edges: Vec<Edge<T, A>>) -> Result<(), Error>
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
    pub fn add_node(&mut self, node: Node<T, A>)
    where
        T: Hash + Eq + Clone + Ord,
        A: Clone,
    {
        self.nodes.insert(node.name.clone(), node);
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
