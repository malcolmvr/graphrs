mod utility;

#[cfg(test)]
mod tests {

    use graphrs::{algorithms::components, generators, Edge, Graph, GraphSpecs};
    use itertools::Itertools;

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
/*
    #[test]
    fn test_strongly_connected_components_4() {
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
        let result = components::connected_components(&graph).unwrap();
        assert_eq!(result.len(), 3);
        let c1 = result.iter().find(|hs| hs.get("n1").is_some()).unwrap();
        assert_eq!(c1, &vec!["n1", "n2", "n3", "n4"].into_iter().collect());
        let c2 = result.iter().find(|hs| hs.get("o1").is_some()).unwrap();
        assert_eq!(c2, &vec!["o1", "o2"].into_iter().collect());
        let c3 = result.iter().find(|hs| hs.get("p1").is_some()).unwrap();
        assert_eq!(c3, &vec!["p1", "p2", "p3"].into_iter().collect());
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
*/
}
