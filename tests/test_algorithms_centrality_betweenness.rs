mod utility;

#[cfg(test)]
mod tests {

    use super::utility::round;
    use graphrs::{algorithms::centrality::betweenness, generators, Edge, Graph, GraphSpecs};

    #[test]
    fn test_betweenness_centrality_1() {
        // directed, weighted, not normalized
        let graph = get_graph_1(true);
        let result = betweenness::betweenness_centrality(&graph, true, false).unwrap();
        assert_eq!(result.get("n1").unwrap(), &0.0);
        assert_eq!(result.get("n2").unwrap(), &0.0);
        assert_eq!(result.get("n3").unwrap(), &3.0);
        assert_eq!(result.get("n4").unwrap(), &2.0);
        assert_eq!(result.get("n5").unwrap(), &0.0);
    }

    #[test]
    fn test_betweenness_centrality_2() {
        // directed, weighted, normalized
        let graph = get_graph_1(true);
        let result = betweenness::betweenness_centrality(&graph, true, true).unwrap();
        assert_eq!(result.get("n1").unwrap(), &0.0);
        assert_eq!(result.get("n2").unwrap(), &0.0);
        assert_eq!(result.get("n3").unwrap(), &0.25);
        assert_eq!(result.get("n4").unwrap(), &(1.0 / 6.0));
        assert_eq!(result.get("n5").unwrap(), &0.0);
    }

    #[test]
    fn test_betweenness_centrality_3() {
        // directed, unweighted, not normalized
        let graph = get_graph_1(true);
        let result = betweenness::betweenness_centrality(&graph, false, false).unwrap();
        assert_eq!(result.get("n1").unwrap(), &0.0);
        assert_eq!(result.get("n2").unwrap(), &0.5);
        assert_eq!(result.get("n3").unwrap(), &2.0);
        assert_eq!(result.get("n4").unwrap(), &0.5);
        assert_eq!(result.get("n5").unwrap(), &0.0);
    }

    #[test]
    fn test_betweenness_centrality_4() {
        // directed, unweighted, normalized
        let graph = get_graph_1(true);
        let result = betweenness::betweenness_centrality(&graph, false, true).unwrap();
        assert_eq!(result.get("n1").unwrap(), &0.0);
        assert_eq!(result.get("n2").unwrap(), &(1.0 / 24.0));
        assert_eq!(result.get("n3").unwrap(), &(1.0 / 6.0));
        assert_eq!(result.get("n4").unwrap(), &(1.0 / 24.0));
        assert_eq!(result.get("n5").unwrap(), &0.0);
    }

    #[test]
    fn test_betweenness_centrality_5() {
        // undirected, weighted, not normalized
        let graph = get_graph_1(false);
        let result = betweenness::betweenness_centrality(&graph, true, false).unwrap();
        assert_eq!(result.get("n1").unwrap(), &1.0);
        assert_eq!(result.get("n2").unwrap(), &0.0);
        assert_eq!(result.get("n3").unwrap(), &3.0);
        assert_eq!(result.get("n4").unwrap(), &2.0);
        assert_eq!(result.get("n5").unwrap(), &0.0);
    }

    #[test]
    fn test_betweenness_centrality_6() {
        // undirected, weighted, normalized
        let graph = get_graph_1(false);
        let result = betweenness::betweenness_centrality(&graph, true, true).unwrap();
        assert_eq!(result.get("n1").unwrap(), &(1.0 / 6.0));
        assert_eq!(result.get("n2").unwrap(), &0.0);
        assert_eq!(result.get("n3").unwrap(), &0.5);
        assert_eq!(result.get("n4").unwrap(), &(1.0 / 3.0));
        assert_eq!(result.get("n5").unwrap(), &0.0);
    }

