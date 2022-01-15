/**
Information about the weighted shortest path between two nodes.
*/
pub struct ShortestPathInfo<T> {
    /// The distance (sum-of-weights) between two nodes.
    pub distance: f64,
    /// The paths between two nodes. If there are more than one they
    /// will be of equal length. In each path the first item is the starting node
    /// and the last item is the target node.
    pub paths: Vec<Vec<T>>,
}

impl<T> ShortestPathInfo<T> {
    pub fn contains_path_through_node(&self, node_name: T) -> bool
    where
        T: Eq + Clone,
    {
        for path in &self.paths {
            if path.len() <= 2 {
                continue;
            }
            if path[1..(path.len() - 1)].contains(&node_name) {
                return true;
            }
        }
        false
    }
}
