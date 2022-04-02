mod utility;

#[cfg(test)]
mod tests {

    use super::utility::round;
    use graphrs::{algorithms::community::partitions, generators, Edge, Graph, GraphSpecs};
    use std::collections::HashSet;

    #[test]
    fn test_is_partition_1() {
        let edges = vec![Edge::new("n1", "n2"), Edge::new("n3", "n4")];
        let graph: Graph<&str, ()> =
            Graph::new_from_nodes_and_edges(vec![], edges, GraphSpecs::undirected_create_missing())
                .unwrap();
        let communities = make_partition(vec![vec!["n1"], vec!["n2"], vec!["n3"], vec!["n4"]]);
        assert!(partitions::is_partition(&graph, &communities));
        let communities = make_partition(vec![vec!["n1", "n2"], vec!["n3", "n4"]]);
        assert!(partitions::is_partition(&graph, &communities));
        let communities = make_partition(vec![vec!["n1", "n2"], vec!["n3"]]);
        assert!(!partitions::is_partition(&graph, &communities));
        let communities = make_partition(vec![]);
        assert!(!partitions::is_partition(&graph, &communities));
    }

    #[test]
    fn test_modularity_1() {
        let mut graph: Graph<&str, ()> = Graph::new(GraphSpecs::directed_create_missing());
        assert!(graph
            .add_edges(vec![
                Edge::with_weight("n1", "n2", 1.0),
                Edge::with_weight("n2", "n3", 5.0),
                Edge::with_weight("n3", "n4", 2.0),
                Edge::with_weight("n2", "n4", 2.0),
                Edge::with_weight("n4", "n2", 2.0),
            ])
            .is_ok());
        let communities = make_partition(vec![vec!["n1", "n2"], vec!["n3", "n4"]]);
        assert_eq!(
            round(
                &partitions::modularity(&graph, &communities, false, None).unwrap(),
                2
            ),
            -0.08
        );
        assert_eq!(
            round(
                &partitions::modularity(&graph, &communities, true, None).unwrap(),
                2
            ),
            -0.17
        );
        assert_eq!(
            round(
                &partitions::modularity(&graph, &communities, true, Some(2.0)).unwrap(),
                2
            ),
            -0.58
        );
        let communities = make_partition(vec![vec!["n1", "n2", "n3"], vec!["n4"]]);
        assert_eq!(
            round(
                &partitions::modularity(&graph, &communities, false, None).unwrap(),
                2
            ),
            -0.16
        );
        assert_eq!(
            round(
                &partitions::modularity(&graph, &communities, true, None).unwrap(),
                2
            ),
            -0.11
        );
    }

    #[test]
    fn test_modularity_2() {
        let graph = generators::social::karate_club_graph();
        let communities = vec![
            vec![0, 1, 2, 3, 4, 5].into_iter().collect(),
            vec![6, 7, 8, 9, 10, 11].into_iter().collect(),
            vec![12, 13, 14, 15, 16, 17].into_iter().collect(),
            vec![18, 19, 20, 21, 22, 23].into_iter().collect(),
            vec![24, 25, 26, 27, 28, 29].into_iter().collect(),
            vec![30, 31, 32, 33].into_iter().collect(),
        ];
        assert_eq!(
            round(
                &partitions::modularity(&graph, &communities, false, None).unwrap(),
                4
            ),
            -0.0015
        );
        assert_eq!(
            round(
                &partitions::modularity(&graph, &communities, false, Some(2.0)).unwrap(),
                4
            ),
            -0.2081
        );
    }

    fn make_partition(partitions: Vec<Vec<&str>>) -> Vec<HashSet<&str>> {
        partitions
            .iter()
            .map(|v| v.iter().copied().collect::<HashSet<&str>>())
            .collect()
    }
}
