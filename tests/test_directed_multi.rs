#[cfg(test)]
mod tests {

    use graphrs::{Edge, Graph, GraphSpecs, Node};

    #[test]
    fn test_add_edges_add_multi() {
        // test addition of a new edge that already exists
        let graph = get_basic_graph(None);

        let new_edges = vec![Edge::with_attribute("n3", "n1", "weight", &4.4)];

        let graph = graph.add_edges(new_edges);
        assert!(graph.is_ok());
        let graph = graph.unwrap();

        let edges = graph.get_all_edges();
        assert_eq!(edges.len(), 6);
        let weights: f64 = edges
            .into_iter()
            .map(|e| e.attributes.as_ref().unwrap()["weight"])
            .sum();
        assert_eq!((weights * 10.0).round() / 10.0, 13.6);

        let nodes = graph.get_all_nodes();
        assert_eq!(nodes.len(), 4);
    }

    #[test]
    fn test_get_edge() {
        let graph = get_basic_graph(None);

        let result = graph.get_edge("n1", "n2");
        assert!(result.is_err());
    }

    #[test]
    fn test_get_edges() {
        let graph = get_basic_graph(None);

        let result = graph.get_edges("n1", "n2");
        assert!(result.is_ok());
        let edges = result.unwrap();
        assert_eq!(edges.len(), 2);
        let weights: f64 = edges
            .into_iter()
            .map(|e| e.attributes.as_ref().unwrap()["weight"])
            .sum();
        assert_eq!(weights, 3.0);

        let result = graph.get_edges("n1", "n3");
        assert!(result.is_ok());
        let weights: f64 = result
            .unwrap()
            .into_iter()
            .map(|e| e.attributes.as_ref().unwrap()["weight"])
            .sum();
        assert_eq!(weights, 1.1);
    }

    #[test]
    fn test_new_from_nodes_and_edges() {
        let graph = get_basic_graph(None);

        assert_eq!(graph.get_all_nodes().len(), 4);
        assert_eq!(graph.get_all_edges().len(), 5);
    }

    fn get_basic_graph<'a>(specs: Option<GraphSpecs>) -> Graph<&'a str, &'a str, &'a f64> {
        let nodes = vec![
            Node::from_name("n1"),
            Node::from_name("n2"),
            Node::from_name("n3"),
            Node::from_name("n4"),
        ];

        let edges = vec![
            Edge::with_attribute("n1", "n2", "weight", &1.0),
            Edge::with_attribute("n1", "n2", "weight", &2.0),
            Edge::with_attribute("n1", "n3", "weight", &1.1),
            Edge::with_attribute("n3", "n1", "weight", &2.1),
            Edge::with_attribute("n4", "n2", "weight", &3.0),
        ];

        let final_specs = match specs {
            Some(s) => s,
            None => GraphSpecs::multi_directed(),
        };

        Graph::new_from_nodes_and_edges(nodes, edges, final_specs).unwrap()
    }
}
