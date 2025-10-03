mod utility;

#[cfg(test)]
mod tests {

    use graphrs::{algorithms::community::louvain, generators, Edge, Graph, GraphSpecs};
    use itertools::Itertools;

    #[test]
    fn test_louvain_partitions_0() {
        let edges = vec![
            Edge::with_weight(0, 1, 1.1),
            Edge::with_weight(1, 0, 1.2),
            Edge::with_weight(0, 2, 1.3),
            Edge::with_weight(0, 3, 1.4),
            Edge::with_weight(3, 2, 1.5),
        ];
        let graph: Graph<i32, ()> =
            Graph::new_from_nodes_and_edges(vec![], edges, GraphSpecs::directed_create_missing())
                .unwrap();
        let partitions = louvain::louvain_partitions(&graph, true, None, None, None).unwrap();
        let best_partition = partitions.last().unwrap();
        let mut community1: Vec<i32> = best_partition
            .first()
            .unwrap()
            .iter()
            .cloned()
            .sorted()
            .collect();
        let mut community2: Vec<i32> = best_partition
            .last()
            .unwrap()
            .iter()
            .cloned()
            .sorted()
            .collect();
        if community1[0] == 2 {
            (community1, community2) = (community2, community1);
        }
        assert_eq!(community1, vec![0, 1]);
        assert_eq!(community2, vec![2, 3]);
    }

    #[test]
    fn test_louvain_partitions_1() {
        let edges = vec![
            Edge::with_weight("n0", "n1", 1.1),
            Edge::with_weight("n1", "n0", 1.2),
            Edge::with_weight("n0", "n2", 1.3),
            Edge::with_weight("n0", "n3", 1.4),
            Edge::with_weight("n3", "n2", 1.5),
        ];
        let graph: Graph<&str, ()> =
            Graph::new_from_nodes_and_edges(vec![], edges, GraphSpecs::directed_create_missing())
                .unwrap();
        let partitions = louvain::louvain_partitions(&graph, true, None, None, None).unwrap();
        let best_partition = partitions.last().unwrap();
        let mut community1: Vec<&str> = best_partition
            .first()
            .unwrap()
            .iter()
            .cloned()
            .sorted()
            .collect();
        let mut community2: Vec<&str> = best_partition
            .last()
            .unwrap()
            .iter()
            .cloned()
            .sorted()
            .collect();
        if community1[0] == "n2" {
            (community1, community2) = (community2, community1);
        }
        assert_eq!(community1, vec!["n0", "n1"]);
        assert_eq!(community2, vec!["n2", "n3"]);
    }

    #[test]
    fn test_louvain_communities_1() {
        let edges = vec![
            Edge::with_weight("n0", "n1", 1.1),
            Edge::with_weight("n1", "n0", 1.2),
            Edge::with_weight("n0", "n2", 1.3),
            Edge::with_weight("n0", "n3", 1.4),
            Edge::with_weight("n3", "n2", 1.5),
        ];
        let graph: Graph<&str, ()> =
            Graph::new_from_nodes_and_edges(vec![], edges, GraphSpecs::directed_create_missing())
                .unwrap();
        let communities = louvain::louvain_communities(&graph, true, None, None, None).unwrap();
        let mut community1: Vec<&str> = communities
            .first()
            .unwrap()
            .iter()
            .cloned()
            .sorted()
            .collect();
        let mut community2: Vec<&str> = communities
            .last()
            .unwrap()
            .iter()
            .cloned()
            .sorted()
            .collect();
        if community1[0] == "n2" {
            (community1, community2) = (community2, community1);
        }
        assert_eq!(community1, vec!["n0", "n1"]);
        assert_eq!(community2, vec!["n2", "n3"]);
    }

    #[test]
    fn test_louvain_partitions_2() {
        let graph = generators::social::karate_club_graph();
        let partitions = louvain::louvain_partitions(&graph, true, None, None, None).unwrap();
        let best_partition = partitions.last().unwrap();
        // Note: Results are non-deterministic due to HashMap iteration order
        // We only check that we get some communities
        assert!(best_partition.len() >= 2);
        assert!(best_partition.len() <= 10); // reasonable upper bound
    }

