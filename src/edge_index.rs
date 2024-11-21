use std::hash::Hash;

#[derive(PartialEq, Eq)]
pub struct EdgeIndex {
    u: usize,
    v: usize,
}

impl Hash for EdgeIndex {
    fn hash<H: std::hash::Hasher>(&self, hasher: &mut H) {
        let single_value: u64 = self.u as u64 + (self.v as u64) << 32;
        hasher.write_u64(single_value);
    }
}

impl EdgeIndex {
    /**
    Creates a new `EdgeIndex`.

    # Arguments

    * `u`: The name of the first node of the edge.
    * `v`: The name of the second node of the edge.
    */
    pub fn new(u: usize, v: usize) -> EdgeIndex {
        EdgeIndex { u, v }
    }
}
