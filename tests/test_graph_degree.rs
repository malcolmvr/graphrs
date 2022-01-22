#[cfg(test)]
mod tests {

    use graphrs::{Edge, Graph, GraphSpecs};

    #[test]
    fn test_get_node_degree_1() {
        let edges = vec![Edge::new("n1", "n2")];
        let graph: Graph<&str, ()> =
            Graph::new_from_nodes_and_edges(vec![], edges, GraphSpecs::directed_create_missing())
                .unwrap();
        assert_eq!(graph.get_node_degree("n1").unwrap(), 1);
        assert_eq!(graph.get_node_degree("n2").unwrap(), 1);
    }

    #[test]
    fn test_get_node_degree_2() {
        let edges = vec![Edge::new("n1", "n2")];
        let graph: Graph<&str, ()> =
            Graph::new_from_nodes_and_edges(vec![], edges, GraphSpecs::undirected_create_missing())
                .unwrap();
        assert_eq!(graph.get_node_degree("n1").unwrap(), 1);
        assert_eq!(graph.get_node_degree("n2").unwrap(), 1);
    }

    #[test]
    fn test_get_node_degree_3() {
        let edges = vec![Edge::new("n1", "n2"), Edge::new("n2", "n1")];
        let graph: Graph<&str, ()> =
            Graph::new_from_nodes_and_edges(vec![], edges, GraphSpecs::directed_create_missing())
                .unwrap();
        assert_eq!(graph.get_node_degree("n1").unwrap(), 2);
        assert_eq!(graph.get_node_degree("n2").unwrap(), 2);
    }

    #[test]
    fn test_get_node_degree_4() {
        let edges = vec![
            Edge::new("n1", "n2"),
            Edge::new("n1", "n3"),
            Edge::new("n1", "n4"),
            Edge::new("n1", "n5"),
        ];
        let graph: Graph<&str, ()> =
            Graph::new_from_nodes_and_edges(vec![], edges, GraphSpecs::directed_create_missing())
                .unwrap();
        assert_eq!(graph.get_node_degree("n1").unwrap(), 4);
        assert_eq!(graph.get_node_degree("n2").unwrap(), 1);
        assert_eq!(graph.get_node_degree("n3").unwrap(), 1);
        assert_eq!(graph.get_node_degree("n4").unwrap(), 1);
        assert_eq!(graph.get_node_degree("n5").unwrap(), 1);
    }

    #[test]
    fn test_get_node_degree_5() {
        let edges = vec![
            Edge::new("n2", "n1"),
            Edge::new("n3", "n1"),
            Edge::new("n4", "n1"),
            Edge::new("n5", "n1"),
        ];
        let graph: Graph<&str, ()> =
            Graph::new_from_nodes_and_edges(vec![], edges, GraphSpecs::directed_create_missing())
                .unwrap();
        assert_eq!(graph.get_node_degree("n1").unwrap(), 4);
        assert_eq!(graph.get_node_degree("n2").unwrap(), 1);
        assert_eq!(graph.get_node_degree("n3").unwrap(), 1);
        assert_eq!(graph.get_node_degree("n4").unwrap(), 1);
        assert_eq!(graph.get_node_degree("n5").unwrap(), 1);
    }

    #[test]
    fn test_get_node_weighted_degree_1() {
        let edges = vec![
            Edge::with_weight("n2", "n1", 1.0),
            Edge::with_weight("n3", "n1", 2.0),
            Edge::with_weight("n4", "n1", 3.0),
            Edge::with_weight("n1", "n4", 4.0),
        ];
        let graph: Graph<&str, ()> =
            Graph::new_from_nodes_and_edges(vec![], edges, GraphSpecs::directed_create_missing())
                .unwrap();
        assert_eq!(graph.get_node_weighted_degree("n1").unwrap(), 10.0);
        assert_eq!(graph.get_node_weighted_degree("n2").unwrap(), 1.0);
        assert_eq!(graph.get_node_weighted_degree("n3").unwrap(), 2.0);
        assert_eq!(graph.get_node_weighted_degree("n4").unwrap(), 7.0);
    }

    #[test]
    fn test_get_node_weighted_degree_2() {
        let edges = vec![
            Edge::with_weight("n2", "n1", 1.0),
            Edge::with_weight("n3", "n1", 2.0),
            Edge::with_weight("n4", "n1", 3.0),
        ];
        let graph: Graph<&str, ()> =
            Graph::new_from_nodes_and_edges(vec![], edges, GraphSpecs::directed_create_missing())
                .unwrap();
        assert_eq!(graph.get_node_weighted_degree("n1").unwrap(), 6.0);
        assert_eq!(graph.get_node_weighted_degree("n2").unwrap(), 1.0);
        assert_eq!(graph.get_node_weighted_degree("n3").unwrap(), 2.0);
        assert_eq!(graph.get_node_weighted_degree("n4").unwrap(), 3.0);
    }
}
