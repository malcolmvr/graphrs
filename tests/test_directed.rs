#[cfg(test)]
mod tests {

    use graphrs::{Edge, EdgeDedupeStrategy, Graph, GraphSpecs, MissingNodeStrategy, Node};
    use itertools::Itertools;
    use std::collections::HashSet;
    use std::iter::FromIterator;

    #[test]
    fn test_add_edges_add() {
        // test addition of a new edge
        let graph = get_basic_graph(None);

        let new_edges = vec![Edge::with_attribute("n3", "n1", "weight", &4.4)];

        let graph = graph.add_edges(new_edges);
        assert!(graph.is_ok());
        let graph = graph.unwrap();

        let edges = graph.get_all_edges();
        assert_eq!(edges.len(), 5);

        let nodes = graph.get_all_nodes();
        assert_eq!(nodes.len(), 3);
    }

    #[test]
    fn test_add_edges_update_error() {
        // test that when an existing edge is added and EdgeDedupeStrategy is Error
        let graph = get_basic_graph(None);

        let new_edges = vec![Edge::with_attribute("n1", "n3", "weight", &4.4)];

        let graph = graph.add_edges(new_edges);
        assert!(graph.is_err());
    }

    #[test]
    fn test_add_edges_update_keep_first() {
        // test addition of an existing edge is added and EdgeDedupeStrategy is KeepFirst
        let specs = GraphSpecs {
            edge_dedupe_strategy: EdgeDedupeStrategy::KeepFirst,
            ..GraphSpecs::directed()
        };
        let graph = get_basic_graph(Some(specs));

        let new_edges = vec![Edge::with_attribute("n1", "n3", "weight", &4.4)];

        let graph = graph.add_edges(new_edges);
        assert!(graph.is_ok());
        let graph = graph.unwrap();

        let edge = graph.get_edge("n1", "n3");
        assert_eq!(edge.unwrap().attributes.as_ref().unwrap()["weight"], &3.0);
    }

    #[test]
    fn test_add_edges_update_keep_last() {
        // test addition of an existing edge is added and EdgeDedupeStrategy is KeepLast
        let specs = GraphSpecs {
            edge_dedupe_strategy: EdgeDedupeStrategy::KeepLast,
            ..GraphSpecs::directed()
        };
        let graph = get_basic_graph(Some(specs));

        let new_edges = vec![Edge::with_attribute("n1", "n3", "weight", &4.4)];

        let graph = graph.add_edges(new_edges);
        assert!(graph.is_ok());
        let graph = graph.unwrap();

        let edge = graph.get_edge("n1", "n3");
        assert_eq!(edge.unwrap().attributes.as_ref().unwrap()["weight"], &4.4);
    }

    #[test]
    fn test_add_edges_add_nodes_error() {
        // test edge addition with MissingNodeStrategy::Error
        let graph = get_basic_graph(None);

        let new_edges = vec![Edge::with_attribute("n4", "n5", "weight", &4.4)];

        let graph = graph.add_edges(new_edges);
        assert!(graph.is_err());
    }

    #[test]
    fn test_add_edges_add_nodes_create() {
        // test edge addition with MissingNodeStrategy::Create
        let specs = GraphSpecs {
            missing_node_strategy: MissingNodeStrategy::Create,
            ..GraphSpecs::directed()
        };
        let graph = get_basic_graph(Some(specs));

        let new_edges = vec![Edge::with_attribute("n4", "n5", "weight", &4.4)];

        let graph = graph.add_edges(new_edges);
        assert!(graph.is_ok());
        let graph = graph.unwrap();

        let edges = graph.get_all_edges();
        assert_eq!(edges.len(), 5);

        let nodes = graph.get_all_nodes();
        assert_eq!(nodes.len(), 5);
        let names = nodes
            .into_iter()
            .map(|n| n.name)
            .sorted()
            .collect::<Vec<&str>>();
        assert_eq!(names, vec!["n1", "n2", "n3", "n4", "n5"]);
    }

