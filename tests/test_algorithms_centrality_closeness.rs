mod utility;

#[cfg(test)]
mod tests {

    use super::utility::round;
    use graphrs::{algorithms::centrality::closeness, generators, Edge, Graph, GraphSpecs};

    #[test]
    fn test_closeness_centrality_1() {
        // directed, weighted, not wf_improved
        let graph = get_graph_1(true);
        let result = closeness::closeness_centrality(&graph, true, false).unwrap();
        assert_eq!(round(result.get("n1").unwrap(), 2), 0.0);
        assert_eq!(round(result.get("n2").unwrap(), 2), 1.0);
        assert_eq!(round(result.get("n3").unwrap(), 2), 0.23);
        assert_eq!(round(result.get("n4").unwrap(), 2), 0.5);
        assert_eq!(round(result.get("n5").unwrap(), 2), 0.24);
    }

    #[test]
    fn test_closeness_centrality_2() {
        // directed, weighted, wf_improved
        let graph = get_graph_1(true);
        let result = closeness::closeness_centrality(&graph, true, true).unwrap();
        assert_eq!(round(result.get("n1").unwrap(), 2), 0.0);
        assert_eq!(round(result.get("n2").unwrap(), 2), 0.25);
        assert_eq!(round(result.get("n3").unwrap(), 2), 0.17);
        assert_eq!(round(result.get("n4").unwrap(), 2), 0.13);
        assert_eq!(round(result.get("n5").unwrap(), 2), 0.24);
    }

    #[test]
    fn test_closeness_centrality_3() {
        // directed, unweighted, not wf_improved
        let graph = get_graph_1(true);
        let result = closeness::closeness_centrality(&graph, false, false).unwrap();
        assert_eq!(round(result.get("n1").unwrap(), 2), 0.0);
        assert_eq!(round(result.get("n2").unwrap(), 2), 1.0);
        assert_eq!(round(result.get("n3").unwrap(), 2), 0.75);
        assert_eq!(round(result.get("n4").unwrap(), 2), 1.0);
        assert_eq!(round(result.get("n5").unwrap(), 2), 0.67);
    }

    #[test]
    fn test_betweenness_centrality_4() {
        // directed, unweighted, wf_improved
        let graph = get_graph_1(true);
        let result = closeness::closeness_centrality(&graph, false, true).unwrap();
        assert_eq!(round(result.get("n1").unwrap(), 2), 0.0);
        assert_eq!(round(result.get("n2").unwrap(), 2), 0.25);
        assert_eq!(round(result.get("n3").unwrap(), 2), 0.56);
        assert_eq!(round(result.get("n4").unwrap(), 2), 0.25);
        assert_eq!(round(result.get("n5").unwrap(), 2), 0.67);
    }

    #[test]
    fn test_betweenness_centrality_5() {
        // undirected, weighted, not wf_improved
        let graph = get_graph_1(false);
        let result = closeness::closeness_centrality(&graph, true, false).unwrap();
        assert_eq!(round(result.get("n1").unwrap(), 2), 0.29);
        assert_eq!(round(result.get("n2").unwrap(), 2), 0.27);
        assert_eq!(round(result.get("n3").unwrap(), 2), 0.29);
        assert_eq!(round(result.get("n4").unwrap(), 2), 0.33);
        assert_eq!(round(result.get("n5").unwrap(), 2), 0.24);
    }

    #[test]
    fn test_betweenness_centrality_6() {
        // undirected, weighted, wf_improved
        let graph = get_graph_1(false);
        let result = closeness::closeness_centrality(&graph, true, true).unwrap();
        assert_eq!(round(result.get("n1").unwrap(), 2), 0.29);
        assert_eq!(round(result.get("n2").unwrap(), 2), 0.27);
        assert_eq!(round(result.get("n3").unwrap(), 2), 0.29);
        assert_eq!(round(result.get("n4").unwrap(), 2), 0.33);
        assert_eq!(round(result.get("n5").unwrap(), 2), 0.24);
    }

