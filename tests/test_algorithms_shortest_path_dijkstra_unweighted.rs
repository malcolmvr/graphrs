#[cfg(test)]
mod tests {

    use graphrs::{
        algorithms::shortest_path::dijkstra, Edge, Graph, GraphSpecs, MissingNodeStrategy, Node,
    };
    use std::collections::HashSet;

    #[test]
    fn test_all_pairs_1() {
        let edges = vec![
            Edge::new("n1", "n2"),
            Edge::new("n2", "n3"),
            Edge::new("n1", "n4"),
            Edge::new("n4", "n3"),
            Edge::new("n1", "n5"),
            Edge::new("n3", "n5"),
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

        let result = dijkstra::all_pairs(&graph, false, None, false);
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
            2.0
        );
        assert_paths_contain_same_items(
            &unwrapped.get("n1").unwrap().get("n3").unwrap().paths,
            &[vec!["n1", "n4", "n3"], vec!["n1", "n2", "n3"]],
        );
        assert_eq!(
            unwrapped.get("n1").unwrap().get("n4").unwrap().distance,
            1.0
        );
        assert_eq!(
            unwrapped.get("n1").unwrap().get("n4").unwrap().paths,
            vec![vec!["n1", "n4"]]
        );
        assert_eq!(
            unwrapped.get("n1").unwrap().get("n5").unwrap().distance,
            1.0
        );
        assert_eq!(
            unwrapped.get("n1").unwrap().get("n5").unwrap().paths,
            vec![vec!["n1", "n5"]]
        );

        assert!(unwrapped.get("n2").unwrap().get("n1").is_none());
        assert_eq!(
            unwrapped.get("n2").unwrap().get("n2").unwrap().distance,
            0.0
        );
        assert_eq!(
            unwrapped.get("n2").unwrap().get("n3").unwrap().distance,
            1.0
        );
        assert_eq!(
            unwrapped.get("n2").unwrap().get("n3").unwrap().paths,
            vec![vec!["n2", "n3"]]
        );
        assert!(unwrapped.get("n2").unwrap().get("n4").is_none());
        assert_eq!(
            unwrapped.get("n2").unwrap().get("n5").unwrap().distance,
            2.0
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
            1.0
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
            2.0
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

    // #[test]
    // fn test_all_pairs_2() {
    //     let graph = generators::social::karate_club_graph();

    //     let result = dijkstra::all_pairs(&graph, false, None);
    //     assert!(result.is_ok());
    //     let unwrapped = result.unwrap();

    //     assert_eq!(unwrapped.get(&0).unwrap().get(&0).unwrap().distance, 0.0);
    //     assert_eq!(unwrapped.get(&0).unwrap().get(&1).unwrap().distance, 1.0);
    //     assert_eq!(unwrapped.get(&0).unwrap().get(&2).unwrap().distance, 1.0);
    //     assert_eq!(unwrapped.get(&0).unwrap().get(&3).unwrap().distance, 1.0);
    //     assert_eq!(unwrapped.get(&0).unwrap().get(&4).unwrap().distance, 1.0);
    //     assert_eq!(unwrapped.get(&0).unwrap().get(&5).unwrap().distance, 1.0);
    //     assert_eq!(unwrapped.get(&0).unwrap().get(&6).unwrap().distance, 1.0);
    //     assert_eq!(unwrapped.get(&0).unwrap().get(&7).unwrap().distance, 1.0);
    //     assert_eq!(unwrapped.get(&0).unwrap().get(&8).unwrap().distance, 1.0);
    //     assert_eq!(unwrapped.get(&0).unwrap().get(&10).unwrap().distance, 1.0);
    //     assert_eq!(unwrapped.get(&0).unwrap().get(&11).unwrap().distance, 1.0);
    //     assert_eq!(unwrapped.get(&0).unwrap().get(&12).unwrap().distance, 1.0);
    //     assert_eq!(unwrapped.get(&0).unwrap().get(&13).unwrap().distance, 1.0);
    //     assert_eq!(unwrapped.get(&0).unwrap().get(&17).unwrap().distance, 1.0);
    //     assert_eq!(unwrapped.get(&0).unwrap().get(&19).unwrap().distance, 1.0);
    //     assert_eq!(unwrapped.get(&0).unwrap().get(&21).unwrap().distance, 1.0);
    //     assert_eq!(unwrapped.get(&0).unwrap().get(&31).unwrap().distance, 1.0);
    //     assert_eq!(unwrapped.get(&0).unwrap().get(&30).unwrap().distance, 2.0);
    //     assert_eq!(unwrapped.get(&0).unwrap().get(&9).unwrap().distance, 2.0);
    //     assert_eq!(unwrapped.get(&0).unwrap().get(&27).unwrap().distance, 2.0);
    //     assert_eq!(unwrapped.get(&0).unwrap().get(&28).unwrap().distance, 2.0);
    //     assert_eq!(unwrapped.get(&0).unwrap().get(&32).unwrap().distance, 2.0);
    //     assert_eq!(unwrapped.get(&0).unwrap().get(&16).unwrap().distance, 2.0);
    //     assert_eq!(unwrapped.get(&0).unwrap().get(&33).unwrap().distance, 2.0);
    //     assert_eq!(unwrapped.get(&0).unwrap().get(&24).unwrap().distance, 2.0);
    //     assert_eq!(unwrapped.get(&0).unwrap().get(&25).unwrap().distance, 2.0);
    //     assert_eq!(unwrapped.get(&0).unwrap().get(&23).unwrap().distance, 3.0);
    //     assert_eq!(unwrapped.get(&0).unwrap().get(&14).unwrap().distance, 3.0);
    //     assert_eq!(unwrapped.get(&0).unwrap().get(&15).unwrap().distance, 3.0);
    //     assert_eq!(unwrapped.get(&0).unwrap().get(&18).unwrap().distance, 3.0);
    //     assert_eq!(unwrapped.get(&0).unwrap().get(&20).unwrap().distance, 3.0);
    //     assert_eq!(unwrapped.get(&0).unwrap().get(&22).unwrap().distance, 3.0);
    //     assert_eq!(unwrapped.get(&0).unwrap().get(&29).unwrap().distance, 3.0);
    //     assert_eq!(unwrapped.get(&0).unwrap().get(&26).unwrap().distance, 3.0);
    // }

    #[test]
    fn test_single_source_1() {
        let edges = vec![
            Edge::new("n1", "n2"),
            Edge::new("n2", "n3"),
            Edge::new("n1", "n4"),
            Edge::new("n4", "n3"),
            Edge::new("n1", "n5"),
            Edge::new("n3", "n5"),
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

        let result = dijkstra::single_source(&graph, false, "n1", Some("n3"), None, false);
        assert!(result.is_ok());
        let unwrapped = result.unwrap();
        assert!(unwrapped.get("n1").is_none());
        assert!(unwrapped.get("n2").is_none());
        assert_eq!(unwrapped.get("n3").unwrap().distance, 2.0);
        assert_paths_contain_same_items(
            &unwrapped.get("n3").unwrap().paths,
            &[vec!["n1", "n4", "n3"], vec!["n1", "n2", "n3"]],
        );
    }

    #[test]
    fn test_single_source_2() {
        let edges = vec![
            Edge::new("n12", "n0"),
            Edge::new("n0", "n2"),
            Edge::new("n2", "n9"),
            Edge::new("n12", "n3"),
            Edge::new("n3", "n1"),
            Edge::new("n3", "n7"),
            Edge::new("n3", "n2"),
            Edge::new("n1", "n7"),
            Edge::new("n1", "n2"),
            Edge::new("n7", "n2"),
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

        let result = dijkstra::single_source(&graph, false, "n12", Some("n9"), None, false);
        assert!(result.is_ok());
        let unwrapped = result.unwrap();
        assert_eq!(unwrapped.get("n9").unwrap().distance, 3.0);
        assert_paths_contain_same_items(
            &unwrapped.get("n9").unwrap().paths,
            &[vec!["n12", "n0", "n2", "n9"], vec!["n12", "n3", "n2", "n9"]],
        );
    }

    #[test]
    fn test_single_source_3() {
        let edges = vec![
            Edge::new("n12", "n0"),
            Edge::new("n0", "n2"),
            Edge::new("n2", "n9"),
            Edge::new("n12", "n3"),
            Edge::new("n3", "n1"),
            Edge::new("n3", "n7"),
            Edge::new("n3", "n2"),
            Edge::new("n1", "n7"),
            Edge::new("n1", "n2"),
            Edge::new("n7", "n2"),
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

        let result = dijkstra::single_source(&graph, false, "n12", Some("n9"), None, true);
        assert!(result.is_ok());
        let unwrapped = result.unwrap();
        let n9 = unwrapped.get("n9").unwrap();
        assert_eq!(n9.distance, 3.0);
        assert_eq!(n9.paths.len(), 1);
        assert!(
            n9.paths[0] == vec!["n12", "n3", "n2", "n9"]
                || n9.paths[0] == vec!["n12", "n0", "n2", "n9"]
        );
    }

    #[test]
    fn test_single_source_4() {
        let edges = vec![
            Edge::new("n1", "n2"),
            Edge::new("n1", "n3"),
            Edge::new("n2", "n3"),
            Edge::new("n4", "n5"),
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

        let result = dijkstra::single_source(&graph, false, "n1", Some("n5"), None, true);
        assert!(result.is_ok());
        let unwrapped = result.unwrap();
        assert_eq!(unwrapped.keys().len(), 0);
        let n5 = unwrapped.get("n5");
        assert!(n5.is_none());
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
            Edge::new("n1", "n2"),
            Edge::new("n2", "n3"),
            Edge::new("n1", "n4"),
            Edge::new("n4", "n3"),
        ];

        let graph = Graph::new_from_nodes_and_edges(nodes, edges, GraphSpecs::directed()).unwrap();

        let result = dijkstra::multi_source(&graph, false, vec!["n1"], Some("n3"), None, false);
        assert!(result.is_ok());
        let unwrapped = result.unwrap();
        let n3_info = unwrapped.get("n3").unwrap();
        assert_eq!(n3_info.distance, 2.0);
        assert_paths_contain_same_items(
            &n3_info.paths,
            &[vec!["n1", "n4", "n3"], vec!["n1", "n2", "n3"]],
        );
    }

    #[test]
    fn test_multi_source_3() {
        let edges = vec![
            Edge::new("n1", "n2"),
            Edge::new("n2", "n3"),
            Edge::new("n1", "n4"),
            Edge::new("n4", "n3"),
            Edge::new("n1", "n5"),
            Edge::new("n3", "n5"),
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

        let result = dijkstra::multi_source(&graph, false, vec!["n1"], None, None, false);
        assert!(result.is_ok());
        let unwrapped = result.unwrap();
        assert_eq!(unwrapped.get("n1").unwrap().distance, 0.0);
        assert_eq!(unwrapped.get("n2").unwrap().distance, 1.0);
        assert_eq!(unwrapped.get("n2").unwrap().paths, vec![vec!["n1", "n2"]]);
        assert_eq!(unwrapped.get("n3").unwrap().distance, 2.0);
        assert_paths_contain_same_items(
            &unwrapped.get("n3").unwrap().paths,
            &[vec!["n1", "n4", "n3"], vec!["n1", "n2", "n3"]],
        );
        assert_eq!(unwrapped.get("n4").unwrap().distance, 1.0);
        assert_eq!(unwrapped.get("n4").unwrap().paths, vec![vec!["n1", "n4"]]);
        assert_eq!(unwrapped.get("n5").unwrap().distance, 1.0);
        assert_eq!(unwrapped.get("n5").unwrap().paths, vec![vec!["n1", "n5"]]);
    }

    #[test]
    fn test_get_all_shortest_paths_involving() {
        let edges = vec![
            Edge::new("n1", "n2"),
            Edge::new("n2", "n3"),
            Edge::new("n1", "n4"),
            Edge::new("n4", "n3"),
            Edge::new("n2", "n5"),
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

        let result = dijkstra::get_all_shortest_paths_involving(&graph, "n2", false);
        assert_eq!(result.len(), 2);
    }

    fn assert_paths_contain_same_items(v1: &[Vec<&str>], v2: &[Vec<&str>]) {
        let a: HashSet<&Vec<&str>> = v1.iter().collect();
        let b: HashSet<&Vec<&str>> = v2.iter().collect();
        assert_eq!(a, b);
    }
}
