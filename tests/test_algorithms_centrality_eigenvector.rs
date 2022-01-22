mod utility;

#[cfg(test)]
mod tests {

    use super::utility::round;
    use graphrs::{
        algorithms::centrality::eigenvector, generators, Edge, ErrorKind, Graph, GraphSpecs,
    };

    #[test]
    fn test_eigenvector_centrality_1() {
        let edges = vec![
            Edge::with_weight("n1", "n2", 1.0),
            Edge::with_weight("n2", "n3", 5.0),
            Edge::with_weight("n1", "n4", 2.0),
            Edge::with_weight("n4", "n3", 3.0),
            Edge::with_weight("n1", "n5", 9.0),
            Edge::with_weight("n3", "n5", 1.0),
        ];
        let graph: Graph<&str, ()> =
            Graph::new_from_nodes_and_edges(vec![], edges, GraphSpecs::directed_create_missing())
                .unwrap();
        let result = eigenvector::eigenvector_centrality(&graph, true, None, None);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(
            err.kind,
            ErrorKind::PowerIterationFailedConvergence
        ));
    }

    #[test]
    fn test_eigenvector_centrality_2() {
        let edges = vec![
            Edge::with_weight("n1", "n2", 1.0),
            Edge::with_weight("n2", "n3", 5.0),
            Edge::with_weight("n1", "n4", 2.0),
            Edge::with_weight("n4", "n3", 3.0),
            Edge::with_weight("n1", "n5", 9.0),
            Edge::with_weight("n3", "n5", 1.0),
            Edge::with_weight("n5", "n1", 1.0),
        ];
        let graph: Graph<&str, ()> =
            Graph::new_from_nodes_and_edges(vec![], edges, GraphSpecs::directed_create_missing())
                .unwrap();
        let result = eigenvector::eigenvector_centrality(&graph, true, None, None).unwrap();
        assert_eq!(round(result.get("n1").unwrap(), 2), 0.28);
        assert_eq!(round(result.get("n2").unwrap(), 2), 0.09);
        assert_eq!(round(result.get("n3").unwrap(), 2), 0.31);
        assert_eq!(round(result.get("n4").unwrap(), 2), 0.18);
        assert_eq!(round(result.get("n5").unwrap(), 2), 0.89);
    }

    #[test]
    fn test_eigenvector_centrality_3() {
        // karate club, unweighted
        let graph = generators::social::karate_club_graph();
        let result = eigenvector::eigenvector_centrality(&graph, false, None, None).unwrap();
        assert_eq!(round(result.get(&0).unwrap(), 2), 0.36);
        assert_eq!(round(result.get(&1).unwrap(), 2), 0.27);
        assert_eq!(round(result.get(&2).unwrap(), 2), 0.32);
        assert_eq!(round(result.get(&3).unwrap(), 2), 0.21);
        assert_eq!(round(result.get(&4).unwrap(), 2), 0.08);
        assert_eq!(round(result.get(&5).unwrap(), 2), 0.08);
        assert_eq!(round(result.get(&6).unwrap(), 2), 0.08);
        assert_eq!(round(result.get(&7).unwrap(), 2), 0.17);
        assert_eq!(round(result.get(&8).unwrap(), 2), 0.23);
        assert_eq!(round(result.get(&9).unwrap(), 2), 0.10);
        assert_eq!(round(result.get(&10).unwrap(), 2), 0.08);
        assert_eq!(round(result.get(&11).unwrap(), 2), 0.05);
        assert_eq!(round(result.get(&12).unwrap(), 2), 0.08);
        assert_eq!(round(result.get(&13).unwrap(), 2), 0.23);
        assert_eq!(round(result.get(&14).unwrap(), 2), 0.10);
        assert_eq!(round(result.get(&15).unwrap(), 2), 0.10);
        assert_eq!(round(result.get(&16).unwrap(), 2), 0.02);
        assert_eq!(round(result.get(&17).unwrap(), 2), 0.09);
        assert_eq!(round(result.get(&18).unwrap(), 2), 0.10);
        assert_eq!(round(result.get(&19).unwrap(), 2), 0.15);
        assert_eq!(round(result.get(&20).unwrap(), 2), 0.10);
        assert_eq!(round(result.get(&21).unwrap(), 2), 0.09);
        assert_eq!(round(result.get(&22).unwrap(), 2), 0.10);
        assert_eq!(round(result.get(&23).unwrap(), 2), 0.15);
        assert_eq!(round(result.get(&24).unwrap(), 2), 0.06);
        assert_eq!(round(result.get(&25).unwrap(), 2), 0.06);
        assert_eq!(round(result.get(&26).unwrap(), 2), 0.08);
        assert_eq!(round(result.get(&27).unwrap(), 2), 0.13);
        assert_eq!(round(result.get(&28).unwrap(), 2), 0.13);
        assert_eq!(round(result.get(&29).unwrap(), 2), 0.13);
        assert_eq!(round(result.get(&30).unwrap(), 2), 0.17);
        assert_eq!(round(result.get(&31).unwrap(), 2), 0.19);
        assert_eq!(round(result.get(&32).unwrap(), 2), 0.31);
        assert_eq!(round(result.get(&33).unwrap(), 2), 0.37);
    }
}
