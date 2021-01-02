#[cfg(test)]
mod tests {

    // use std::collections::HashMap;
    use graphx::{DiGraph, Node, Edge, MergeStrategy};


    #[test]
    fn test_digraph_new_from_nodes_and_edges() {

        let mut nodes = Vec::new();
        nodes.push(Node::from_name("n1"));
        nodes.push(Node::from_name("n2"));
        nodes.push(Node::<&str, &str, &f64>::from_name("n3"));
        
        let mut edges = Vec::new();
        edges.push(Edge::with_weight("n1", "n2", &1.0));
        edges.push(Edge::with_weight("n2", "n1", &2.0));
        edges.push(Edge::with_weight("n1", "n3", &3.0));

        let graph = DiGraph::new_from_nodes_and_edges(nodes, edges);

        assert_eq!(graph.nodes().len(), 3);
        assert_eq!(graph.edges().len(), 3);

    }


    #[test]
    fn test_digraph_add_or_update_nodes_replace() {

        let graph = get_basic_graph();

        let mut nodes = Vec::new();
        nodes.push(Node::from_name_and_attribute_tuples("n1", vec![("a", &1.0), ("b", &2.0)]));
        let graph = graph.add_or_update_nodes(nodes, MergeStrategy::Replace);

        assert_eq!(graph.nodes()["n1"].attributes.as_ref().unwrap()["a"], &1.0);
        assert_eq!(graph.nodes()["n1"].attributes.as_ref().unwrap()["b"], &2.0);

        let mut nodes = Vec::new();
        nodes.push(Node::from_name_and_attribute_tuples("n1", vec![("a", &2.0)]));
        nodes.push(Node::from_name_and_attribute_tuples("n10", vec![("a", &1.0)]));
        nodes.push(Node::from_name_and_attribute_tuples("n11", vec![("a", &2.0)]));

        let graph = graph.add_or_update_nodes(nodes, MergeStrategy::Replace);

        assert_eq!(graph.nodes().len(), 5);
        assert_eq!(graph.nodes()["n1"].attributes.as_ref().unwrap()["a"], &2.0);
        assert_eq!(graph.nodes()["n1"].attributes.as_ref().unwrap().contains_key("b"), false);

    }


    #[test]
    fn test_digraph_add_or_update_nodes_update() {

        let graph = get_basic_graph();

        let mut nodes = Vec::new();
        nodes.push(Node::from_name_and_attribute_tuples("n1", vec![("a", &1.0), ("b", &2.0)]));
        let graph = graph.add_or_update_nodes(nodes, MergeStrategy::Update);

        assert_eq!(graph.nodes()["n1"].attributes.as_ref().unwrap()["a"], &1.0);
        assert_eq!(graph.nodes()["n1"].attributes.as_ref().unwrap()["b"], &2.0);

        let mut nodes = Vec::new();
        nodes.push(Node::from_name_and_attribute_tuples("n1", vec![("a", &2.0)]));
        nodes.push(Node::from_name_and_attribute_tuples("n10", vec![("a", &1.0)]));
        nodes.push(Node::from_name_and_attribute_tuples("n11", vec![("a", &2.0)]));

        let graph = graph.add_or_update_nodes(nodes, MergeStrategy::Update);

        assert_eq!(graph.nodes().len(), 5);
        assert_eq!(graph.nodes()["n1"].attributes.as_ref().unwrap()["a"], &2.0);
        assert_eq!(graph.nodes()["n1"].attributes.as_ref().unwrap()["b"], &2.0);

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

        DiGraph::<&str, &str, &f64>::new_from_nodes_and_edges(nodes, edges)

    }

}