pub struct GraphSpecs {
    pub acyclic: bool,
    pub directed: bool,
    pub edge_dedupe_strategy: EdgeDedupeStrategy,
    pub missing_node_strategy: MissingNodeStrategy,
    pub multi_edges: bool,
    pub self_loops: bool,
    pub self_loops_false_strategy: SelfLoopsFalseStrategy,
}

#[derive(PartialEq)]
pub enum EdgeDedupeStrategy {
    Error,
    KeepFirst,
    KeepLast,
}

#[derive(PartialEq)]
pub enum MissingNodeStrategy {
    Create,
    Error,
}

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
    pub fn directed() -> GraphSpecs {
        DEFAULT_GRAPH_SPECS
    }

    pub fn undirected() -> GraphSpecs {
        GraphSpecs { directed: false, ..DEFAULT_GRAPH_SPECS }
    }

    pub fn multi_directed() -> GraphSpecs {
        GraphSpecs { multi_edges: true, self_loops: true, ..DEFAULT_GRAPH_SPECS }
    }

    pub fn multi_undirected() -> GraphSpecs {
        GraphSpecs { directed: false, multi_edges: true, self_loops: true, ..DEFAULT_GRAPH_SPECS }
    }
}
