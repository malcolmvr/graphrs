/**
Specifications for the type of [Graph](./struct.Graph.html) being created
and how various situations involving the addition of nodes and edges are handled.

# Example: using the `directed` associated function to specify a directed graph

```
use graphrs::{GraphSpecs};
let specs = GraphSpecs::directed();
```

# Example: defining an undirected graph that allows self loops

```
use graphrs::{GraphSpecs};
let specs = GraphSpecs {
    self_loops: true,
    ..GraphSpecs::undirected()
};
```
*/
#[derive(Clone)]
pub struct GraphSpecs {
    /// Determines if a [Graph](./struct.Graph.html) is directed or undirected.
    pub directed: bool,
    /// Determines what happens if duplicate or redundant edges are added to a [Graph](./struct.Graph.html).
    pub edge_dedupe_strategy: EdgeDedupeStrategy,
    /// Determines what happens if an [Edge](./struct.Edge.html) is added to a [Graph](./struct.Graph.html) but a corresponding
    /// [Node](./struct.Node.html) doesn't exist.
    pub missing_node_strategy: MissingNodeStrategy,
    /// Determines if a [Graph](./struct.Graph.html) supports multiple edges between nodes.
    pub multi_edges: bool,
    /// Determines if a [Graph](./struct.Graph.html) allows an [Edge](./struct.Edge.html) to be defined that starts and ends on the
    /// same [Node](./struct.Node.html).
    pub self_loops: bool,
    /// Determines what happens if a self-loop is added to a [Graph](./struct.Graph.html) that doesn't support them.
    pub self_loops_false_strategy: SelfLoopsFalseStrategy,
}

/**
Specifies options for a situation where a duplicate edge is being added to a [Graph](./struct.Graph.html).

`Error`: return an `Error`.

`KeepFirst`: keep the first (original) [Edge](./struct.Edge.html) and discard the one that is being added.

`KeepLast`: discard the first (original) [Edge](./struct.Edge.html) and keep the one that is being added.
*/
#[derive(Clone, PartialEq)]
pub enum EdgeDedupeStrategy {
    Error,
    KeepFirst,
    KeepLast,
}

/**
Specifies options for a situation where an [Edge](./struct.Edge.html) is being added to a
[Graph](./struct.Graph.html) but a corresponding [Node](./struct.Node.html) is not present in the
[Graph](./struct.Graph.html).

`Create`: create any nodes that aren't present

`Error`: return an `Error`.
*/
#[derive(Clone, PartialEq)]
pub enum MissingNodeStrategy {
    Create,
    Error,
}

/**
Specifies options for a situation where an [Edge](./struct.Edge.html) that starts and ends on the same [Node](./struct.Node.html) is
being added to a [Graph](./struct.Graph.html) but the [Graph](./struct.Graph.html) doesn't support self-loops.

`Error`: return an `Error`.

`Drop`: drops any self-loop edges
*/
#[derive(Clone, PartialEq)]
pub enum SelfLoopsFalseStrategy {
    Error,
    Drop,
}

const DEFAULT_GRAPH_SPECS: GraphSpecs = GraphSpecs {
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
    `directed` is true; `multi_edges` is false; `self_loops` is false;
    `edge_dedupe_strategy`, `missing_node_strategy`, and `self_loops_false_strategy` are
    all set to `Error`.

    # Examples

    ```
    use graphrs::{GraphSpecs};
    let specs = GraphSpecs::directed();
    ```
    */
    pub fn directed() -> GraphSpecs {
        DEFAULT_GRAPH_SPECS
    }

    /**
    Returns the specifications for a directed graph where missing nodes are
    automatically created.
    `directed` is true; `multi_edges` is false; `self_loops` is false;
    `missing_node_strategy` is `Create`,
    `edge_dedupe_strategy` and `self_loops_false_strategy` are set to `Error`.

    # Examples

    ```
    use graphrs::{GraphSpecs};
    let specs = GraphSpecs::directed_create_missing();
    ```
    */
    pub fn directed_create_missing() -> GraphSpecs {
        GraphSpecs {
            missing_node_strategy: MissingNodeStrategy::Create,
            ..DEFAULT_GRAPH_SPECS
        }
    }

    /**
    Returns the default `GraphSpecs` for an undirected graph.
    `directed` is false; `multi_edges` is false; `self_loops` is false;
    `edge_dedupe_strategy`, `missing_node_strategy`, and `self_loops_false_strategy` are
    all set to `Error`.

    # Examples

    ```
    use graphrs::{GraphSpecs};
    let specs = GraphSpecs::undirected();
    ```
    */
    pub fn undirected() -> GraphSpecs {
        GraphSpecs {
            directed: false,
            ..DEFAULT_GRAPH_SPECS
        }
    }

    /**
    Returns the specifications for an undirected graph where missing nodes are
    automatically created.
    `directed` is false; `multi_edges` is false; `self_loops` is false;
    `missing_node_strategy` is `Create`,
    `edge_dedupe_strategy` and `self_loops_false_strategy` are set to `Error`.

    # Examples

    ```
    use graphrs::{GraphSpecs};
    let specs = GraphSpecs::undirected_create_missing();
    ```
    */
    pub fn undirected_create_missing() -> GraphSpecs {
        GraphSpecs {
            directed: false,
            missing_node_strategy: MissingNodeStrategy::Create,
            ..DEFAULT_GRAPH_SPECS
        }
    }

    /**
    Returns the default `GraphSpecs` for an directed multi-graph.
    `directed` is true; `multi_edges` is true; `self_loops` is true;
    `edge_dedupe_strategy`, `missing_node_strategy`, and `self_loops_false_strategy` are
    all set to `Error`.

    # Examples

    ```
    use graphrs::{GraphSpecs};
    let specs = GraphSpecs::multi_directed();
    ```
    */
    pub fn multi_directed() -> GraphSpecs {
        GraphSpecs {
            multi_edges: true,
            self_loops: true,
            ..DEFAULT_GRAPH_SPECS
        }
    }

    /**
    Returns the default `GraphSpecs` for an undirected multi-graph.
    `directed` is false; `multi_edges` is true; `self_loops` is true;
    `edge_dedupe_strategy`, `missing_node_strategy`, and `self_loops_false_strategy` are
    all set to `Error`.

    # Examples

    ```
    use graphrs::{GraphSpecs};
    let specs = GraphSpecs::multi_undirected();
    ```
    */
    pub fn multi_undirected() -> GraphSpecs {
        GraphSpecs {
            directed: false,
            multi_edges: true,
            self_loops: true,
            ..DEFAULT_GRAPH_SPECS
        }
    }
}
