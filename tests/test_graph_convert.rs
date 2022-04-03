#[allow(unused_must_use)]
#[cfg(test)]
mod tests {

    use graphrs::{Edge, Graph, GraphSpecs, MissingNodeStrategy};

    #[test]
    fn test_to_single_edges_1() {
        let mut graph: Graph<&str, ()> = Graph::new(GraphSpecs {
            missing_node_strategy: MissingNodeStrategy::Create,
            multi_edges: true,
            ..GraphSpecs::directed()
        });
        graph.add_edges(vec![
            Edge::with_weight("n1", "n2", 1.1),
            Edge::with_weight("n1", "n2", 2.2),
            Edge::with_weight("n1", "n2", 3.3),
            Edge::with_weight("n1", "n3", 4.4),
        ]);
        let new_graph = graph.to_single_edges().unwrap();
        assert!(!new_graph.specs.multi_edges);
        assert_eq!(new_graph.get_all_edges().len(), 2);
        assert_eq!(new_graph.get_edge("n1", "n2").unwrap().weight, 6.6);
        assert_eq!(new_graph.get_edge("n1", "n3").unwrap().weight, 4.4);
    }

    #[test]
    fn test_to_single_edges_2() {
        let mut graph: Graph<&str, ()> = Graph::new(GraphSpecs::directed_create_missing());
        graph.add_edges(vec![
            Edge::with_weight("n1", "n2", 1.1),
            Edge::with_weight("n1", "n3", 4.4),
        ]);
        assert!(graph.to_single_edges().is_err());
    }

    #[test]
    fn test_to_single_edges_3() {
        let mut graph: Graph<&str, ()> = Graph::new(GraphSpecs {
            missing_node_strategy: MissingNodeStrategy::Create,
            multi_edges: true,
            ..GraphSpecs::directed()
        });
        graph.add_edges(vec![
            Edge::new("n1", "n2"),
            Edge::new("n1", "n2"),
            Edge::new("n1", "n2"),
            Edge::new("n1", "n3"),
        ]);
        let new_graph = graph.to_single_edges().unwrap();
        assert!(!new_graph.specs.multi_edges);
        assert_eq!(new_graph.get_all_edges().len(), 2);
        assert!(new_graph.get_edge("n1", "n2").unwrap().weight.is_nan());
        assert!(new_graph.get_edge("n1", "n3").unwrap().weight.is_nan());
    }

    #[test]
    fn test_set_all_edge_weights() {
        let mut graph: Graph<&str, ()> = Graph::new(GraphSpecs::directed_create_missing());
        graph.add_edges(vec![Edge::new("n1", "n3"), Edge::new("n2", "n3")]);
        let new_graph = graph.set_all_edge_weights(2.0);
        assert_eq!(new_graph.get_edge("n1", "n3").unwrap().weight, 2.0);
    }
}
