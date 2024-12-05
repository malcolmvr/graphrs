#[cfg(test)]
mod tests {

    use graphrs::{
        Edge, EdgeDedupeStrategy, Graph, GraphSpecs, MissingNodeStrategy, Node,
        SelfLoopsFalseStrategy,
    };
    use itertools::Itertools;
    use std::collections::HashSet;
    use std::iter::FromIterator;
    use std::sync::Arc;

    #[test]
    fn test_add_edge_1() {
        let mut graph: Graph<&str, ()> = Graph::new(GraphSpecs::directed());
        let result = graph.add_edge(Edge::new("n1", "n2"));
        assert!(result.is_err());
    }

    #[test]
    fn test_add_edge_2() {
        let mut graph: Graph<&str, ()> = Graph::new(GraphSpecs {
            missing_node_strategy: MissingNodeStrategy::Create,
            ..GraphSpecs::directed()
        });
        let result = graph.add_edge(Edge::new("n1", "n2"));
        assert!(result.is_ok());
        assert!(graph.get_edge("n2", "n1").is_err());
        assert!(graph.get_edge("n1", "n2").is_ok());
        assert_eq!(graph.get_all_edges().len(), 1);
        assert_eq!(graph.get_predecessor_nodes("n1").unwrap().len(), 0);
        assert_eq!(graph.get_predecessor_nodes("n2").unwrap().len(), 1);
        assert_eq!(graph.get_predecessor_nodes("n2").unwrap()[0].name, "n1");
        assert_eq!(graph.get_successor_nodes("n1").unwrap().len(), 1);
        assert_eq!(graph.get_successor_nodes("n1").unwrap()[0].name, "n2");
        assert_eq!(graph.get_successor_nodes("n2").unwrap().len(), 0);
    }

    #[test]
    fn test_add_edge_3() {
        let mut graph: Graph<&str, ()> = Graph::new(GraphSpecs {
            missing_node_strategy: MissingNodeStrategy::Create,
            ..GraphSpecs::directed()
        });
        let result = graph.add_edge(Edge::new("n1", "n2"));
        assert!(result.is_ok());
        let result = graph.add_edge(Edge::new("n1", "n3"));
        assert!(result.is_ok());

        assert_eq!(graph.get_all_edges().len(), 2);
        assert!(graph.get_edge("n1", "n2").is_ok());
        assert!(graph.get_edge("n2", "n1").is_err());
        assert!(graph.get_edge("n1", "n3").is_ok());

        assert_eq!(graph.get_predecessor_nodes("n1").unwrap().len(), 0);
        assert_eq!(graph.get_predecessor_nodes("n2").unwrap().len(), 1);
        assert_eq!(graph.get_predecessor_nodes("n2").unwrap()[0].name, "n1");
        assert_eq!(graph.get_predecessor_nodes("n3").unwrap().len(), 1);
        assert_eq!(graph.get_predecessor_nodes("n3").unwrap()[0].name, "n1");

        assert_eq!(graph.get_successor_nodes("n1").unwrap().len(), 2);
        assert_eq!(
            graph
                .get_successor_nodes("n1")
                .unwrap()
                .iter()
                .map(|n| n.name)
                .sorted()
                .collect::<Vec<&str>>(),
            vec!["n2", "n3"]
        );
        assert_eq!(graph.get_successor_nodes("n2").unwrap().len(), 0);
        assert_eq!(graph.get_successor_nodes("n3").unwrap().len(), 0);
    }

    #[test]
    fn test_add_edge_4() {
        let mut graph: Graph<&str, ()> = Graph::new(GraphSpecs {
            edge_dedupe_strategy: EdgeDedupeStrategy::Error,
            missing_node_strategy: MissingNodeStrategy::Create,
            ..GraphSpecs::directed()
        });
        let result = graph.add_edge(Edge::new("n1", "n2"));
        assert!(result.is_ok());
        let result = graph.add_edge(Edge::new("n1", "n2"));
        assert!(result.is_err());
    }