    #[test]
    fn test_betweenness_centrality_7() {
        // undirected, unweighted, not wf_improved
        let graph = get_graph_1(false);
        let result = closeness::closeness_centrality(&graph, false, false).unwrap();
        assert_eq!(round(result.get("n1").unwrap(), 2), 0.8);
        assert_eq!(round(result.get("n2").unwrap(), 2), 0.67);
        assert_eq!(round(result.get("n3").unwrap(), 2), 0.8);
        assert_eq!(round(result.get("n4").unwrap(), 2), 0.67);
        assert_eq!(round(result.get("n5").unwrap(), 2), 0.67);
    }

    #[test]
    fn test_betweenness_centrality_8() {
        // undirected, unweighted, wf_improved
        let graph = get_graph_1(false);
        let result = closeness::closeness_centrality(&graph, false, true).unwrap();
        assert_eq!(round(result.get("n1").unwrap(), 2), 0.8);
        assert_eq!(round(result.get("n2").unwrap(), 2), 0.67);
        assert_eq!(round(result.get("n3").unwrap(), 2), 0.8);
        assert_eq!(round(result.get("n4").unwrap(), 2), 0.67);
        assert_eq!(round(result.get("n5").unwrap(), 2), 0.67);
    }

    #[test]
    fn test_betweenness_centrality_9() {
        // karate club, unweighted, not wf_improved
        let graph = generators::social::karate_club_graph();
        let result = closeness::closeness_centrality(&graph, false, true).unwrap();
        assert_eq!(round(result.get(&0).unwrap(), 2), 0.57);
        assert_eq!(round(result.get(&1).unwrap(), 2), 0.49);
        assert_eq!(round(result.get(&2).unwrap(), 2), 0.56);
        assert_eq!(round(result.get(&3).unwrap(), 2), 0.46);
        assert_eq!(round(result.get(&4).unwrap(), 2), 0.38);
        assert_eq!(round(result.get(&5).unwrap(), 2), 0.38);
        assert_eq!(round(result.get(&6).unwrap(), 2), 0.38);
        assert_eq!(round(result.get(&7).unwrap(), 2), 0.44);
        assert_eq!(round(result.get(&8).unwrap(), 2), 0.52);
        assert_eq!(round(result.get(&9).unwrap(), 2), 0.43);
        assert_eq!(round(result.get(&10).unwrap(), 2), 0.38);
        assert_eq!(round(result.get(&11).unwrap(), 2), 0.37);
        assert_eq!(round(result.get(&12).unwrap(), 2), 0.37);
        assert_eq!(round(result.get(&13).unwrap(), 2), 0.52);
        assert_eq!(round(result.get(&14).unwrap(), 2), 0.37);
        assert_eq!(round(result.get(&15).unwrap(), 2), 0.37);
        assert_eq!(round(result.get(&16).unwrap(), 2), 0.28);
        assert_eq!(round(result.get(&17).unwrap(), 2), 0.38);
        assert_eq!(round(result.get(&18).unwrap(), 2), 0.37);
        assert_eq!(round(result.get(&19).unwrap(), 2), 0.5);
        assert_eq!(round(result.get(&20).unwrap(), 2), 0.37);
        assert_eq!(round(result.get(&21).unwrap(), 2), 0.38);
        assert_eq!(round(result.get(&22).unwrap(), 2), 0.37);
        assert_eq!(round(result.get(&23).unwrap(), 2), 0.39);
        assert_eq!(round(result.get(&24).unwrap(), 2), 0.38);
        assert_eq!(round(result.get(&25).unwrap(), 2), 0.38);
        assert_eq!(round(result.get(&26).unwrap(), 2), 0.36);
        assert_eq!(round(result.get(&27).unwrap(), 2), 0.46);
        assert_eq!(round(result.get(&28).unwrap(), 2), 0.45);
        assert_eq!(round(result.get(&29).unwrap(), 2), 0.38);
        assert_eq!(round(result.get(&30).unwrap(), 2), 0.46);
        assert_eq!(round(result.get(&31).unwrap(), 2), 0.54);
        assert_eq!(round(result.get(&32).unwrap(), 2), 0.52);
        assert_eq!(round(result.get(&33).unwrap(), 2), 0.55);
    }

