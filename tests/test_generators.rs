#[cfg(test)]
mod tests {

    use graphrs::{complete_graph, Edge};
    use itertools::Itertools;
    use std::collections::HashSet;
    use std::iter::FromIterator;

    #[test]
    fn test_complete_graph_directed() {
        let graph = complete_graph(3, true);
        let all_edges = graph.get_all_edges();
        assert_eq!(all_edges.len(), 6);
        let hashset = all_edges
            .into_iter()
            .collect::<HashSet<&Edge<i32, &str, i32>>>();
        assert!(hashset.contains(&Edge::new(0, 1)));
        assert!(hashset.contains(&Edge::new(1, 2)));
        assert!(hashset.contains(&Edge::new(0, 2)));
        assert!(hashset.contains(&Edge::new(1, 0)));
        assert!(hashset.contains(&Edge::new(2, 1)));
        assert!(hashset.contains(&Edge::new(2, 0)));
    }

    #[test]
    fn test_complete_graph_undirected() {
        let graph = complete_graph(3, false);
        let all_edges = graph.get_all_edges();
        assert_eq!(all_edges.len(), 3);
        let hashset = all_edges
            .into_iter()
            .collect::<HashSet<&Edge<i32, &str, i32>>>();
        assert!(hashset.contains(&Edge::new(0, 1)));
        assert!(hashset.contains(&Edge::new(1, 2)));
        assert!(hashset.contains(&Edge::new(0, 2)));
    }
}