    #[test]
    fn test_add_edge_5() {
        let mut graph: Graph<&str, ()> = Graph::new(GraphSpecs {
            edge_dedupe_strategy: EdgeDedupeStrategy::KeepFirst,
            missing_node_strategy: MissingNodeStrategy::Create,
            ..GraphSpecs::directed()
        });
        let result = graph.add_edge(Edge::with_weight("n1", "n2", 1.0));
        assert!(result.is_ok());
        let result = graph.add_edge(Edge::with_weight("n1", "n2", 2.0));
        assert!(result.is_ok());
        assert!(graph.get_edge("n1", "n2").is_ok());
        assert_eq!(graph.get_edge("n1", "n2").unwrap().weight, 1.0);
    }

    #[test]
    fn test_add_edge_6() {
        let mut graph: Graph<&str, ()> = Graph::new(GraphSpecs {
            edge_dedupe_strategy: EdgeDedupeStrategy::KeepLast,
            missing_node_strategy: MissingNodeStrategy::Create,
            ..GraphSpecs::directed()
        });
        let result = graph.add_edge(Edge::with_weight("n1", "n2", 1.0));
        assert!(result.is_ok());
        let result = graph.add_edge(Edge::with_weight("n1", "n2", 2.0));
        assert!(result.is_ok());
        assert!(graph.get_edge("n1", "n2").is_ok());
        assert_eq!(graph.get_edge("n1", "n2").unwrap().weight, 2.0);
    }

    #[test]
    fn test_add_edge_7() {
        let mut graph: Graph<&str, ()> = Graph::new(GraphSpecs {
            self_loops: false,
            missing_node_strategy: MissingNodeStrategy::Create,
            ..GraphSpecs::directed()
        });
        let result = graph.add_edge(Edge::new("n1", "n1"));
        assert!(result.is_err());
    }

    #[test]
    fn test_add_edge_8() {
        let mut graph: Graph<&str, ()> = Graph::new(GraphSpecs {
            self_loops: false,
            self_loops_false_strategy: SelfLoopsFalseStrategy::Drop,
            missing_node_strategy: MissingNodeStrategy::Create,
            ..GraphSpecs::directed()
        });
        let result = graph.add_edge(Edge::new("n1", "n1"));
        assert!(result.is_ok());
        assert_eq!(graph.get_all_edges().len(), 0);
    }

    #[test]
    fn test_add_edge_9() {
        let mut graph: Graph<&str, ()> = Graph::new(GraphSpecs {
            self_loops: true,
            missing_node_strategy: MissingNodeStrategy::Create,
            ..GraphSpecs::directed()
        });
        let result = graph.add_edge(Edge::new("n1", "n1"));
        assert!(result.is_ok());
    }

    #[test]
    fn test_add_edges_1() {
        let mut graph: Graph<&str, ()> = Graph::new(GraphSpecs {
            missing_node_strategy: MissingNodeStrategy::Create,
            ..GraphSpecs::directed()
        });
        let result = graph.add_edges(vec![Edge::new("n1", "n2"), Edge::new("n2", "n3")]);
        assert!(result.is_ok());
        assert!(graph.get_edge("n1", "n2").is_ok());
        assert!(graph.get_edge("n2", "n3").is_ok());
    }

    #[test]
    fn test_add_nodes() {
        let nodes = vec![Node::from_name_and_attributes("n1", 100)];

        let mut graph =
            Graph::new_from_nodes_and_edges(vec![], vec![], GraphSpecs::directed()).unwrap();
        graph.add_nodes(nodes);

        assert_eq!(graph.get_all_nodes().len(), 1);

        let node = graph.get_node("n1").unwrap();
        assert_eq!(node.attributes.unwrap(), 100);

        let nodes = vec![
            Node::from_name_and_attributes("n1", 100),
            Node::from_name_and_attributes("n10", 110),
            Node::from_name_and_attributes("n11", 111),
        ];

        graph.add_nodes(nodes);
        assert_eq!(graph.get_all_nodes().len(), 3);
        assert_eq!(graph.get_node("n1").unwrap().attributes.unwrap(), 100);
        assert_eq!(graph.get_node("n10").unwrap().attributes.unwrap(), 110);
        assert_eq!(graph.get_node("n11").unwrap().attributes.unwrap(), 111);
    }

