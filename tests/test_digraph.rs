#[cfg(test)]
mod tests {

    use graphx::{AttributeMergeStrategy, DiGraph, Edge, MissingNodeStrategy, Node};
    use itertools::Itertools;
    use std::collections::HashSet;
    use std::iter::FromIterator;

    #[test]
    fn test_add_or_update_edges_add() {
        // test addition of a new edge
        let graph = get_basic_graph();

        let new_edges = vec![Edge::with_weight("n3", "n1", &4.4)];

        let graph = graph.add_or_update_edges(new_edges, MissingNodeStrategy::Error);
        assert!(graph.is_ok());
        let graph = graph.unwrap();

        let edges = graph.get_all_edges();
        assert_eq!(edges.len(), 5);

        let nodes = graph.get_all_nodes();
        assert_eq!(nodes.len(), 3);
    }

    #[test]
    fn test_add_or_update_edges_update() {
        // test that when an existing edge is added again the weight is updated
        let graph = get_basic_graph();

        let new_edges = vec![Edge::with_weight("n1", "n3", &4.4)];

        let graph = graph.add_or_update_edges(new_edges, MissingNodeStrategy::Error);
        assert!(graph.is_ok());
        let graph = graph.unwrap();

        let edge = graph.get_edge("n1", "n3");
        assert_eq!(edge.unwrap().weight.unwrap(), &4.4);
    }

    #[test]
    fn test_add_or_update_edges_add_nodes_error() {
        // test edge addition with MissingNodeStrategy::Error
        let graph = get_basic_graph();

        let new_edges = vec![Edge::with_weight("n4", "n5", &4.4)];

        let graph = graph.add_or_update_edges(new_edges, MissingNodeStrategy::Error);
        assert!(graph.is_err());
    }