    #[test]
    fn test_add_nodes() {
        let graph = get_basic_graph(None);

        let nodes = vec![Node::from_name_and_attribute_tuples(
            "n1",
            vec![("a", &1.0), ("b", &2.0)],
        )];

        let graph = graph.add_nodes(nodes);
        assert!(graph.is_ok());
        let graph = graph.unwrap();

        let node = graph.get_node("n1").unwrap();
        assert_eq!(node.attributes.as_ref().unwrap()["a"], &1.0);
        assert_eq!(node.attributes.as_ref().unwrap()["b"], &2.0);

        let nodes = vec![
            Node::from_name_and_attribute_tuples("n1", vec![("a", &2.0)]),
            Node::from_name_and_attribute_tuples("n10", vec![("a", &1.0)]),
            Node::from_name_and_attribute_tuples("n11", vec![("a", &2.0)]),
        ];

        let graph = graph.add_nodes(nodes);
        assert!(graph.is_ok());
        let graph = graph.unwrap();

        let node = graph.get_node("n1").unwrap();
        assert_eq!(graph.get_all_nodes().len(), 5);
        assert_eq!(node.attributes.as_ref().unwrap()["a"], &2.0);
        assert!(!node.attributes.as_ref().unwrap().contains_key("b"));
    }

    #[test]
    fn test_get_edge() {
        let graph = get_basic_graph(None);

        let result = graph.get_edge("n1", "n2");
        assert!(result.is_ok());
        let edge = result.unwrap();
        assert_eq!(edge.u, "n1");
        assert_eq!(edge.v, "n2");

        let edge = graph.get_edge("n4", "n5");
        assert!(edge.is_err());
    }

    #[test]
    fn test_get_edges() {
        let graph = get_basic_graph(None);

        let result = graph.get_edges("n1", "n2");
        assert!(result.is_err());
    }

    #[test]
    fn test_get_node() {
        let graph = get_basic_graph(None);

        let node = graph.get_node("n1");
        assert!(node.is_some());
        assert_eq!(node.unwrap().name, "n1");

        let node = graph.get_node("n4");
        assert!(node.is_none());
    }

    #[test]
    fn test_get_neighbor_nodes() {
        let graph = get_basic_graph(None);

        let nodes = graph.get_neighbor_nodes("n1").unwrap();
        let hashset = HashSet::<&str>::from_iter(nodes.iter().map(|n| n.name));
        assert_eq!(nodes.len(), 2);
        assert!(hashset.contains("n2"));
        assert!(hashset.contains("n3"));

        let nodes = graph.get_neighbor_nodes("n2").unwrap();
        let hashset = HashSet::<&str>::from_iter(nodes.iter().map(|n| n.name));
        assert_eq!(nodes.len(), 2);
        assert!(hashset.contains("n1"));
        assert!(hashset.contains("n3"));

        let nodes = graph.get_neighbor_nodes("n3").unwrap();
        let hashset = HashSet::<&str>::from_iter(nodes.iter().map(|n| n.name));
        assert_eq!(nodes.len(), 2);
        assert!(hashset.contains("n1"));
        assert!(hashset.contains("n2"));

        let nodes = graph.get_neighbor_nodes("n4");
        assert!(nodes.is_err());
    }

    #[test]
    fn test_get_predecessor_nodes() {
        let graph = get_basic_graph(None);

        let nodes = graph.get_predecessor_nodes("n1").unwrap();
        assert_eq!(nodes.len(), 1);
        assert_eq!(nodes[0].name, "n2");

        let nodes = graph.get_predecessor_nodes("n2").unwrap();
        assert_eq!(nodes.len(), 1);
        assert_eq!(nodes[0].name, "n1");

        let nodes = graph.get_predecessor_nodes("n3").unwrap();
        let hashset = HashSet::<&str>::from_iter(nodes.iter().map(|n| n.name));
        assert_eq!(nodes.len(), 2);
        assert!(hashset.contains("n1"));
        assert!(hashset.contains("n2"));
    }

