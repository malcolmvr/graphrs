#[cfg(test)]
mod tests {

    use graphrs::{
        algorithms::shortest_path::unweighted, generators, Edge, Graph, GraphSpecs,
        MissingNodeStrategy, Node,
    };

    #[test]
    fn test_all_pairs_dijkstra_1() {
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

        let result = unweighted::all_pairs_dijkstra(&graph, None);
        assert!(result.is_ok());
        let unwrapped = result.unwrap();
        assert_eq!(unwrapped.get("n1").unwrap().get("n1").unwrap().distance, 0);
        assert_eq!(unwrapped.get("n1").unwrap().get("n2").unwrap().distance, 1);
        assert_eq!(
            unwrapped.get("n1").unwrap().get("n2").unwrap().path,
            vec!["n1", "n2"]
        );
        assert_eq!(unwrapped.get("n1").unwrap().get("n3").unwrap().distance, 2);
        assert!(
            unwrapped.get("n1").unwrap().get("n3").unwrap().path == vec!["n1", "n4", "n3"]
                || unwrapped.get("n1").unwrap().get("n3").unwrap().path == vec!["n1", "n2", "n3"]
        );
        assert_eq!(unwrapped.get("n1").unwrap().get("n4").unwrap().distance, 1);
        assert_eq!(
            unwrapped.get("n1").unwrap().get("n4").unwrap().path,
            vec!["n1", "n4"]
        );
        assert_eq!(unwrapped.get("n1").unwrap().get("n5").unwrap().distance, 1);
        assert_eq!(
            unwrapped.get("n1").unwrap().get("n5").unwrap().path,
            vec!["n1", "n5"]
        );

        assert!(unwrapped.get("n2").unwrap().get("n1").is_none());
        assert_eq!(unwrapped.get("n2").unwrap().get("n2").unwrap().distance, 0);
        assert_eq!(unwrapped.get("n2").unwrap().get("n3").unwrap().distance, 1);
        assert_eq!(
            unwrapped.get("n2").unwrap().get("n3").unwrap().path,
            vec!["n2", "n3"]
        );
        assert!(unwrapped.get("n2").unwrap().get("n4").is_none());
        assert_eq!(unwrapped.get("n2").unwrap().get("n5").unwrap().distance, 2);
        assert_eq!(
            unwrapped.get("n2").unwrap().get("n5").unwrap().path,
            vec!["n2", "n3", "n5"]
        );

        assert!(unwrapped.get("n3").unwrap().get("n1").is_none());
        assert!(unwrapped.get("n3").unwrap().get("n2").is_none());
        assert_eq!(unwrapped.get("n3").unwrap().get("n3").unwrap().distance, 0);
        assert!(unwrapped.get("n3").unwrap().get("n4").is_none());
        assert_eq!(unwrapped.get("n3").unwrap().get("n5").unwrap().distance, 1);
        assert_eq!(
            unwrapped.get("n3").unwrap().get("n5").unwrap().path,
            vec!["n3", "n5"]
        );

        assert!(unwrapped.get("n4").unwrap().get("n1").is_none());
        assert!(unwrapped.get("n4").unwrap().get("n2").is_none());
        assert_eq!(unwrapped.get("n4").unwrap().get("n3").unwrap().distance, 1);
        assert_eq!(
            unwrapped.get("n4").unwrap().get("n3").unwrap().path,
            vec!["n4", "n3"]
        );
        assert_eq!(unwrapped.get("n4").unwrap().get("n4").unwrap().distance, 0);
        assert_eq!(unwrapped.get("n4").unwrap().get("n5").unwrap().distance, 2);
        assert_eq!(
            unwrapped.get("n4").unwrap().get("n5").unwrap().path,
            vec!["n4", "n3", "n5"]
        );

        assert!(unwrapped.get("n5").unwrap().get("n1").is_none());
        assert!(unwrapped.get("n5").unwrap().get("n2").is_none());
        assert!(unwrapped.get("n5").unwrap().get("n3").is_none());
        assert!(unwrapped.get("n5").unwrap().get("n4").is_none());
        assert_eq!(unwrapped.get("n5").unwrap().get("n5").unwrap().distance, 0);
    }

