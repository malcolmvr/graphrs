pub struct GraphSpecs {
    pub acyclic: bool,
    pub directed: bool,
    pub edge_dedupe_strategy: EdgeDedupeStrategy,
    pub multi_edges: bool,
    pub self_loops: bool,
    pub self_loops_false_strategy: SelfLoopsFalseStrategy,
}

pub enum EdgeDedupeStrategy {
    Error,
    KeepFirst,
    KeepLast,
    MergeAttributes,
}

#[derive(PartialEq)]
pub enum SelfLoopsFalseStrategy {
    Error,
    Drop,
}

impl GraphSpecs {
    pub fn directed() -> GraphSpecs {
        GraphSpecs {
            acyclic: false,
            directed: true,
            edge_dedupe_strategy: EdgeDedupeStrategy::Error,
            multi_edges: false,
            self_loops: false,
            self_loops_false_strategy: SelfLoopsFalseStrategy::Error,
        }
    }

    pub fn undirected() -> GraphSpecs {
        GraphSpecs {
            acyclic: false,
            directed: false,
            edge_dedupe_strategy: EdgeDedupeStrategy::Error,
            multi_edges: false,
            self_loops: false,
            self_loops_false_strategy: SelfLoopsFalseStrategy::Error,
        }
    }

    pub fn multi_directed() -> GraphSpecs {
        GraphSpecs {
            acyclic: false,
            directed: true,
            edge_dedupe_strategy: EdgeDedupeStrategy::Error,
            multi_edges: true,
            self_loops: true,
            self_loops_false_strategy: SelfLoopsFalseStrategy::Error,
        }
    }

    pub fn multi_undirected() -> GraphSpecs {
        GraphSpecs {
            acyclic: false,
            directed: false,
            edge_dedupe_strategy: EdgeDedupeStrategy::Error,
            multi_edges: true,
            self_loops: true,
            self_loops_false_strategy: SelfLoopsFalseStrategy::Error,
        }
    }
}