    #[test]
    fn test_get_predecessors_map() {
        let graph = get_basic_graph(None);

        let map = graph.get_predecessors_map();
        assert_eq!(map.len(), 3);
        assert!(map.contains_key("n1"));
        assert!(map.contains_key("n2"));
        assert!(map.contains_key("n3"));

        let set = map.get("n1").unwrap();
        assert_eq!(set.len(), 1);
        assert!(set.contains("n2"));

        let set = map.get("n2").unwrap();
        assert_eq!(set.len(), 1);
        assert!(set.contains("n1"));

        let set = map.get("n3").unwrap();
        assert_eq!(set.len(), 2);
        assert!(set.contains("n1"));
        assert!(set.contains("n2"));
    }

    #[test]
    fn test_get_successors_map() {
        let graph = get_basic_graph(None);

        let map = graph.get_successors_map();
        assert_eq!(map.len(), 2);
        assert!(map.contains_key("n1"));
        assert!(map.contains_key("n2"));

        let set = map.get("n1").unwrap();
        assert_eq!(set.len(), 2);
        assert!(set.contains("n2"));
        assert!(set.contains("n3"));

        let set = map.get("n2").unwrap();
        assert_eq!(set.len(), 2);
        assert!(set.contains("n1"));
        assert!(set.contains("n3"));

        let set = map.get("n3");
        assert!(set.is_none());
    }

    #[test]
    fn test_get_successor_nodes() {
        let graph = get_basic_graph(None);

        let nodes = graph.get_successor_nodes("n1").unwrap();
        let hashset = HashSet::<&str>::from_iter(nodes.iter().map(|n| n.name));
        assert_eq!(nodes.len(), 2);
        assert!(hashset.contains("n2"));
        assert!(hashset.contains("n3"));

        let nodes = graph.get_successor_nodes("n2").unwrap();
        let hashset = HashSet::<&str>::from_iter(nodes.iter().map(|n| n.name));
        assert_eq!(nodes.len(), 2);
        assert!(hashset.contains("n1"));
        assert!(hashset.contains("n3"));

        let nodes = graph.get_successor_nodes("n3");
        let expected: Vec<&Node<&str, &str, &f64>> = vec![];
        assert_eq!(nodes.unwrap(), expected);
    }

    #[test]
    fn test_new_from_nodes_and_edges() {
        let nodes = vec![
            Node::from_name("n1"),
            Node::from_name("n2"),
            Node::<&str, &str, &f64>::from_name("n3"),
        ];

        let edges = vec![
            Edge::with_attribute("n1", "n2", "weight", &1.0),
            Edge::with_attribute("n2", "n1", "weight", &2.0),
            Edge::with_attribute("n1", "n3", "weight", &3.0),
        ];

        let specs = GraphSpecs::directed();
        let graph = Graph::new_from_nodes_and_edges(nodes, edges, specs);
        assert!(graph.is_ok());
        let graph = graph.unwrap();

        assert_eq!(graph.get_all_nodes().len(), 3);
        assert_eq!(graph.get_all_edges().len(), 3);
    }

    fn get_basic_graph<'a>(specs: Option<GraphSpecs>) -> Graph<&'a str, &'a str, &'a f64> {
        let nodes = vec![
            Node::from_name("n1"),
            Node::from_name("n2"),
            Node::from_name("n3"),
        ];

        let edges = vec![
            Edge::with_attribute("n1", "n2", "weight", &1.0),
            Edge::with_attribute("n2", "n1", "weight", &2.0),
            Edge::with_attribute("n1", "n3", "weight", &3.0),
            Edge::with_attribute("n2", "n3", "weight", &3.0),
        ];

        let final_specs = match specs {
            Some(s) => s,
            None => GraphSpecs::directed(),
        };

        Graph::new_from_nodes_and_edges(nodes, edges, final_specs).unwrap()
    }
}
