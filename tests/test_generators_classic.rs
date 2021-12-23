#[cfg(test)]
mod tests {

    use graphrs::{generators, Edge};
    use std::collections::HashSet;

    #[test]
    fn test_complete_graph_directed() {
        let graph = generators::classic::complete_graph(3, true);
        let all_edges = graph.get_all_edges();
        assert_eq!(all_edges.len(), 6);
        let hashset = all_edges.into_iter().collect::<HashSet<&Edge<i32, ()>>>();
        assert!(hashset.contains(&Edge::new(0, 1)));
        assert!(hashset.contains(&Edge::new(1, 2)));
        assert!(hashset.contains(&Edge::new(0, 2)));
        assert!(hashset.contains(&Edge::new(1, 0)));
        assert!(hashset.contains(&Edge::new(2, 1)));
        assert!(hashset.contains(&Edge::new(2, 0)));
    }

    #[test]
    fn test_complete_graph_undirected() {
        let graph = generators::classic::complete_graph(3, false);
        let all_edges = graph.get_all_edges();
        assert_eq!(all_edges.len(), 3);
        let hashset = all_edges.into_iter().collect::<HashSet<&Edge<i32, ()>>>();
        assert!(hashset.contains(&Edge::new(0, 1)));
        assert!(hashset.contains(&Edge::new(1, 2)));
        assert!(hashset.contains(&Edge::new(0, 2)));
    }
}