    #[test]
    fn test_all_pairs_dijkstra_2() {
        let graph = generators::social::karate_club_graph();

        let result = unweighted::all_pairs_dijkstra(&graph, None);
        assert!(result.is_ok());
        let unwrapped = result.unwrap();

        assert_eq!(unwrapped.get(&0).unwrap().get(&0).unwrap().distance, 0);
        assert_eq!(unwrapped.get(&0).unwrap().get(&1).unwrap().distance, 1);
        assert_eq!(unwrapped.get(&0).unwrap().get(&2).unwrap().distance, 1);
        assert_eq!(unwrapped.get(&0).unwrap().get(&3).unwrap().distance, 1);
        assert_eq!(unwrapped.get(&0).unwrap().get(&4).unwrap().distance, 1);
        assert_eq!(unwrapped.get(&0).unwrap().get(&5).unwrap().distance, 1);
        assert_eq!(unwrapped.get(&0).unwrap().get(&6).unwrap().distance, 1);
        assert_eq!(unwrapped.get(&0).unwrap().get(&7).unwrap().distance, 1);
        assert_eq!(unwrapped.get(&0).unwrap().get(&8).unwrap().distance, 1);
        assert_eq!(unwrapped.get(&0).unwrap().get(&10).unwrap().distance, 1);
        assert_eq!(unwrapped.get(&0).unwrap().get(&11).unwrap().distance, 1);
        assert_eq!(unwrapped.get(&0).unwrap().get(&12).unwrap().distance, 1);
        assert_eq!(unwrapped.get(&0).unwrap().get(&13).unwrap().distance, 1);
        assert_eq!(unwrapped.get(&0).unwrap().get(&17).unwrap().distance, 1);
        assert_eq!(unwrapped.get(&0).unwrap().get(&19).unwrap().distance, 1);
        assert_eq!(unwrapped.get(&0).unwrap().get(&21).unwrap().distance, 1);
        assert_eq!(unwrapped.get(&0).unwrap().get(&31).unwrap().distance, 1);
        assert_eq!(unwrapped.get(&0).unwrap().get(&30).unwrap().distance, 2);
        assert_eq!(unwrapped.get(&0).unwrap().get(&9).unwrap().distance, 2);
        assert_eq!(unwrapped.get(&0).unwrap().get(&27).unwrap().distance, 2);
        assert_eq!(unwrapped.get(&0).unwrap().get(&28).unwrap().distance, 2);
        assert_eq!(unwrapped.get(&0).unwrap().get(&32).unwrap().distance, 2);
        assert_eq!(unwrapped.get(&0).unwrap().get(&16).unwrap().distance, 2);
        assert_eq!(unwrapped.get(&0).unwrap().get(&33).unwrap().distance, 2);
        assert_eq!(unwrapped.get(&0).unwrap().get(&24).unwrap().distance, 2);
        assert_eq!(unwrapped.get(&0).unwrap().get(&25).unwrap().distance, 2);
        assert_eq!(unwrapped.get(&0).unwrap().get(&23).unwrap().distance, 3);
        assert_eq!(unwrapped.get(&0).unwrap().get(&14).unwrap().distance, 3);
        assert_eq!(unwrapped.get(&0).unwrap().get(&15).unwrap().distance, 3);
        assert_eq!(unwrapped.get(&0).unwrap().get(&18).unwrap().distance, 3);
        assert_eq!(unwrapped.get(&0).unwrap().get(&20).unwrap().distance, 3);
        assert_eq!(unwrapped.get(&0).unwrap().get(&22).unwrap().distance, 3);
        assert_eq!(unwrapped.get(&0).unwrap().get(&29).unwrap().distance, 3);
        assert_eq!(unwrapped.get(&0).unwrap().get(&26).unwrap().distance, 3);
    }

    #[test]
    fn test_single_source_dijkstra_1() {
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

        let result = unweighted::single_source_dijkstra(&graph, "n1", Some("n3"), None);
        assert!(result.is_ok());
        let unwrapped = result.unwrap();
        assert_eq!(unwrapped.get("n1").unwrap().distance, 0);
        assert_eq!(unwrapped.get("n2").unwrap().distance, 1);
        assert_eq!(unwrapped.get("n2").unwrap().path, vec!["n1", "n2"]);
        assert_eq!(unwrapped.get("n3").unwrap().distance, 2);
        assert!(
            unwrapped.get("n3").unwrap().path == vec!["n1", "n4", "n3"]
                || unwrapped.get("n3").unwrap().path == vec!["n1", "n2", "n3"]
        );
    }

    #[test]
    fn test_multi_source_dijkstra_1() {
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

        let result = unweighted::multi_source_dijkstra(&graph, vec!["n1"], Some("n3"), None);
        assert!(result.is_ok());
        let unwrapped = result.unwrap();
        let n3_info = unwrapped.get("n3").unwrap();
        assert_eq!(n3_info.distance, 2);
        assert!(n3_info.path == vec!["n1", "n4", "n3"] || n3_info.path == vec!["n1", "n2", "n3"]);
    }

    #[test]
    fn test_multi_source_dijkstra_3() {
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

        let result = unweighted::multi_source_dijkstra(&graph, vec!["n1"], None, None);
        assert!(result.is_ok());
        let unwrapped = result.unwrap();
        assert_eq!(unwrapped.get("n1").unwrap().distance, 0);
        assert_eq!(unwrapped.get("n2").unwrap().distance, 1);
        assert_eq!(unwrapped.get("n2").unwrap().path, vec!["n1", "n2"]);
        assert_eq!(unwrapped.get("n3").unwrap().distance, 2);
        assert!(
            unwrapped.get("n3").unwrap().path == vec!["n1", "n4", "n3"]
                || unwrapped.get("n3").unwrap().path == vec!["n1", "n2", "n3"]
        );
        assert_eq!(unwrapped.get("n4").unwrap().distance, 1);
        assert_eq!(unwrapped.get("n4").unwrap().path, vec!["n1", "n4"]);
        assert_eq!(unwrapped.get("n5").unwrap().distance, 1);
        assert_eq!(unwrapped.get("n5").unwrap().path, vec!["n1", "n5"]);
    }
}