    #[test]
    fn test_betweenness_centrality_10() {
        // disconnected, not wf_improved
        let edges = vec![
            Edge::with_weight("n1", "n2", 1.0),
            Edge::with_weight("n2", "n3", 5.0),
            Edge::with_weight("n1", "n4", 2.0),
            Edge::with_weight("n4", "n3", 3.0),
            Edge::with_weight("n1", "n5", 9.0),
            Edge::with_weight("n3", "n5", 1.0),
            Edge::with_weight("n6", "n7", 1.0),
            Edge::with_weight("n7", "n8", 1.0),
        ];
        let graph: Graph<&str, ()> =
            Graph::new_from_nodes_and_edges(vec![], edges, GraphSpecs::undirected_create_missing())
                .unwrap();
        let result = closeness::closeness_centrality(&graph, true, false).unwrap();
        assert_eq!(round(result.get("n1").unwrap(), 2), 0.29);
        assert_eq!(round(result.get("n2").unwrap(), 2), 0.27);
        assert_eq!(round(result.get("n3").unwrap(), 2), 0.29);
        assert_eq!(round(result.get("n4").unwrap(), 2), 0.33);
        assert_eq!(round(result.get("n5").unwrap(), 2), 0.24);
        assert_eq!(round(result.get("n6").unwrap(), 2), 0.67);
        assert_eq!(round(result.get("n7").unwrap(), 2), 1.0);
        assert_eq!(round(result.get("n8").unwrap(), 2), 0.67);
    }

    #[test]
    fn test_betweenness_centrality_11() {
        // disconnected, wf_improved
        let edges = vec![
            Edge::with_weight("n1", "n2", 1.0),
            Edge::with_weight("n2", "n3", 5.0),
            Edge::with_weight("n1", "n4", 2.0),
            Edge::with_weight("n4", "n3", 3.0),
            Edge::with_weight("n1", "n5", 9.0),
            Edge::with_weight("n3", "n5", 1.0),
            Edge::with_weight("n6", "n7", 1.0),
            Edge::with_weight("n7", "n8", 1.0),
        ];
        let graph: Graph<&str, ()> =
            Graph::new_from_nodes_and_edges(vec![], edges, GraphSpecs::undirected_create_missing())
                .unwrap();
        let result = closeness::closeness_centrality(&graph, true, true).unwrap();
        assert_eq!(round(result.get("n1").unwrap(), 2), 0.16);
        assert_eq!(round(result.get("n2").unwrap(), 2), 0.15);
        assert_eq!(round(result.get("n3").unwrap(), 2), 0.16);
        assert_eq!(round(result.get("n4").unwrap(), 2), 0.19);
        assert_eq!(round(result.get("n5").unwrap(), 2), 0.13);
        assert_eq!(round(result.get("n6").unwrap(), 2), 0.19);
        assert_eq!(round(result.get("n7").unwrap(), 2), 0.29);
        assert_eq!(round(result.get("n8").unwrap(), 2), 0.19);
    }

    fn get_graph_1<'a>(directed: bool) -> Graph<&'a str, ()> {
        let edges = vec![
            Edge::with_weight("n1", "n2", 1.0),
            Edge::with_weight("n2", "n3", 5.0),
            Edge::with_weight("n1", "n4", 2.0),
            Edge::with_weight("n4", "n3", 3.0),
            Edge::with_weight("n1", "n5", 9.0),
            Edge::with_weight("n3", "n5", 1.0),
        ];
        let graph: Graph<&str, ()> = Graph::new_from_nodes_and_edges(
            vec![],
            edges,
            GraphSpecs {
                directed,
                ..GraphSpecs::directed_create_missing()
            },
        )
        .unwrap();
        graph
    }
}
