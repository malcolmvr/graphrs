#[cfg(test)]
mod tests {

    use graphrs::{
        algorithms::shortest_path::dijkstra, generators, Edge, Graph, GraphSpecs,
        MissingNodeStrategy, Node,
    };
    use std::collections::HashSet;

    #[test]
    fn test_all_pairs_1() {
        let edges = vec![
            Edge::with_weight("n1", "n2", 1.0),
            Edge::with_weight("n2", "n3", 5.0),
            Edge::with_weight("n1", "n4", 2.0),
            Edge::with_weight("n4", "n3", 3.0),
            Edge::with_weight("n1", "n5", 9.0),
            Edge::with_weight("n3", "n5", 1.0),
        ];

        let graph: Graph<&str, ()> = Graph::new_from_nodes_and_edges(
            vec![],
            edges,
            GraphSpecs {
                missing_node_strategy: MissingNodeStrategy::Create,
                ..GraphSpecs::directed()
            },
        )
        .unwrap();

        let result = dijkstra::all_pairs(&graph, true, None, false);
        assert!(result.is_ok());
        let unwrapped = result.unwrap();
        assert_eq!(
            unwrapped.get("n1").unwrap().get("n1").unwrap().distance,
            0.0
        );
        assert_eq!(
            unwrapped.get("n1").unwrap().get("n2").unwrap().distance,
            1.0
        );
        assert_eq!(
            unwrapped.get("n1").unwrap().get("n2").unwrap().paths,
            vec![vec!["n1", "n2"]]
        );
        assert_eq!(
            unwrapped.get("n1").unwrap().get("n3").unwrap().distance,
            5.0
        );
        assert_eq!(
            unwrapped.get("n1").unwrap().get("n3").unwrap().paths,
            vec![vec!["n1", "n4", "n3"]]
        );
        assert_eq!(
            unwrapped.get("n1").unwrap().get("n4").unwrap().distance,
            2.0
        );
        assert_eq!(
            unwrapped.get("n1").unwrap().get("n4").unwrap().paths,
            vec![vec!["n1", "n4"]]
        );
        assert_eq!(
            unwrapped.get("n1").unwrap().get("n5").unwrap().distance,
            6.0
        );
        assert_eq!(
            unwrapped.get("n1").unwrap().get("n5").unwrap().paths,
            vec![vec!["n1", "n4", "n3", "n5"]]
        );

        assert!(unwrapped.get("n2").unwrap().get("n1").is_none());
        assert_eq!(
            unwrapped.get("n2").unwrap().get("n2").unwrap().distance,
            0.0
        );
        assert_eq!(
            unwrapped.get("n2").unwrap().get("n3").unwrap().distance,
            5.0
        );
        assert_eq!(
            unwrapped.get("n2").unwrap().get("n3").unwrap().paths,
            vec![vec!["n2", "n3"]]
        );
        assert!(unwrapped.get("n2").unwrap().get("n4").is_none());
        assert_eq!(
            unwrapped.get("n2").unwrap().get("n5").unwrap().distance,
            6.0
        );
        assert_eq!(
            unwrapped.get("n2").unwrap().get("n5").unwrap().paths,
            vec![vec!["n2", "n3", "n5"]]
        );

        assert!(unwrapped.get("n3").unwrap().get("n1").is_none());
        assert!(unwrapped.get("n3").unwrap().get("n2").is_none());
        assert_eq!(
            unwrapped.get("n3").unwrap().get("n3").unwrap().distance,
            0.0
        );
        assert!(unwrapped.get("n3").unwrap().get("n4").is_none());
        assert_eq!(
            unwrapped.get("n3").unwrap().get("n5").unwrap().distance,
            1.0
        );
        assert_eq!(
            unwrapped.get("n3").unwrap().get("n5").unwrap().paths,
            vec![vec!["n3", "n5"]]
        );

        assert!(unwrapped.get("n4").unwrap().get("n1").is_none());
        assert!(unwrapped.get("n4").unwrap().get("n2").is_none());
        assert_eq!(
            unwrapped.get("n4").unwrap().get("n3").unwrap().distance,
            3.0
        );
        assert_eq!(
            unwrapped.get("n4").unwrap().get("n3").unwrap().paths,
            vec![vec!["n4", "n3"]]
        );
        assert_eq!(
            unwrapped.get("n4").unwrap().get("n4").unwrap().distance,
            0.0
        );
        assert_eq!(
            unwrapped.get("n4").unwrap().get("n5").unwrap().distance,
            4.0
        );
        assert_eq!(
            unwrapped.get("n4").unwrap().get("n5").unwrap().paths,
            vec![vec!["n4", "n3", "n5"]]
        );

        assert!(unwrapped.get("n5").unwrap().get("n1").is_none());
        assert!(unwrapped.get("n5").unwrap().get("n2").is_none());
        assert!(unwrapped.get("n5").unwrap().get("n3").is_none());
        assert!(unwrapped.get("n5").unwrap().get("n4").is_none());
        assert_eq!(
            unwrapped.get("n5").unwrap().get("n5").unwrap().distance,
            0.0
        );
    }