    #[test]
    fn test_add_or_update_edges_add_nodes_create() {
        // test edge addition with MissingNodeStrategy::Create
        let graph = get_basic_graph();

        let new_edges = vec![Edge::with_weight("n4", "n5", &4.4)];

        let graph = graph.add_or_update_edges(new_edges, MissingNodeStrategy::Create);
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
    fn test_add_or_update_nodes_replace() {
        // test node addition with replace strategy
        let graph = get_basic_graph();

        let mut nodes = Vec::new();
        nodes.push(Node::from_name_and_attribute_tuples(
            "n1",
            vec![("a", &1.0), ("b", &2.0)],
        ));

        let graph = graph.add_or_update_nodes(nodes, AttributeMergeStrategy::Replace);
        assert!(graph.is_ok());
        let graph = graph.unwrap();

        let node = graph.get_node("n1").unwrap();
        assert_eq!(node.attributes.as_ref().unwrap()["a"], &1.0);
        assert_eq!(node.attributes.as_ref().unwrap()["b"], &2.0);

        let mut nodes = Vec::new();
        nodes.push(Node::from_name_and_attribute_tuples(
            "n1",
            vec![("a", &2.0)],
        ));
        nodes.push(Node::from_name_and_attribute_tuples(
            "n10",
            vec![("a", &1.0)],
        ));
        nodes.push(Node::from_name_and_attribute_tuples(
            "n11",
            vec![("a", &2.0)],
        ));

        let graph = graph.add_or_update_nodes(nodes, AttributeMergeStrategy::Replace);
        assert!(graph.is_ok());
        let graph = graph.unwrap();

        let node = graph.get_node("n1").unwrap();
        assert_eq!(graph.get_all_nodes().len(), 5);
        assert_eq!(node.attributes.as_ref().unwrap()["a"], &2.0);
        assert_eq!(node.attributes.as_ref().unwrap().contains_key("b"), false);
    }

    #[test]
    fn test_add_or_update_nodes_update() {
        // test node addition with update strategy
        let graph = get_basic_graph();

        let mut nodes = Vec::new();
        nodes.push(Node::from_name_and_attribute_tuples(
            "n1",
            vec![("a", &1.0), ("b", &2.0)],
        ));

        let graph = graph.add_or_update_nodes(nodes, AttributeMergeStrategy::Update);
        assert!(graph.is_ok());
        let graph = graph.unwrap();

        let node = graph.get_node("n1").unwrap();
        assert_eq!(node.attributes.as_ref().unwrap()["a"], &1.0);
        assert_eq!(node.attributes.as_ref().unwrap()["b"], &2.0);

        let mut nodes = Vec::new();
        nodes.push(Node::from_name_and_attribute_tuples(
            "n1",
            vec![("a", &2.0)],
        ));
        nodes.push(Node::from_name_and_attribute_tuples(
            "n10",
            vec![("a", &1.0)],
        ));
        nodes.push(Node::from_name_and_attribute_tuples(
            "n11",
            vec![("a", &2.0)],
        ));

        let graph = graph.add_or_update_nodes(nodes, AttributeMergeStrategy::Update);
        assert!(graph.is_ok());
        let graph = graph.unwrap();

        let node = graph.get_node("n1").unwrap();
        assert_eq!(graph.get_all_nodes().len(), 5);
        assert_eq!(node.attributes.as_ref().unwrap()["a"], &2.0);
        assert_eq!(node.attributes.as_ref().unwrap()["b"], &2.0);
    }

    #[test]
    fn test_get_edge() {
        let graph = get_basic_graph();

        let edge = graph.get_edge("n1", "n2");
        assert!(edge.is_some());
        assert_eq!(edge.unwrap().u, "n1");
        assert_eq!(edge.unwrap().v, "n2");

        let edge = graph.get_edge("n4", "n5");
        assert!(edge.is_none());
    }

    #[test]
    fn test_get_node() {
        let graph = get_basic_graph();

        let node = graph.get_node("n1");
        assert!(node.is_some());
        assert_eq!(node.unwrap().name, "n1");

        let node = graph.get_node("n4");
        assert!(node.is_none());
    }

    #[test]
    fn test_get_predecessor_nodes() {
        let graph = get_basic_graph();

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
        let graph = get_basic_graph();

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
        let graph = get_basic_graph();

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
        let graph = get_basic_graph();

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
        assert!(nodes.is_none());
    }

    #[test]
    fn test_new_from_nodes_and_edges() {
        let mut nodes = Vec::new();
        nodes.push(Node::from_name("n1"));
        nodes.push(Node::from_name("n2"));
        nodes.push(Node::<&str, &str, &f64>::from_name("n3"));

        let mut edges = Vec::new();
        edges.push(Edge::with_weight("n1", "n2", &1.0));
        edges.push(Edge::with_weight("n2", "n1", &2.0));
        edges.push(Edge::with_weight("n1", "n3", &3.0));

        let graph = DiGraph::new_from_nodes_and_edges(nodes, edges, MissingNodeStrategy::Error);
        assert!(graph.is_ok());
        let graph = graph.unwrap();

        assert_eq!(graph.get_all_nodes().len(), 3);
        assert_eq!(graph.get_all_edges().len(), 3);
    }

    fn get_basic_graph<'a>() -> DiGraph<&'a str, &'a str, &'a f64> {
        let mut nodes = Vec::new();
        nodes.push(Node::from_name("n1"));
        nodes.push(Node::from_name("n2"));
        nodes.push(Node::<&str, &str, &f64>::from_name("n3"));

        let mut edges = Vec::new();
        edges.push(Edge::with_weight("n1", "n2", &1.0));
        edges.push(Edge::with_weight("n2", "n1", &2.0));
        edges.push(Edge::with_weight("n1", "n3", &3.0));
        edges.push(Edge::with_weight("n2", "n3", &3.0));

        DiGraph::<&str, &str, &f64>::new_from_nodes_and_edges(
            nodes,
            edges,
            MissingNodeStrategy::Error,
        )
        .unwrap()
    }
}