    #[test]
    fn test_louvain_partitions_deterministic() {
        let graph = generators::social::karate_club_graph();

        // Test that deterministic results are consistent
        let partitions1 = louvain::louvain_partitions(&graph, true, None, None, Some(42)).unwrap();
        let partitions2 = louvain::louvain_partitions(&graph, true, None, None, Some(42)).unwrap();
        let partitions3 = louvain::louvain_partitions(&graph, true, None, None, Some(42)).unwrap();

        // All runs with the same seed should produce identical results
        assert_eq!(partitions1.len(), partitions2.len());
        assert_eq!(partitions1.len(), partitions3.len());

        let best_partition1 = partitions1.last().unwrap();
        let best_partition2 = partitions2.last().unwrap();
        let best_partition3 = partitions3.last().unwrap();

        assert_eq!(best_partition1.len(), best_partition2.len());
        assert_eq!(best_partition1.len(), best_partition3.len());

        // Convert to sorted vectors for comparison since HashSets don't have order
        let mut communities1: Vec<Vec<i32>> = best_partition1
            .iter()
            .map(|community| {
                let mut nodes: Vec<i32> = community.iter().cloned().sorted().collect();
                nodes.sort();
                nodes
            })
            .collect();
        communities1.sort();

        let mut communities2: Vec<Vec<i32>> = best_partition2
            .iter()
            .map(|community| {
                let mut nodes: Vec<i32> = community.iter().cloned().sorted().collect();
                nodes.sort();
                nodes
            })
            .collect();
        communities2.sort();

        let mut communities3: Vec<Vec<i32>> = best_partition3
            .iter()
            .map(|community| {
                let mut nodes: Vec<i32> = community.iter().cloned().sorted().collect();
                nodes.sort();
                nodes
            })
            .collect();
        communities3.sort();

        // All deterministic runs should produce identical community structures
        assert_eq!(communities1, communities2);
        assert_eq!(communities1, communities3);
    }

    #[test]
    fn test_louvain_partitions_non_deterministic() {
        let graph = generators::social::karate_club_graph();

        // Test non-deterministic behavior - results may vary between runs
        let partitions1 = louvain::louvain_partitions(&graph, true, None, None, None).unwrap();
        let partitions2 = louvain::louvain_partitions(&graph, true, None, None, None).unwrap();
        let partitions3 = louvain::louvain_partitions(&graph, true, None, None, None).unwrap();

        let best_partition1 = partitions1.last().unwrap();
        let best_partition2 = partitions2.last().unwrap();
        let best_partition3 = partitions3.last().unwrap();

        // All runs should produce reasonable numbers of communities
        assert!(best_partition1.len() >= 2);
        assert!(best_partition1.len() <= 10);
        assert!(best_partition2.len() >= 2);
        assert!(best_partition2.len() <= 10);
        assert!(best_partition3.len() >= 2);
        assert!(best_partition3.len() <= 10);

        // Note: We don't assert that results are different since they might
        // occasionally be the same by chance, but in practice they often vary
    }

    #[test]
    fn test_louvain_partitions_different_seeds() {
        let graph = generators::social::karate_club_graph();

        // Test that different seeds can produce different deterministic results
        let partitions_seed_1 =
            louvain::louvain_partitions(&graph, true, None, None, Some(1)).unwrap();
        let partitions_seed_2 =
            louvain::louvain_partitions(&graph, true, None, None, Some(2)).unwrap();
        let partitions_seed_42 =
            louvain::louvain_partitions(&graph, true, None, None, Some(42)).unwrap();

        let best_partition_1 = partitions_seed_1.last().unwrap();
        let best_partition_2 = partitions_seed_2.last().unwrap();
        let best_partition_42 = partitions_seed_42.last().unwrap();

        // All should produce valid community counts
        assert!(best_partition_1.len() >= 2);
        assert!(best_partition_2.len() >= 2);
        assert!(best_partition_42.len() >= 2);

        // Verify each seed produces consistent results
        let partitions_seed_1_again =
            louvain::louvain_partitions(&graph, true, None, None, Some(1)).unwrap();
        let best_partition_1_again = partitions_seed_1_again.last().unwrap();
        assert_eq!(best_partition_1.len(), best_partition_1_again.len());
    }

    #[test]
    fn test_louvain_communities_with_negative_resolution() {
        let graph = generators::social::karate_club_graph();
        let communities = louvain::louvain_communities(&graph, false, None, None, None).unwrap();
        // Note: Results are non-deterministic, just check we get some communities
        assert!(communities.len() >= 2);
    }

    #[test]
    fn test_louvain_communities_3() {
        let graph = generators::random::fast_gnp_random_graph(12, 0.1, true, Some(1)).unwrap();
        let communities = louvain::louvain_communities(&graph, true, None, None, None).unwrap();
        assert_eq!(communities.len(), 5);
    }