    #[test]
    fn test_get_edge() {
        let graph = get_basic_graph(None);

        let result = graph.get_edge("n1", "n2");
        assert!(result.is_ok());
        let edge = result.unwrap();
        assert_eq!(edge.u, "n1");
        assert_eq!(edge.v, "n2");
        assert_eq!(edge.weight, 1.0);

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
        let expected: Vec<&Arc<Node<&str, ()>>> = vec![];
        assert_eq!(nodes.unwrap(), expected);
    }

    #[test]
    fn test_get_successors_or_neighbors() {
        let graph = get_basic_graph(None);

        let nodes = graph.get_successors_or_neighbors("n1");
        let hashset = HashSet::<&str>::from_iter(nodes.iter().map(|n| n.name));
        assert_eq!(nodes.len(), 2);
        assert!(hashset.contains("n2"));
        assert!(hashset.contains("n3"));

        let nodes = graph.get_successors_or_neighbors("n2");
        let hashset = HashSet::<&str>::from_iter(nodes.iter().map(|n| n.name));
        assert_eq!(nodes.len(), 2);
        assert!(hashset.contains("n1"));
        assert!(hashset.contains("n3"));

        let nodes = graph.get_successors_or_neighbors("n3");
        let expected: Vec<&Arc<Node<&str, ()>>> = vec![];
        assert_eq!(nodes, expected);
    }

    #[test]
    fn test_get_reverse() {
        let graph = get_basic_graph(None);
        let result = graph.reverse();
        assert!(result.is_ok());
        let graph = result.unwrap();
        assert_eq!(graph.get_all_nodes().len(), 3);
        assert_eq!(graph.get_all_edges().len(), 4);
        assert!(graph.get_edge("n1", "n2").is_ok());
        assert!(graph.get_edge("n2", "n1").is_ok());
        assert!(graph.get_edge("n3", "n1").is_ok());
        assert!(graph.get_edge("n3", "n2").is_ok());
        assert!(graph.get_edge("n1", "n3").is_err());
        assert!(graph.get_edge("n2", "n3").is_err());

        let nodes = graph.get_successor_nodes("n3").unwrap();
        let hashset = HashSet::<&str>::from_iter(nodes.iter().map(|n| n.name));
        assert_eq!(nodes.len(), 2);
        assert!(hashset.contains("n1"));
        assert!(hashset.contains("n2"));

        let nodes = graph.get_predecessor_nodes("n1").unwrap();
        let hashset = HashSet::<&str>::from_iter(nodes.iter().map(|n| n.name));
        assert_eq!(nodes.len(), 2);
        assert!(hashset.contains("n2"));
        assert!(hashset.contains("n3"));
    }

    #[test]
    fn test_new_from_nodes_and_edges() {
        let nodes = vec![
            Node::from_name("n1"),
            Node::from_name("n2"),
            Node::<&str, ()>::from_name("n3"),
        ];

        let edges = vec![
            Edge::with_weight("n1", "n2", 1.0),
            Edge::with_weight("n2", "n1", 2.0),
            Edge::with_weight("n1", "n3", 3.0),
        ];

        let specs = GraphSpecs::directed();
        let graph = Graph::new_from_nodes_and_edges(nodes, edges, specs);
        assert!(graph.is_ok());
        let graph = graph.unwrap();

        assert_eq!(graph.get_all_nodes().len(), 3);
        assert_eq!(graph.get_all_edges().len(), 3);
    }

    fn get_basic_graph<'a>(specs: Option<GraphSpecs>) -> Graph<&'a str, ()> {
        let nodes = vec![
            Node::from_name("n1"),
            Node::from_name("n2"),
            Node::from_name("n3"),
        ];

        let edges = vec![
            Edge::with_weight("n1", "n2", 1.0),
            Edge::with_weight("n2", "n1", 2.0),
            Edge::with_weight("n1", "n3", 3.0),
            Edge::with_weight("n2", "n3", 3.0),
        ];

        let final_specs = match specs {
            Some(s) => s,
            None => GraphSpecs::directed(),
        };

        Graph::new_from_nodes_and_edges(nodes, edges, final_specs).unwrap()
    }
}
