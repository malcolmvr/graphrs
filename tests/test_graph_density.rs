#[cfg(test)]
mod tests {

    use graphrs::{generators, Edge, Graph, GraphSpecs};

    #[test]
    fn test_graph_get_density_1() {
        let graph = generators::social::karate_club_graph();
        assert_eq!(graph.get_density(), 0.13903743315508021);
    }

    #[test]
    fn test_graph_get_density_2() {
        let edges = vec![
            Edge::new("n2", "n1"),
            Edge::new("n3", "n1"),
            Edge::new("n4", "n1"),
            Edge::new("n5", "n1"),
        ];
        let graph: Graph<&str, ()> =
            Graph::new_from_nodes_and_edges(vec![], edges, GraphSpecs::directed_create_missing())
                .unwrap();
        assert_eq!(graph.get_density(), 0.2);
    }

    #[test]
    fn test_graph_get_density_3() {
        let edges = vec![
            Edge::new("n2", "n1"),
            Edge::new("n3", "n1"),
            Edge::new("n4", "n1"),
            Edge::new("n5", "n1"),
        ];
        let graph: Graph<&str, ()> =
            Graph::new_from_nodes_and_edges(vec![], edges, GraphSpecs::undirected_create_missing())
                .unwrap();
        assert_eq!(graph.get_density(), 0.4);
    }

    #[test]
    fn test_graph_get_density_4() {
        let graph: Graph<&str, ()> =
            Graph::new_from_nodes_and_edges(vec![], vec![], GraphSpecs::directed_create_missing())
                .unwrap();
        assert_eq!(graph.get_density(), 0.0);
    }
}
