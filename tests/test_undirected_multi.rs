#[cfg(test)]
mod tests {

    use graphrs::{Edge, Graph, GraphSpecs, MissingNodeStrategy, Node};
    use itertools::Itertools;

    #[test]
    fn test_add_edge_1() {
        let mut graph: Graph<&str, ()> = Graph::new(GraphSpecs {
            missing_node_strategy: MissingNodeStrategy::Create,
            multi_edges: true,
            ..GraphSpecs::undirected()
        });
        let result = graph.add_edge(Edge::new("n1", "n2"));
        assert!(result.is_ok());
        let result = graph.add_edge(Edge::new("n2", "n1"));
        assert!(result.is_ok());
    }

    #[test]
    fn test_add_edge_2() {
        let mut graph: Graph<&str, ()> = Graph::new(GraphSpecs {
            missing_node_strategy: MissingNodeStrategy::Create,
            multi_edges: true,
            ..GraphSpecs::undirected()
        });
        let result = graph.add_edge(Edge::new("n1", "n2"));
        assert!(result.is_ok());
        let result = graph.add_edge(Edge::new("n2", "n1"));
        assert!(result.is_ok());
        assert!(graph.get_edges("n2", "n1").is_ok());
        assert_eq!(graph.get_edges("n2", "n1").unwrap().len(), 2);
        assert!(graph.get_edges("n1", "n2").is_ok());
        assert!(graph.get_edges("n1", "n3").is_err());
        assert_eq!(graph.get_all_edges().len(), 2);
        assert_eq!(graph.get_neighbor_nodes("n1").unwrap().len(), 1);
        assert_eq!(graph.get_neighbor_nodes("n1").unwrap()[0].name, "n2");
        assert_eq!(graph.get_neighbor_nodes("n2").unwrap().len(), 1);
        assert_eq!(graph.get_neighbor_nodes("n2").unwrap()[0].name, "n1");
    }

    #[test]
    fn test_add_edge_3() {
        let mut graph: Graph<&str, ()> = Graph::new(GraphSpecs {
            missing_node_strategy: MissingNodeStrategy::Create,
            multi_edges: true,
            ..GraphSpecs::undirected()
        });
        let result = graph.add_edge(Edge::new("n1", "n2"));
        assert!(result.is_ok());
        let result = graph.add_edge(Edge::new("n1", "n2"));
        assert!(result.is_ok());
        let result = graph.add_edge(Edge::new("n1", "n3"));
        assert!(result.is_ok());
        let result = graph.add_edge(Edge::new("n1", "n3"));
        assert!(result.is_ok());

        assert_eq!(graph.get_all_edges().len(), 4);
        assert!(graph.get_edges("n2", "n1").is_ok());
        assert!(graph.get_edges("n1", "n3").is_ok());

        assert_eq!(graph.get_neighbor_nodes("n1").unwrap().len(), 2);
        assert_eq!(
            graph
                .get_neighbor_nodes("n1")
                .unwrap()
                .iter()
                .map(|n| n.name)
                .sorted()
                .collect::<Vec<&str>>(),
            vec!["n2", "n3"]
        );

        assert_eq!(graph.get_neighbor_nodes("n2").unwrap().len(), 1);
        assert_eq!(graph.get_neighbor_nodes("n2").unwrap()[0].name, "n1");

        assert_eq!(graph.get_neighbor_nodes("n3").unwrap().len(), 1);
        assert_eq!(graph.get_neighbor_nodes("n3").unwrap()[0].name, "n1");
    }

    #[test]
    fn test_add_edge_4() {
        let mut graph: Graph<&str, ()> = Graph::new(GraphSpecs {
            self_loops: false,
            missing_node_strategy: MissingNodeStrategy::Create,
            multi_edges: true,
            ..GraphSpecs::undirected()
        });
        let result = graph.add_edge(Edge::new("n1", "n1"));
        assert!(result.is_err());
    }

    #[test]
    fn test_add_edge_5() {
        let mut graph: Graph<&str, ()> = Graph::new(GraphSpecs {
            self_loops: true,
            missing_node_strategy: MissingNodeStrategy::Create,
            multi_edges: true,
            ..GraphSpecs::undirected()
        });
        let result = graph.add_edge(Edge::new("n1", "n1"));
        assert!(result.is_ok());
    }

    #[test]
    fn test_add_edges_1() {
        let mut graph: Graph<&str, ()> = Graph::new(GraphSpecs {
            missing_node_strategy: MissingNodeStrategy::Create,
            multi_edges: true,
            ..GraphSpecs::undirected()
        });
        let result = graph.add_edges(vec![Edge::new("n1", "n2"), Edge::new("n2", "n3")]);
        assert!(result.is_ok());
        assert!(graph.get_edges("n1", "n2").is_ok());
        assert!(graph.get_edges("n2", "n3").is_ok());
    }

    #[test]
    fn test_get_edge() {
        let graph = get_basic_graph(None);

        let result = graph.get_edge("n1", "n2");
        assert!(result.is_err());
    }

    #[test]
    fn test_get_edges_1() {
        let graph = get_basic_graph(None);

        let result = graph.get_edges("n1", "n2");
        assert!(result.is_ok());
        let edges = result.unwrap();
        assert_eq!(edges.len(), 2);
        let weights: f64 = edges.into_iter().map(|e| e.weight).sum();
        assert_eq!(weights, 3.0);

        let result = graph.get_edges("n1", "n3");
        assert!(result.is_ok());
        let weights: f64 = result.unwrap().into_iter().map(|e| e.weight).sum();
        assert_eq!(weights, 3.2);
    }

    #[test]
    fn test_get_edges_2() {
        let mut graph = get_basic_graph(None);
        let result = graph.add_edge(Edge::new("n2", "n1"));
        assert!(result.is_ok());

        let result = graph.get_edges("n1", "n2");
        assert!(result.is_ok());
        let edges = result.unwrap();
        assert_eq!(edges.len(), 3);
    }

    #[test]
    fn test_new_from_nodes_and_edges() {
        let graph = get_basic_graph(None);

        assert_eq!(graph.get_all_nodes().len(), 4);
        assert_eq!(graph.get_all_edges().len(), 5);
    }

    fn get_basic_graph<'a>(specs: Option<GraphSpecs>) -> Graph<&'a str, ()> {
        let nodes = vec![
            Node::from_name("n1"),
            Node::from_name("n2"),
            Node::from_name("n3"),
            Node::from_name("n4"),
        ];

        let edges = vec![
            Edge::with_weight("n1", "n2", 1.0),
            Edge::with_weight("n1", "n2", 2.0),
            Edge::with_weight("n1", "n3", 1.1),
            Edge::with_weight("n3", "n1", 2.1),
            Edge::with_weight("n4", "n2", 3.0),
        ];

        let final_specs = match specs {
            Some(s) => s,
            None => GraphSpecs::multi_undirected(),
        };

        Graph::new_from_nodes_and_edges(nodes, edges, final_specs).unwrap()
    }
}