    #[test]
    fn test_betweenness_centrality_7() {
        // undirected, unweighted, not normalized
        let graph = get_graph_1(false);
        let result = betweenness::betweenness_centrality(&graph, false, false).unwrap();
        assert_eq!(result.get("n1").unwrap(), &1.5);
        assert_eq!(result.get("n2").unwrap(), &(1.0 / 3.0));
        assert_eq!(result.get("n3").unwrap(), &1.5);
        assert_eq!(result.get("n4").unwrap(), &(1.0 / 3.0));
        assert_eq!(result.get("n5").unwrap(), &(1.0 / 3.0));
    }

    #[test]
    fn test_betweenness_centrality_8() {
        // undirected, unweighted, normalized
        let graph = get_graph_1(false);
        let result = betweenness::betweenness_centrality(&graph, false, true).unwrap();
        assert_eq!(result.get("n1").unwrap(), &0.25);
        assert_eq!(result.get("n2").unwrap(), &(1.0 / 18.0));
        assert_eq!(result.get("n3").unwrap(), &0.25);
        assert_eq!(result.get("n4").unwrap(), &(1.0 / 18.0));
        assert_eq!(result.get("n5").unwrap(), &(1.0 / 18.0));
    }

    #[test]
    fn test_betweenness_centrality_9() {
        // karate club, unweighted, not normalized
        let graph = generators::social::karate_club_graph();
        let result = betweenness::betweenness_centrality(&graph, false, false).unwrap();
        assert_eq!(round(result.get(&0).unwrap(), 2), 231.07);
        assert_eq!(round(result.get(&1).unwrap(), 2), 28.48);
        assert_eq!(round(result.get(&2).unwrap(), 2), 75.85);
        assert_eq!(round(result.get(&3).unwrap(), 2), 6.29);
        assert_eq!(round(result.get(&4).unwrap(), 2), 0.33);
        assert_eq!(round(result.get(&5).unwrap(), 2), 15.83);
        assert_eq!(round(result.get(&6).unwrap(), 2), 15.83);
        assert_eq!(round(result.get(&7).unwrap(), 2), 0.0);
        assert_eq!(round(result.get(&8).unwrap(), 2), 29.53);
        assert_eq!(round(result.get(&9).unwrap(), 2), 0.45);
        assert_eq!(round(result.get(&10).unwrap(), 2), 0.33);
        assert_eq!(round(result.get(&11).unwrap(), 2), 0.0);
        assert_eq!(round(result.get(&12).unwrap(), 2), 0.0);
        assert_eq!(round(result.get(&13).unwrap(), 2), 24.22);
        assert_eq!(round(result.get(&14).unwrap(), 2), 0.0);
        assert_eq!(round(result.get(&15).unwrap(), 2), 0.0);
        assert_eq!(round(result.get(&16).unwrap(), 2), 0.0);
        assert_eq!(round(result.get(&17).unwrap(), 2), 0.0);
        assert_eq!(round(result.get(&18).unwrap(), 2), 0.0);
        assert_eq!(round(result.get(&19).unwrap(), 2), 17.15);
        assert_eq!(round(result.get(&20).unwrap(), 2), 0.0);
        assert_eq!(round(result.get(&21).unwrap(), 2), 0.0);
        assert_eq!(round(result.get(&22).unwrap(), 2), 0.0);
        assert_eq!(round(result.get(&23).unwrap(), 2), 9.30);
        assert_eq!(round(result.get(&24).unwrap(), 2), 1.17);
        assert_eq!(round(result.get(&25).unwrap(), 2), 2.03);
        assert_eq!(round(result.get(&26).unwrap(), 2), 0.0);
        assert_eq!(round(result.get(&27).unwrap(), 2), 11.79);
        assert_eq!(round(result.get(&28).unwrap(), 2), 0.95);
        assert_eq!(round(result.get(&29).unwrap(), 2), 1.54);
        assert_eq!(round(result.get(&30).unwrap(), 2), 7.61);
        assert_eq!(round(result.get(&31).unwrap(), 2), 73.01);
        assert_eq!(round(result.get(&32).unwrap(), 2), 76.69);
        assert_eq!(round(result.get(&33).unwrap(), 2), 160.55);
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
