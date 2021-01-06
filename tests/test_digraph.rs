#[cfg(test)]
mod tests {

    use graphx::{AttributeMergeStrategy, DiGraph, Edge, MissingNodeStrategy, Node};
    use std::collections::HashSet;
    use std::iter::FromIterator;

    #[test]
    fn test_add_or_update_nodes_replace() {
        let graph = get_basic_graph();

        let mut nodes = Vec::new();
        nodes.push(Node::from_name_and_attribute_tuples(
            "n1",
            vec![("a", &1.0), ("b", &2.0)],
        ));

        let graph = graph.add_or_update_nodes(nodes, AttributeMergeStrategy::Replace);
        assert!(graph.is_ok());
        let graph = graph.unwrap();

        let node = graph.get_node(&"n1").unwrap();
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

        let node = graph.get_node(&"n1").unwrap();
        assert_eq!(graph.get_all_nodes().len(), 5);
        assert_eq!(node.attributes.as_ref().unwrap()["a"], &2.0);
        assert_eq!(node.attributes.as_ref().unwrap().contains_key("b"), false);
    }

    #[test]
    fn test_add_or_update_nodes_update() {
        let graph = get_basic_graph();

        let mut nodes = Vec::new();
        nodes.push(Node::from_name_and_attribute_tuples(
            "n1",
            vec![("a", &1.0), ("b", &2.0)],
        ));

        let graph = graph.add_or_update_nodes(nodes, AttributeMergeStrategy::Update);
        assert!(graph.is_ok());
        let graph = graph.unwrap();

        let node = graph.get_node(&"n1").unwrap();
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

        let node = graph.get_node(&"n1").unwrap();
        assert_eq!(graph.get_all_nodes().len(), 5);
        assert_eq!(node.attributes.as_ref().unwrap()["a"], &2.0);
        assert_eq!(node.attributes.as_ref().unwrap()["b"], &2.0);
    }

    #[test]
    fn test_get_predecessor_nodes() {
        let graph = get_basic_graph();

        let nodes = graph.get_predecessor_nodes(&"n1").unwrap();
        assert_eq!(nodes.len(), 1);
        assert_eq!(nodes[0].name, "n2");

        let nodes = graph.get_predecessor_nodes(&"n2").unwrap();
        assert_eq!(nodes.len(), 1);
        assert_eq!(nodes[0].name, "n1");

        let nodes = graph.get_predecessor_nodes(&"n3").unwrap();
        let hashset = HashSet::<&str>::from_iter(nodes.iter().map(|n| n.name));
        assert_eq!(nodes.len(), 2);
        assert!(hashset.contains("n1"));
        assert!(hashset.contains("n2"));
    }

    #[test]
    fn test_get_successor_nodes() {
        let graph = get_basic_graph();

        let nodes = graph.get_successor_nodes(&"n1").unwrap();
        let hashset = HashSet::<&str>::from_iter(nodes.iter().map(|n| n.name));
        assert_eq!(nodes.len(), 2);
        assert!(hashset.contains("n2"));
        assert!(hashset.contains("n3"));

        let nodes = graph.get_successor_nodes(&"n2").unwrap();
        let hashset = HashSet::<&str>::from_iter(nodes.iter().map(|n| n.name));
        assert_eq!(nodes.len(), 2);
        assert!(hashset.contains("n1"));
        assert!(hashset.contains("n3"));

        let nodes = graph.get_successor_nodes(&"n3");
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
