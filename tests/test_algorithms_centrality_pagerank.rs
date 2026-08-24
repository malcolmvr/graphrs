mod utility;

#[cfg(test)]
mod tests {
    use super::utility::round;
    use graphrs::{algorithms::centrality::pagerank, Edge, ErrorKind, Graph, GraphSpecs};

    #[test]
    fn test_pagerank_cycle_is_uniform() {
        let graph: Graph<&str, ()> = Graph::new_from_nodes_and_edges(
            vec![],
            vec![
                Edge::new("a", "b"),
                Edge::new("b", "c"),
                Edge::new("c", "a"),
            ],
            GraphSpecs::directed_create_missing(),
        )
        .unwrap();
        let result = pagerank::pagerank(&graph, false, None, None, None).unwrap();
        assert_eq!(round(result.get("a").unwrap(), 6), 0.333333);
        assert_eq!(round(result.get("b").unwrap(), 6), 0.333333);
        assert_eq!(round(result.get("c").unwrap(), 6), 0.333333);
    }

    #[test]
    fn test_pagerank_handles_dangling_node() {
        let graph: Graph<&str, ()> = Graph::new_from_nodes_and_edges(
            vec![],
            vec![Edge::new("a", "b")],
            GraphSpecs::directed_create_missing(),
        )
        .unwrap();
        let result = pagerank::pagerank(&graph, false, None, None, None).unwrap();
        assert!(result["b"] > result["a"]);
        assert_eq!(round(&result.values().sum::<f64>(), 6), 1.0);
    }

    #[test]
    fn test_pagerank_uses_weights() {
        let graph: Graph<&str, ()> = Graph::new_from_nodes_and_edges(
            vec![],
            vec![
                Edge::with_weight("a", "b", 9.0),
                Edge::with_weight("a", "c", 1.0),
                Edge::new("b", "a"),
                Edge::new("c", "a"),
            ],
            GraphSpecs::directed_create_missing(),
        )
        .unwrap();
        let result = pagerank::pagerank(&graph, true, None, None, None).unwrap();
        assert!(result["b"] > result["c"]);
    }

    #[test]
    fn test_pagerank_rejects_invalid_arguments() {
        let graph: Graph<&str, ()> = Graph::new(GraphSpecs::directed());
        let result = pagerank::pagerank(&graph, false, Some(1.0), None, None);
        assert!(matches!(
            result.unwrap_err().kind,
            ErrorKind::InvalidArgument
        ));
    }
}
