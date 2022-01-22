#[cfg(test)]
mod tests {

    use graphrs::{Edge, Graph, GraphSpecs};

    #[test]
    fn test_get_edges_for_node_1() {
        let edges = vec![Edge::new("n1", "n2")];
        let graph: Graph<&str, ()> =
            Graph::new_from_nodes_and_edges(vec![], edges, GraphSpecs::undirected_create_missing())
                .unwrap();
        assert!(graph.get_edges_for_node("n0").is_err());
        let n1_in_edges = graph.get_edges_for_node("n1").unwrap();
        assert_eq!(n1_in_edges.len(), 1);
        assert_eq!(n1_in_edges[0].clone(), Edge::new("n1", "n2"));
        let n2_in_edges = graph.get_edges_for_node("n2").unwrap();
        assert_eq!(n2_in_edges.len(), 1);
        assert_eq!(n2_in_edges[0].clone(), Edge::new("n1", "n2"));
    }

    #[test]
    fn test_get_in_edges_for_node_1() {
        let edges = vec![Edge::new("n1", "n2")];
        let graph: Graph<&str, ()> =
            Graph::new_from_nodes_and_edges(vec![], edges, GraphSpecs::directed_create_missing())
                .unwrap();
        assert!(graph.get_in_edges_for_node("n0").is_err());
        assert_eq!(graph.get_in_edges_for_node("n1").unwrap().len(), 0);
        let n2_in_edges = graph.get_in_edges_for_node("n2").unwrap();
        assert_eq!(n2_in_edges.len(), 1);
        assert_eq!(n2_in_edges[0].clone(), Edge::new("n1", "n2"));
    }

    #[test]
    fn test_get_in_edges_for_node_2() {
        let edges = vec![
            Edge::new("n1", "n2"),
            Edge::new("n1", "n2"),
            Edge::new("n3", "n2"),
        ];
        let specs = GraphSpecs {
            multi_edges: true,
            ..GraphSpecs::directed_create_missing()
        };
        let graph: Graph<&str, ()> = Graph::new_from_nodes_and_edges(vec![], edges, specs).unwrap();
        assert_eq!(graph.get_in_edges_for_node("n1").unwrap().len(), 0);
        let mut n2_in_edges = graph.get_in_edges_for_node("n2").unwrap();
        n2_in_edges.sort();
        assert_eq!(n2_in_edges.len(), 3);
        assert_eq!(n2_in_edges[0].clone(), Edge::new("n1", "n2"));
        assert_eq!(n2_in_edges[1].clone(), Edge::new("n1", "n2"));
        assert_eq!(n2_in_edges[2].clone(), Edge::new("n3", "n2"));
        assert_eq!(graph.get_in_edges_for_node("n3").unwrap().len(), 0);
    }

    #[test]
    fn test_get_out_edges_for_node_1() {
        let edges = vec![Edge::new("n1", "n2")];
        let graph: Graph<&str, ()> =
            Graph::new_from_nodes_and_edges(vec![], edges, GraphSpecs::directed_create_missing())
                .unwrap();
        assert!(graph.get_out_edges_for_node("n0").is_err());
        let n1_out_edges = graph.get_out_edges_for_node("n1").unwrap();
        assert_eq!(n1_out_edges.len(), 1);
        assert_eq!(n1_out_edges[0].clone(), Edge::new("n1", "n2"));
        assert_eq!(graph.get_out_edges_for_node("n2").unwrap().len(), 0);
    }

    #[test]
    fn test_get_out_edges_for_node_2() {
        let edges = vec![
            Edge::new("n1", "n2"),
            Edge::new("n1", "n2"),
            Edge::new("n3", "n2"),
        ];
        let specs = GraphSpecs {
            multi_edges: true,
            ..GraphSpecs::directed_create_missing()
        };
        let graph: Graph<&str, ()> = Graph::new_from_nodes_and_edges(vec![], edges, specs).unwrap();
        let mut n1_out_edges = graph.get_out_edges_for_node("n1").unwrap();
        n1_out_edges.sort();
        assert_eq!(n1_out_edges.len(), 2);
        assert_eq!(n1_out_edges[0].clone(), Edge::new("n1", "n2"));
        assert_eq!(n1_out_edges[1].clone(), Edge::new("n1", "n2"));
        assert_eq!(graph.get_out_edges_for_node("n2").unwrap().len(), 0);
        let n3_out_edges = graph.get_out_edges_for_node("n3").unwrap();
        assert_eq!(n3_out_edges.len(), 1);
        assert_eq!(n3_out_edges[0].clone(), Edge::new("n3", "n2"));
    }
}