    #[test]
    fn test_all_pairs_2() {
        let edges = vec![
            Edge::with_weight("n1", "n2", 1.0),
            Edge::with_weight("n2", "n3", 5.0),
            Edge::with_weight("n1", "n4", 2.0),
            Edge::with_weight("n4", "n3", 3.0),
            Edge::with_weight("n1", "n5", 9.0),
            Edge::with_weight("n3", "n5", 1.0),
        ];

        let graph: Graph<&str, ()> = Graph::new_from_nodes_and_edges(
            vec![],
            edges,
            GraphSpecs {
                missing_node_strategy: MissingNodeStrategy::Create,
                ..GraphSpecs::directed()
            },
        )
        .unwrap();

        let result = dijkstra::all_pairs(&graph, true, Some(2.9), false);
        assert!(result.is_ok());
        let unwrapped = result.unwrap();
        assert_eq!(
            unwrapped.get("n1").unwrap().get("n1").unwrap().distance,
            0.0
        );
        assert_eq!(
            unwrapped.get("n1").unwrap().get("n2").unwrap().distance,
            1.0
        );
        assert_eq!(
            unwrapped.get("n1").unwrap().get("n2").unwrap().paths,
            vec![vec!["n1", "n2"]]
        );
        assert!(unwrapped.get("n1").unwrap().get("n3").is_none());
        assert_eq!(
            unwrapped.get("n1").unwrap().get("n4").unwrap().distance,
            2.0
        );
        assert_eq!(
            unwrapped.get("n1").unwrap().get("n4").unwrap().paths,
            vec![vec!["n1", "n4"]]
        );
        assert!(unwrapped.get("n1").unwrap().get("n5").is_none());

        assert!(unwrapped.get("n2").unwrap().get("n1").is_none());
        assert_eq!(
            unwrapped.get("n2").unwrap().get("n2").unwrap().distance,
            0.0
        );
        assert!(unwrapped.get("n2").unwrap().get("n3").is_none());
        assert!(unwrapped.get("n2").unwrap().get("n4").is_none());
        assert!(unwrapped.get("n2").unwrap().get("n5").is_none());

        assert!(unwrapped.get("n3").unwrap().get("n1").is_none());
        assert!(unwrapped.get("n3").unwrap().get("n2").is_none());
        assert_eq!(
            unwrapped.get("n3").unwrap().get("n3").unwrap().distance,
            0.0
        );
        assert!(unwrapped.get("n3").unwrap().get("n4").is_none());
        assert_eq!(
            unwrapped.get("n3").unwrap().get("n5").unwrap().distance,
            1.0
        );
        assert_eq!(
            unwrapped.get("n3").unwrap().get("n5").unwrap().paths,
            vec![vec!["n3", "n5"]]
        );

        assert!(unwrapped.get("n4").unwrap().get("n1").is_none());
        assert!(unwrapped.get("n4").unwrap().get("n2").is_none());
        assert!(unwrapped.get("n4").unwrap().get("n3").is_none());
        assert_eq!(
            unwrapped.get("n4").unwrap().get("n4").unwrap().distance,
            0.0
        );
        assert!(unwrapped.get("n4").unwrap().get("n5").is_none());

        assert!(unwrapped.get("n5").unwrap().get("n1").is_none());
        assert!(unwrapped.get("n5").unwrap().get("n2").is_none());
        assert!(unwrapped.get("n5").unwrap().get("n3").is_none());
        assert!(unwrapped.get("n5").unwrap().get("n4").is_none());
        assert_eq!(
            unwrapped.get("n5").unwrap().get("n5").unwrap().distance,
            0.0
        );
    }

