#[derive(Debug, PartialEq)]
pub(crate) struct Successor {
    pub node_index: usize,
    pub weight: f64,
}

impl Successor {
    pub fn new(node_index: usize, weight: f64) -> Self {
        Successor { node_index, weight }
    }
}
