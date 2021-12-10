/**
`GraphSpecs` contains fields that place constaints on the graph and control the behavior
of adding nodes and edges.
**/
pub struct GraphSpecs {
    /// Forces a `Graph` to be acyclic, meaning it has no graph cycles.
    pub acyclic: bool,
    /// Determines if a `Graph` is directed or undirected.
    pub directed: bool,
    /// Determines what happens if duplicate or redundant edges are added to a `Graph`.
    pub edge_dedupe_strategy: EdgeDedupeStrategy,
    /// Determines what happens if an `Edge` is added to a `Graph` but a corresponding
    /// `Node` doesn't exist.
    pub missing_node_strategy: MissingNodeStrategy,
    /// Determines if a `Graph` supports multiple edges between nodes.
    pub multi_edges: bool,
    /// Determines if a `Graph` allows an `Edge` to be defined that starts and ends on the
    /// same `Node`.
    pub self_loops: bool,
    /// Determines what happens if a self-loop is added to a `Graph` that doesn't support them.
    pub self_loops_false_strategy: SelfLoopsFalseStrategy,
}

/**
Specifies options for a situation where a duplicate edge is being added to a `Graph`.

`Error`: return an `Error`.

`KeepFirst`: keep the first (original) `Edge` and discard the one that is being added.

`KeepLast`: discard the first (original) `Edge` and keep the one that is being added.
**/
#[derive(PartialEq)]
pub enum EdgeDedupeStrategy {
    Error,
    KeepFirst,
    KeepLast,
}

/**
Specifies options for a situation where an `Edge` is being added to a `Graph` but a corresponding
`Node` is not present in the `Graph`.

`Create`: create any nodes that aren't present

`Error`: return an `Error`.
**/
#[derive(PartialEq)]
pub enum MissingNodeStrategy {
    Create,
    Error,
}

/**
Specifies options for a situation where an `Edge` that starts and ends on the same `Node` is
being added to a `Graph` but the `Graph` doesn't support self-loops.

`Error`: return an `Error`.

`Drop`: drops any self-loop edges
**/
#[derive(PartialEq)]
pub enum SelfLoopsFalseStrategy {
    Error,
    Drop,
}

const DEFAULT_GRAPH_SPECS: GraphSpecs = GraphSpecs {
    acyclic: false,
    directed: true,
    edge_dedupe_strategy: EdgeDedupeStrategy::Error,
    missing_node_strategy: MissingNodeStrategy::Error,
    multi_edges: false,
    self_loops: false,
    self_loops_false_strategy: SelfLoopsFalseStrategy::Error,
};

impl GraphSpecs {

    /**
    Returns the default `GraphSpecs` for a directed graph.
    `acyclic` is false; `directed` is true; `multi_edges` is false; `self_loops` is false;
    `edge_dedupe_strategy`, `missing_node_strategy`, and `self_loops_false_strategy` are
    all set to `Error`.
    **/
    pub fn directed() -> GraphSpecs {
        DEFAULT_GRAPH_SPECS
    }

    /**
    Returns the default `GraphSpecs` for an undirected graph.
    `acyclic` is false; `directed` is false; `multi_edges` is false; `self_loops` is false;
    `edge_dedupe_strategy`, `missing_node_strategy`, and `self_loops_false_strategy` are
    all set to `Error`.
    **/
    pub fn undirected() -> GraphSpecs {
        GraphSpecs {
            directed: false,
            ..DEFAULT_GRAPH_SPECS
        }
    }

    /**
    Returns the default `GraphSpecs` for an directed multi-graph.
    `acyclic` is false; `directed` is true; `multi_edges` is true; `self_loops` is true;
    `edge_dedupe_strategy`, `missing_node_strategy`, and `self_loops_false_strategy` are
    all set to `Error`.
    **/
    pub fn multi_directed() -> GraphSpecs {
        GraphSpecs {
            multi_edges: true,
            self_loops: true,
            ..DEFAULT_GRAPH_SPECS
        }
    }

    /**
    Returns the default `GraphSpecs` for an undirected multi-graph.
    `acyclic` is false; `directed` is false; `multi_edges` is true; `self_loops` is true;
    `edge_dedupe_strategy`, `missing_node_strategy`, and `self_loops_false_strategy` are
    all set to `Error`.
    **/
    pub fn multi_undirected() -> GraphSpecs {
        GraphSpecs {
            directed: false,
            multi_edges: true,
            self_loops: true,
            ..DEFAULT_GRAPH_SPECS
        }
    }
}