    #[test]
    fn test_single_source_1() {
        let edges = vec![
            Edge::with_weight("n1", "n2", 1.0),
            Edge::with_weight("n2", "n3", 5.0),
            Edge::with_weight("n1", "n4", 2.0),
            Edge::with_weight("n4", "n3", 3.0),
            Edge::with_weight("n1", "n5", 9.0),
            Edge::with_weight("n3", "n5", 1.0),
        ];

        let graph: Graph<&str, ()> = Graph::new_from_nodes_and_edges(
            vec![],
            edges,
            GraphSpecs {
                missing_node_strategy: MissingNodeStrategy::Create,
                ..GraphSpecs::directed()
            },
        )
        .unwrap();

        let result = dijkstra::single_source(&graph, true, "n1", Some("n3"), None, false);
        assert!(result.is_ok());
        let unwrapped = result.unwrap();
        assert!(unwrapped.get("n1").is_none());
        assert!(unwrapped.get("n2").is_none());
        assert_eq!(unwrapped.get("n3").unwrap().distance, 5.0);
        assert_eq!(
            unwrapped.get("n3").unwrap().paths,
            vec![vec!["n1", "n4", "n3"]]
        );
    }

    #[test]
    fn test_single_source_2() {
        let edges = vec![
            Edge::with_weight("n1", "n2", 1.0),
            Edge::with_weight("n1", "n3", 1.0),
            Edge::with_weight("n2", "n4", 1.0),
            Edge::with_weight("n3", "n4", 1.0),
        ];

        let graph: Graph<&str, ()> = Graph::new_from_nodes_and_edges(
            vec![],
            edges,
            GraphSpecs {
                missing_node_strategy: MissingNodeStrategy::Create,
                ..GraphSpecs::directed()
            },
        )
        .unwrap();

        let result = dijkstra::single_source(&graph, true, "n1", Some("n4"), None, false);
        assert!(result.is_ok());
        let unwrapped = result.unwrap();
        assert_eq!(unwrapped.get("n4").unwrap().distance, 2.0);
        assert_paths_contain_same_items(
            &unwrapped.get("n4").unwrap().paths,
            &[vec!["n1", "n2", "n4"], vec!["n1", "n3", "n4"]],
        );
    }

    #[test]
    fn test_single_source_3() {
        let edges = vec![
            Edge::with_weight("n1", "n2", 2.0),
            Edge::with_weight("n2", "n5", 1.0),
            Edge::with_weight("n1", "n3", 1.0),
            Edge::with_weight("n3", "n4", 1.0),
            Edge::with_weight("n4", "n5", 1.0),
        ];

        let graph: Graph<&str, ()> = Graph::new_from_nodes_and_edges(
            vec![],
            edges,
            GraphSpecs {
                missing_node_strategy: MissingNodeStrategy::Create,
                ..GraphSpecs::directed()
            },
        )
        .unwrap();

        let result = dijkstra::single_source(&graph, true, "n1", Some("n5"), None, false);
        assert!(result.is_ok());
        let unwrapped = result.unwrap();
        assert_eq!(unwrapped.get("n5").unwrap().distance, 3.0);
        assert_paths_contain_same_items(
            &unwrapped.get("n5").unwrap().paths,
            &[vec!["n1", "n2", "n5"], vec!["n1", "n3", "n4", "n5"]],
        );
    }