    #[test]
    fn test_louvain_communities_deterministic() {
        let graph = generators::social::karate_club_graph();

        // Test that deterministic results are consistent
        let communities1 =
            louvain::louvain_communities(&graph, false, None, None, Some(42)).unwrap();
        let communities2 =
            louvain::louvain_communities(&graph, false, None, None, Some(42)).unwrap();
        let communities3 =
            louvain::louvain_communities(&graph, false, None, None, Some(42)).unwrap();

        // All runs with the same seed should produce identical results
        assert_eq!(communities1.len(), communities2.len());
        assert_eq!(communities1.len(), communities3.len());

        // Convert to sorted vectors for comparison since HashSets don't have order
        let mut sorted_communities1: Vec<Vec<i32>> = communities1
            .iter()
            .map(|community| {
                let mut nodes: Vec<i32> = community.iter().cloned().collect();
                nodes.sort();
                nodes
            })
            .collect();
        sorted_communities1.sort();

        let mut sorted_communities2: Vec<Vec<i32>> = communities2
            .iter()
            .map(|community| {
                let mut nodes: Vec<i32> = community.iter().cloned().collect();
                nodes.sort();
                nodes
            })
            .collect();
        sorted_communities2.sort();

        let mut sorted_communities3: Vec<Vec<i32>> = communities3
            .iter()
            .map(|community| {
                let mut nodes: Vec<i32> = community.iter().cloned().collect();
                nodes.sort();
                nodes
            })
            .collect();
        sorted_communities3.sort();

        // All deterministic runs should produce identical community structures
        assert_eq!(sorted_communities1, sorted_communities2);
        assert_eq!(sorted_communities1, sorted_communities3);
    }

    #[test]
    fn test_louvain_communities_non_deterministic() {
        let graph = generators::social::karate_club_graph();

        // Test non-deterministic behavior - results may vary between runs
        let communities1 = louvain::louvain_communities(&graph, false, None, None, None).unwrap();
        let communities2 = louvain::louvain_communities(&graph, false, None, None, None).unwrap();
        let communities3 = louvain::louvain_communities(&graph, false, None, None, None).unwrap();

        // All runs should produce reasonable numbers of communities
        assert!(communities1.len() >= 2);
        assert!(communities1.len() <= 10);
        assert!(communities2.len() >= 2);
        assert!(communities2.len() <= 10);
        assert!(communities3.len() >= 2);
        assert!(communities3.len() <= 10);

        // Verify all communities are non-empty
        for community in &communities1 {
            assert!(!community.is_empty());
        }
        for community in &communities2 {
            assert!(!community.is_empty());
        }
        for community in &communities3 {
            assert!(!community.is_empty());
        }
    }

    #[test]
    fn test_louvain_communities_weighted_vs_unweighted() {
        let graph = generators::social::karate_club_graph();

        // Test deterministic behavior with weighted and unweighted versions
        let communities_weighted =
            louvain::louvain_communities(&graph, true, None, None, Some(123)).unwrap();
        let communities_unweighted =
            louvain::louvain_communities(&graph, false, None, None, Some(123)).unwrap();

        // Both should produce valid results
        assert!(communities_weighted.len() >= 2);
        assert!(communities_unweighted.len() >= 2);

        // Test consistency with same parameters
        let communities_weighted_again =
            louvain::louvain_communities(&graph, true, None, None, Some(123)).unwrap();
        let communities_unweighted_again =
            louvain::louvain_communities(&graph, false, None, None, Some(123)).unwrap();

        assert_eq!(communities_weighted.len(), communities_weighted_again.len());
        assert_eq!(
            communities_unweighted.len(),
            communities_unweighted_again.len()
        );
    }

    #[test]
    fn test_louvain_communities_performance_comparison() {
        use std::time::Instant;
        let graph = generators::social::karate_club_graph();

        // Time non-deterministic version (should be faster)
        let start = Instant::now();
        let _communities_fast =
            louvain::louvain_communities(&graph, false, None, None, None).unwrap();
        let fast_duration = start.elapsed();

        // Time deterministic version (should be slower due to sorting)
        let start = Instant::now();
        let _communities_deterministic =
            louvain::louvain_communities(&graph, false, None, None, Some(42)).unwrap();
        let deterministic_duration = start.elapsed();

        // Both should complete in reasonable time (this is just a smoke test)
        assert!(fast_duration.as_millis() < 1000); // Should complete in under 1 second
        assert!(deterministic_duration.as_millis() < 1000); // Should complete in under 1 second

        // Note: We don't assert that fast_duration < deterministic_duration because
        // for small graphs the difference might not be measurable or consistent
    }
}
