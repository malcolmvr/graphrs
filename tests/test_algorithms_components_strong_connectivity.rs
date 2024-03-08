mod utility;

#[cfg(test)]
mod tests {

    use graphrs::{algorithms::components, Edge, Graph, GraphSpecs};

    #[test]
    fn test_strongly_connected_components_1() {
        let edges = vec![
            Edge::new("n1", "n2"),
        ];
        let specs = GraphSpecs::undirected_create_missing();
        let graph: Graph<&str, ()> = Graph::new_from_nodes_and_edges(vec![], edges, specs).unwrap();
        let result = components::strongly_connected_components(&graph);
        assert!(result.is_err());
    }

    #[test]
    fn test_strongly_connected_components_2() {
        let edges = vec![
            Edge::new("n1", "n2"),
            Edge::new("n2", "n1"),
            Edge::new("n2", "n3"),
            Edge::new("n3", "n4"),
        ];
        let specs = GraphSpecs::directed_create_missing();
        let graph: Graph<&str, ()> = Graph::new_from_nodes_and_edges(vec![], edges, specs).unwrap();
        let result = components::strongly_connected_components(&graph).unwrap();
        assert_eq!(result.len(), 3);
        let c1 = result.iter().find(|hs| hs.get("n1").is_some()).unwrap();
        assert_eq!(c1, &vec!["n1", "n2"].into_iter().collect());
        let c2 = result.iter().find(|hs| hs.get("n3").is_some()).unwrap();
        assert_eq!(c2, &vec!["n3"].into_iter().collect());
        let c3 = result.iter().find(|hs| hs.get("n4").is_some()).unwrap();
        assert_eq!(c3, &vec!["n4"].into_iter().collect());
    }

    #[test]
    fn test_strongly_connected_components_3() {
        let edges = vec![
            Edge::new("n1", "n2"),
            Edge::new("n2", "n1"),
            Edge::new("n2", "n3"),
            Edge::new("n3", "n4"),
            Edge::new("n4", "n5"),
            Edge::new("n5", "n3"),
        ];
        let specs = GraphSpecs::directed_create_missing();
        let graph: Graph<&str, ()> = Graph::new_from_nodes_and_edges(vec![], edges, specs).unwrap();
        let result = components::strongly_connected_components(&graph).unwrap();
        assert_eq!(result.len(), 2);
        let c1 = result.iter().find(|hs| hs.get("n1").is_some()).unwrap();
        assert_eq!(c1, &vec!["n1", "n2"].into_iter().collect());
        let c2 = result.iter().find(|hs| hs.get("n3").is_some()).unwrap();
        assert_eq!(c2, &vec!["n5", "n4", "n3"].into_iter().collect());
    }

    #[test]
    fn test_strongly_connected_components_4() {
        let edges: Vec<Edge<i32, ()>> = vec![
            Edge::new(0, 1),
            Edge::new(1, 6),
            Edge::new(1, 7),
            Edge::new(1, 9),
            Edge::new(2, 8),
            Edge::new(2, 9),
            Edge::new(2, 1),
            Edge::new(3, 4),
            Edge::new(3, 6),
            Edge::new(3, 9),
            Edge::new(4, 7),
            Edge::new(4, 3),
            Edge::new(5, 0),
            Edge::new(5, 3),
            Edge::new(6, 4),
            Edge::new(7, 0),
            Edge::new(8, 7),
        ];
        let specs = GraphSpecs::directed_create_missing();
        let graph: Graph<i32, ()> = Graph::new_from_nodes_and_edges(vec![], edges, specs).unwrap();
        let result = components::strongly_connected_components(&graph).unwrap();
        assert_eq!(result.len(), 5);
        let c1 = result.iter().find(|hs| hs.get(&0).is_some()).unwrap();
        assert_eq!(c1, &vec![0, 1, 3, 4, 6, 7].into_iter().collect());
    }

    #[test]
    fn test_number_of_connected_components_1() {
        let edges = vec![
            Edge::new("n1", "n2"),
            Edge::new("n2", "n3"),
            Edge::new("n3", "n4"),
            Edge::new("o1", "o2"),
            Edge::new("p1", "p2"),
            Edge::new("p2", "p3"),
        ];
        let specs = GraphSpecs::undirected_create_missing();
        let graph: Graph<&str, ()> = Graph::new_from_nodes_and_edges(vec![], edges, specs).unwrap();
        let result = components::number_of_connected_components(&graph).unwrap();
        assert_eq!(result, 3);
    }

    #[test]
    fn test_node_connected_component_1() {
        let edges = vec![
            Edge::new("n1", "n2"),
            Edge::new("n2", "n3"),
            Edge::new("n3", "n4"),
            Edge::new("o1", "o2"),
            Edge::new("p1", "p2"),
            Edge::new("p2", "p3"),
        ];
        let specs = GraphSpecs::undirected_create_missing();
        let graph: Graph<&str, ()> = Graph::new_from_nodes_and_edges(vec![], edges, specs).unwrap();
        let result = components::node_connected_component(&graph, &"n2").unwrap();
        assert_eq!(result.len(), 4);
        let result = components::node_connected_component(&graph, &"o1").unwrap();
        assert_eq!(result.len(), 2);
        let result = components::node_connected_component(&graph, &"p3").unwrap();
        assert_eq!(result.len(), 3);
    }

}
