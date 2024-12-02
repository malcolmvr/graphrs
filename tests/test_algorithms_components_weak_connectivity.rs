mod utility;

#[cfg(test)]
mod tests {

    use std::collections::HashSet;

    use graphrs::{algorithms::components, generators, Edge, Graph, GraphSpecs};
    use std::iter::FromIterator;

    #[test]
    fn test_weakly_connected_components_1() {
        let edges = vec![Edge::new("n1", "n2")];
        let specs = GraphSpecs::undirected_create_missing();
        let graph: Graph<&str, ()> = Graph::new_from_nodes_and_edges(vec![], edges, specs).unwrap();
        let result = components::weakly_connected_components(&graph);
        assert!(result.is_err());
    }

    #[test]
    fn test_weakly_connected_components_2() {
        let edges = vec![
            Edge::new("n1", "n2"),
            Edge::new("n2", "n1"),
            Edge::new("n3", "n4"),
            Edge::new("n4", "n5"),
            Edge::new("n5", "n6"),
        ];
        let specs = GraphSpecs::directed_create_missing();
        let graph: Graph<&str, ()> = Graph::new_from_nodes_and_edges(vec![], edges, specs).unwrap();
        let result = components::weakly_connected_components(&graph).unwrap();
        assert_eq!(result.len(), 2);
        let c1 = result.iter().find(|hs| hs.get("n1").is_some()).unwrap();
        assert_eq!(c1, &vec!["n1", "n2"].into_iter().collect());
        let c2 = result.iter().find(|hs| hs.get("n3").is_some()).unwrap();
        assert_eq!(c2, &vec!["n3", "n4", "n5", "n6"].into_iter().collect());
    }

    #[test]
    fn test_weakly_connected_components_3() {
        let edges = vec![
            Edge::new("n1", "n2"),
            Edge::new("n3", "n4"),
            Edge::new("n5", "n6"),
            Edge::new("n7", "n8"),
            Edge::new("n8", "n1"),
        ];
        let specs = GraphSpecs::directed_create_missing();
        let graph: Graph<&str, ()> = Graph::new_from_nodes_and_edges(vec![], edges, specs).unwrap();
        let result = components::weakly_connected_components(&graph).unwrap();
        assert_eq!(result.len(), 3);
        let c1 = result.iter().find(|hs| hs.get("n1").is_some()).unwrap();
        assert_eq!(c1, &vec!["n1", "n2", "n7", "n8"].into_iter().collect());
        let c2 = result.iter().find(|hs| hs.get("n3").is_some()).unwrap();
        assert_eq!(c2, &vec!["n3", "n4"].into_iter().collect());
        let c3 = result.iter().find(|hs| hs.get("n5").is_some()).unwrap();
        assert_eq!(c3, &vec!["n5", "n6"].into_iter().collect());
    }

    #[test]
    fn test_bfs_equal_size_partitions_1() {
        let graph = generators::random::fast_gnp_random_graph(50, 0.05, true, Some(1)).unwrap();
        let result = components::bfs_equal_size_partitions(&graph, 4);
        assert_eq!(result.len(), 4);
        assert_eq!(result[0].len(), 13);
        assert_eq!(result[1].len(), 13);
        assert_eq!(result[2].len(), 13);
        assert_eq!(result[3].len(), 11);
        // check that the union of all partitions is the set of nodes
        let mut all_nodes: Vec<i32> = vec![];
        for partition in result.iter() {
            all_nodes.extend(partition.iter());
        }
        all_nodes.sort();
        assert_eq!(all_nodes[0], 0);
        assert_eq!(all_nodes[49], 49);
        let hs = HashSet::<&i32>::from_iter(all_nodes.iter());
        assert_eq!(hs.len(), 50);
    }

    #[test]
    fn test_bfs_equal_size_partitions_2() {
        let graph = generators::random::fast_gnp_random_graph(100, 0.01, true, Some(1)).unwrap();
        let result = components::bfs_equal_size_partitions(&graph, 5);
        assert_eq!(result.len(), 5);
        assert_eq!(result[0].len(), 21);
        assert_eq!(result[1].len(), 21);
        assert_eq!(result[2].len(), 21);
        assert_eq!(result[3].len(), 21);
        assert_eq!(result[4].len(), 16);
    }
}