    #[test]
    fn test_single_source_4() {
        let edges = vec![
            Edge::with_weight("n1", "n2", 2.0),
            Edge::with_weight("n2", "n5", 1.0),
            Edge::with_weight("n1", "n3", 1.0),
            Edge::with_weight("n3", "n4", 1.0),
            Edge::with_weight("n4", "n5", 1.0),
        ];

        let graph: Graph<&str, ()> = Graph::new_from_nodes_and_edges(
            vec![],
            edges,
            GraphSpecs {
                missing_node_strategy: MissingNodeStrategy::Create,
                ..GraphSpecs::directed()
            },
        )
        .unwrap();

        let result = dijkstra::single_source(&graph, true, "n1", Some("n5"), None, true);
        assert!(result.is_ok());
        let unwrapped = result.unwrap();
        let n5 = unwrapped.get("n5").unwrap();
        assert_eq!(n5.distance, 3.0);
        assert_eq!(n5.paths.len(), 1);
        assert!(
            n5.paths[0] == vec!["n1", "n2", "n5"] || n5.paths[0] == vec!["n1", "n3", "n4", "n5"]
        );
    }

    #[test]
    fn test_multi_source_1() {
        let nodes = vec![
            Node::<&str, ()>::from_name("n2"),
            Node::<&str, ()>::from_name("n1"),
            Node::<&str, ()>::from_name("n3"),
            Node::<&str, ()>::from_name("n4"),
        ];

        let edges = vec![
            Edge::with_weight("n1", "n2", 1.0),
            Edge::with_weight("n2", "n3", 5.0),
            Edge::with_weight("n1", "n4", 2.0),
            Edge::with_weight("n4", "n3", 3.0),
        ];

        let graph = Graph::new_from_nodes_and_edges(nodes, edges, GraphSpecs::directed()).unwrap();

        let result = dijkstra::multi_source(&graph, true, vec!["n1"], Some("n3"), None, false);
        assert!(result.is_ok());
        let unwrapped = result.unwrap();
        let n3_info = unwrapped.get("n3").unwrap();
        assert_eq!(n3_info.distance, 5.0);
        assert_eq!(n3_info.paths, vec![vec!["n1", "n4", "n3"]]);
    }

    #[test]
    fn test_multi_source_2() {
        let graph = generators::social::karate_club_graph();
        let result = dijkstra::multi_source(&graph, true, vec![0, 1, 2], Some(24), None, false);
        assert!(result.is_err());
    }

    #[test]
    fn test_multi_source_3() {
        let edges = vec![
            Edge::with_weight("n1", "n2", 1.0),
            Edge::with_weight("n2", "n3", 5.0),
            Edge::with_weight("n1", "n4", 2.0),
            Edge::with_weight("n4", "n3", 3.0),
            Edge::with_weight("n1", "n5", 9.0),
            Edge::with_weight("n3", "n5", 1.0),
        ];

        let graph: Graph<&str, ()> = Graph::new_from_nodes_and_edges(
            vec![],
            edges,
            GraphSpecs {
                missing_node_strategy: MissingNodeStrategy::Create,
                ..GraphSpecs::directed()
            },
        )
        .unwrap();

        let result = dijkstra::multi_source(&graph, true, vec!["n1"], None, None, false);
        assert!(result.is_ok());
        let unwrapped = result.unwrap();
        assert_eq!(unwrapped.get("n1").unwrap().distance, 0.0);
        assert_eq!(unwrapped.get("n2").unwrap().distance, 1.0);
        assert_eq!(unwrapped.get("n2").unwrap().paths, vec![vec!["n1", "n2"]]);
        assert_eq!(unwrapped.get("n3").unwrap().distance, 5.0);
        assert_eq!(
            unwrapped.get("n3").unwrap().paths,
            vec![vec!["n1", "n4", "n3"]]
        );
        assert_eq!(unwrapped.get("n4").unwrap().distance, 2.0);
        assert_eq!(unwrapped.get("n4").unwrap().paths, vec![vec!["n1", "n4"]]);
        assert_eq!(unwrapped.get("n5").unwrap().distance, 6.0);
        assert_eq!(
            unwrapped.get("n5").unwrap().paths,
            vec![vec!["n1", "n4", "n3", "n5"]]
        );
    }

    fn assert_paths_contain_same_items(v1: &[Vec<&str>], v2: &[Vec<&str>]) {
        let a: HashSet<&Vec<&str>> = v1.iter().collect();
        let b: HashSet<&Vec<&str>> = v2.iter().collect();
        assert_eq!(a, b);
    }
}
