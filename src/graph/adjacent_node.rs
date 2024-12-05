#[derive(Debug, PartialEq)]
pub(crate) struct AdjacentNode {
    pub node_index: usize,
    pub weight: f64,
}

impl AdjacentNode {
    pub fn new(node_index: usize, weight: f64) -> Self {
        AdjacentNode { node_index, weight }
    }
}
