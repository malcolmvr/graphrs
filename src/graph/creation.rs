use super::Graph;
use crate::{
    AdjacentNode, Edge, EdgeDedupeStrategy, Error, ErrorKind, GraphSpecs, MissingNodeStrategy,
    Node, SelfLoopsFalseStrategy,
};
use nohash::{IntMap, IntSet};
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

        // add or insert nodes
        if !self.nodes_map.contains_key(&edge.u) {
            self.add_node(Node::from_name(edge.u.clone()).into());
        }
        if !self.nodes_map.contains_key(&edge.v) {
            self.add_node(Node::from_name(edge.v.clone()).into());
        }

        // get node indexes
        let u_node_index = *self.nodes_map.get(&edge.u).unwrap();
        let v_node_index = *self.nodes_map.get(&edge.v).unwrap();

        // check for duplicate edges
        let edge_already_exists = self.get_edge_by_indexes(u_node_index, v_node_index).is_ok();
        if self.specs.edge_dedupe_strategy == EdgeDedupeStrategy::Error
            && !self.specs.multi_edges
            && edge_already_exists
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

        // if undirected, order the edge as that it can be easily queried for
        let ordered = match self.specs.directed {
            false => edge.clone().ordered().into(),
            true => Arc::clone(&edge),
        };
        let (ordered_edge_u, ordered_edge_v) =
            match !self.specs.directed && u_node_index > v_node_index {
                false => (u_node_index, v_node_index),
                true => (v_node_index, u_node_index),
            };

        // add to the successors HashMap
        self.successors
            .entry(edge.u.clone())
            .or_default()
            .insert(edge.v.clone());

        // add to the successors node index IntMap
        self.successors_map
            .entry(u_node_index)
            .or_default()
            .insert(v_node_index);

        // add to the successors vec
        add_to_adjacency_vec(
            &mut self.successors_vec,
            ordered_edge_u,
            ordered_edge_v,
            edge.weight,
            edge_already_exists,
        );

        // add to predecessors
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
                add_to_adjacency_vec(
                    &mut self.predecessors_vec,
                    ordered_edge_v,
                    ordered_edge_u,
                    edge.weight,
                    edge_already_exists,
                );
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
                add_to_adjacency_vec(
                    &mut self.successors_vec,
                    ordered_edge_v,
                    ordered_edge_u,
                    edge.weight,
                    edge_already_exists,
                );
            }
        }

        // add edge
        match self.specs.multi_edges {
            true => {
                self.edges
                    .entry((ordered.u.clone(), ordered.v.clone()))
                    .or_default()
                    .push(ordered.clone());
                self.edges_map
                    .entry(ordered_edge_u)
                    .or_default()
                    .entry(ordered_edge_v)
                    .or_default()
                    .push(ordered.clone());
            }
            false => match self
                .get_edge_by_indexes(ordered_edge_u, ordered_edge_v)
                .is_ok()
            {
                false => {
                    self.edges.insert(
                        (ordered.u.clone(), ordered.v.clone()),
                        vec![ordered.clone()],
                    );
                    self.edges_map
                        .entry(ordered_edge_u)
                        .or_default()
                        .insert(ordered_edge_v, vec![ordered.clone()]);
                }
                true => match self.specs.edge_dedupe_strategy {
                    EdgeDedupeStrategy::KeepLast => {
                        self.edges.insert(
                            (ordered.u.clone(), ordered.v.clone()),
                            vec![ordered.clone()],
                        );
                        self.edges_map
                            .entry(ordered_edge_u)
                            .or_default()
                            .insert(ordered_edge_v, vec![ordered.clone()]);
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
                let node_index = self.get_node_index(&node.name).unwrap();
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
                self.successors_map
                    .insert(node_index, IntSet::<usize>::default());
                self.predecessors_map
                    .insert(node_index, IntSet::<usize>::default());
                self.successors_vec.push(vec![]);
                self.predecessors_vec.push(vec![]);
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
            nodes_map: HashMap::<T, usize>::default(),
            nodes_map_rev: IntMap::<usize, Arc<Node<T, A>>>::default(),
            nodes_vec: Vec::<Arc<Node<T, A>>>::new(),
            edges: HashMap::<(T, T), Vec<Arc<Edge<T, A>>>>::new(),
            edges_map: IntMap::<usize, IntMap<usize, Vec<Arc<Edge<T, A>>>>>::default(),
            specs,
            successors: HashMap::<T, HashSet<T>>::new(),
            successors_map: IntMap::<usize, IntSet<usize>>::default(),
            successors_vec: vec![],
            predecessors: HashMap::<T, HashSet<T>>::new(),
            predecessors_map: IntMap::<usize, IntSet<usize>>::default(),
            predecessors_vec: vec![],
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

/**
Adds a node to an adjacency (successor or predecessor) vector.
 */
fn add_to_adjacency_vec(
    adjacency_vec: &mut Vec<Vec<AdjacentNode>>,
    u_node_index: usize,
    v_node_index: usize,
    weight: f64,
    edge_already_exists: bool,
) {
    match edge_already_exists {
        true => {
            let index = adjacency_vec[u_node_index]
                .iter()
                .position(|succ| succ.node_index == v_node_index)
                .unwrap();
            if weight < adjacency_vec[u_node_index][index].weight {
                adjacency_vec[u_node_index][index] = AdjacentNode::new(v_node_index, weight);
            }
        }
        false => adjacency_vec[u_node_index].push(AdjacentNode::new(v_node_index, weight)),
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::{Edge, Graph, GraphSpecs};

    #[test]
    fn test_successors_vec_directed() {
        let edges = vec![
            Edge::with_weight("n0", "n1", 1.1),
            Edge::with_weight("n0", "n2", 1.3),
            Edge::with_weight("n0", "n3", 1.4),
            Edge::with_weight("n3", "n2", 1.5),
        ];
        let graph: Graph<&str, ()> =
            Graph::new_from_nodes_and_edges(vec![], edges, GraphSpecs::directed_create_missing())
                .unwrap();
        assert_eq!(
            graph.successors_vec,
            vec![
                vec![
                    AdjacentNode::new(1, 1.1),
                    AdjacentNode::new(2, 1.3),
                    AdjacentNode::new(3, 1.4)
                ],
                vec![],
                vec![],
                vec![AdjacentNode::new(2, 1.5)],
            ]
        );
        assert_eq!(
            graph.predecessors_vec,
            vec![
                vec![],
                vec![AdjacentNode::new(0, 1.1)],
                vec![AdjacentNode::new(0, 1.3), AdjacentNode::new(3, 1.5)],
                vec![AdjacentNode::new(0, 1.4)],
            ]
        );
    }

    #[test]
    fn test_successors_vec_undirected() {
        let edges = vec![
            Edge::with_weight("n0", "n1", 1.1),
            Edge::with_weight("n0", "n2", 1.3),
            Edge::with_weight("n0", "n3", 1.4),
            Edge::with_weight("n3", "n2", 1.5),
        ];
        let graph: Graph<&str, ()> =
            Graph::new_from_nodes_and_edges(vec![], edges, GraphSpecs::undirected_create_missing())
                .unwrap();
        assert_eq!(
            graph.successors_vec,
            vec![
                vec![
                    AdjacentNode::new(1, 1.1),
                    AdjacentNode::new(2, 1.3),
                    AdjacentNode::new(3, 1.4)
                ],
                vec![AdjacentNode::new(0, 1.1)],
                vec![AdjacentNode::new(0, 1.3), AdjacentNode::new(3, 1.5)],
                vec![AdjacentNode::new(0, 1.4), AdjacentNode::new(2, 1.5)],
            ]
        );
        assert_eq!(
            graph.predecessors_vec,
            vec![vec![], vec![], vec![], vec![],]
        );
    }
}
