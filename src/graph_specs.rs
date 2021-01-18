pub struct GraphSpecs {
    pub acyclic: bool,
    pub directed: bool,
    pub multi_edges: bool,
    pub self_loops: bool,
}

impl GraphSpecs {
    pub fn new(acyclic: bool, directed: bool, multi_edges: bool, self_loops: bool) -> GraphSpecs {
        GraphSpecs {
            acyclic,
            directed,
            multi_edges,
            self_loops,
        }
    }
}
