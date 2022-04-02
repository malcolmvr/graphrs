mod utility;

#[cfg(test)]
mod tests {

    use graphrs::{algorithms::community::louvain, generators, Edge, Graph, GraphSpecs};
    use itertools::Itertools;

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
        let partitions = louvain::louvain_partitions(&graph, false, None, None, Some(1)).unwrap();
        let best_partition = partitions.last().unwrap();
        assert_eq!(best_partition.len(), 4);
    }

    #[test]
    fn test_louvain_communities_2() {
        let graph = generators::social::karate_club_graph();
        let communities = louvain::louvain_communities(&graph, false, None, None, Some(1)).unwrap();
        assert_eq!(communities.len(), 4);
    }
}
