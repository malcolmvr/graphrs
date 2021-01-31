#[cfg(test)]
mod tests {

    use graphrs::{Edge, EdgeDedupeStrategy, Graph, GraphSpecs, MissingNodeStrategy, Node};
    use itertools::Itertools;
    use std::collections::HashSet;
    use std::iter::FromIterator;

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
        let weights: f64 = result.unwrap().into_iter().map(|e| e.weight.unwrap()).sum();
        assert_eq!(weights, 3.0);

        let result = graph.get_edges("n1", "n3");
        assert!(result.is_ok());
        let weights: f64 = result.unwrap().into_iter().map(|e| e.weight.unwrap()).sum();
        assert_eq!(weights, 3.2);
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
            Edge::with_weight("n1", "n2", &1.0),
            Edge::with_weight("n1", "n2", &2.0),
            Edge::with_weight("n1", "n3", &1.1),
            Edge::with_weight("n3", "n1", &2.1),
            Edge::with_weight("n4", "n2", &3.0),
        ];

        let final_specs = match specs {
            Some(s) => s,
            None => GraphSpecs::multi_undirected(),
        };

        Graph::new_from_nodes_and_edges(nodes, edges, final_specs).unwrap()
    }
}
