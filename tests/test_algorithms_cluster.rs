mod utility;

#[cfg(test)]
mod tests {

    use super::utility::round;
    use graphrs::{algorithms::cluster, generators, Edge, Graph, GraphSpecs};
    use std::collections::HashMap;

    #[test]
    fn test_average_clustering_1() {
        let graph = generators::social::karate_club_graph();
        let result = cluster::average_clustering(&graph, false, None, true).unwrap();
        assert_eq!(round(&result, 3), 0.571);
    }

    #[test]
    fn test_average_clustering_2() {
        let graph = generators::social::karate_club_graph();
        let result = cluster::average_clustering(&graph, false, None, false).unwrap();
        assert_eq!(round(&result, 3), 0.606);
    }

    #[test]
    fn test_clustering_undirected_unweighted() {
        let graph = generators::social::karate_club_graph();
        let result = cluster::clustering(&graph, false, None).unwrap();
        assert_eq!(
            result,
            vec![
                (0, 0.15),
                (1, 0.3333333333333333),
                (2, 0.24444444444444444),
                (3, 0.6666666666666666),
                (4, 0.6666666666666666),
                (5, 0.5),
                (6, 0.5),
                (7, 1.0),
                (8, 0.5),
                (9, 0.0),
                (10, 0.6666666666666666),
                (11, 0.0),
                (12, 1.0),
                (13, 0.6),
                (14, 1.0),
                (15, 1.0),
                (16, 1.0),
                (17, 1.0),
                (18, 1.0),
                (19, 0.3333333333333333),
                (20, 1.0),
                (21, 1.0),
                (22, 1.0),
                (23, 0.4),
                (24, 0.3333333333333333),
                (25, 0.3333333333333333),
                (26, 1.0),
                (27, 0.16666666666666666),
                (28, 0.3333333333333333),
                (29, 0.6666666666666666),
                (30, 0.5),
                (31, 0.2),
                (32, 0.19696969696969696),
                (33, 0.11029411764705882),
            ]
            .into_iter()
            .collect::<HashMap<i32, f64>>(),
        )
    }

    #[test]
    fn test_clustering_undirected_weighted() {
        let edges = vec![
            Edge::with_weight("n0", "n1", 1.1),
            Edge::with_weight("n0", "n2", 1.3),
            Edge::with_weight("n0", "n3", 1.4),
            Edge::with_weight("n3", "n2", 1.5),
            Edge::with_weight("n4", "n3", 1.6),
            Edge::with_weight("n4", "n2", 1.7),
        ];
        let graph: Graph<&str, ()> =
            Graph::new_from_nodes_and_edges(vec![], edges, GraphSpecs::undirected_create_missing())
                .unwrap();
        let result = cluster::clustering(&graph, true, None).unwrap();
        assert_eq!(result.get("n0").unwrap(), &0.274042154286997);
        assert_eq!(result.get("n1").unwrap(), &0.0);
        assert_eq!(result.get("n2").unwrap(), &0.5873586146969583);
        assert_eq!(result.get("n3").unwrap(), &0.5873586146969583);
        assert_eq!(result.get("n4").unwrap(), &0.9399493812298838);
    }

    #[test]
    fn test_clustering_directed_unweighted() {
        let edges = vec![
            Edge::new("n0", "n1"),
            Edge::new("n0", "n2"),
            Edge::new("n0", "n3"),
            Edge::new("n3", "n2"),
            Edge::new("n4", "n3"),
            Edge::new("n4", "n2"),
        ];
        let graph: Graph<&str, ()> =
            Graph::new_from_nodes_and_edges(vec![], edges, GraphSpecs::directed_create_missing())
                .unwrap();
        let result = cluster::clustering(&graph, false, None).unwrap();
        assert_eq!(result.get("n0").unwrap(), &0.16666666666666666);
        assert_eq!(result.get("n1").unwrap(), &0.0);
        assert_eq!(result.get("n2").unwrap(), &0.3333333333333333);
        assert_eq!(result.get("n3").unwrap(), &0.3333333333333333);
        assert_eq!(result.get("n4").unwrap(), &0.5);
    }

    #[test]
    fn test_clustering_directed_weighted() {
        let edges = vec![
            Edge::with_weight("n0", "n1", 1.1),
            Edge::with_weight("n0", "n2", 1.3),
            Edge::with_weight("n0", "n3", 1.4),
            Edge::with_weight("n3", "n2", 1.5),
            Edge::with_weight("n4", "n3", 1.6),
            Edge::with_weight("n4", "n2", 1.7),
        ];
        let graph: Graph<&str, ()> =
            Graph::new_from_nodes_and_edges(vec![], edges, GraphSpecs::directed_create_missing())
                .unwrap();
        let result = cluster::clustering(&graph, true, None).unwrap();
        assert_eq!(result.get("n0").unwrap(), &0.1370210771434985);
        assert_eq!(result.get("n1").unwrap(), &0.0);
        assert_eq!(result.get("n2").unwrap(), &0.29367930734847913);
        assert_eq!(result.get("n3").unwrap(), &0.29367930734847913);
        assert_eq!(result.get("n4").unwrap(), &0.4699746906149419);
    }

    #[test]
    fn test_generalized_degree_1() {
        let edges = vec![
            Edge::with_weight("n0", "n1", 1.1),
            Edge::with_weight("n0", "n2", 1.3),
            Edge::with_weight("n0", "n3", 1.4),
            Edge::with_weight("n3", "n2", 1.5),
        ];
        let graph: Graph<&str, ()> =
            Graph::new_from_nodes_and_edges(vec![], edges, GraphSpecs::undirected_create_missing())
                .unwrap();
        let result = cluster::generalized_degree(&graph, None).unwrap();
        assert_eq!(
            result.get("n0").unwrap(),
            &vec![(1, 2), (0, 1)].into_iter().collect::<HashMap<usize, usize>>()
        );
        assert_eq!(
            result.get("n1").unwrap(),
            &vec![(0, 1)].into_iter().collect::<HashMap<usize, usize>>()
        );
        assert_eq!(
            result.get("n2").unwrap(),
            &vec![(1, 2)].into_iter().collect::<HashMap<usize, usize>>()
        );
        assert_eq!(
            result.get("n3").unwrap(),
            &vec![(1, 2)].into_iter().collect::<HashMap<usize, usize>>()
        );
    }

    #[test]
    fn test_generalized_degree_2() {
        let edges = vec![
            Edge::with_weight("n0", "n1", 1.1),
            Edge::with_weight("n0", "n2", 1.3),
            Edge::with_weight("n0", "n3", 1.4),
            Edge::with_weight("n3", "n2", 1.5),
        ];
        let graph: Graph<&str, ()> =
            Graph::new_from_nodes_and_edges(vec![], edges, GraphSpecs::directed_create_missing())
                .unwrap();
        let result = cluster::generalized_degree(&graph, None);
        assert!(result.is_err());
    }

    #[test]
    fn test_generalized_degree_3() {
        let graph = generators::social::karate_club_graph();
        let result = cluster::generalized_degree(&graph, None).unwrap();
        assert_eq!(
            result.get(&0).unwrap(),
            &vec![(0, 2), (1, 5), (2, 4), (3, 2), (5, 2), (7, 1)]
                .into_iter()
                .collect::<HashMap<usize, usize>>()
        );
    }

    #[test]
    fn test_square_clustering_1() {
        let graph = generators::social::karate_club_graph();
        let result = cluster::square_clustering(&graph, Some(&[0, 1, 33]));
        assert_eq!(result.get(&0).unwrap(), &0.09051724137931035);
        assert_eq!(result.get(&1).unwrap(), &0.17216117216117216);
        assert_eq!(result.get(&33).unwrap(), &0.12158054711246201);
    }

    #[test]
    fn test_transitivity_1() {
        let graph = generators::social::karate_club_graph();
        let result = cluster::transitivity(&graph).unwrap();
        assert_eq!(result, 0.2556818181818182);
    }

    #[test]
    fn test_transitivity_2() {
        let edges = vec![
            Edge::with_weight("n0", "n1", 1.1),
            Edge::with_weight("n0", "n2", 1.3),
            Edge::with_weight("n0", "n3", 1.4),
            Edge::with_weight("n3", "n2", 1.5),
        ];
        let graph: Graph<&str, ()> =
            Graph::new_from_nodes_and_edges(vec![], edges, GraphSpecs::directed_create_missing())
                .unwrap();
        let result = cluster::transitivity(&graph);
        assert!(result.is_err());
    }

    #[test]
    fn test_transitivity_3() {
        let graph: Graph<&str, ()> = Graph::new_from_nodes_and_edges(
            vec![],
            vec![],
            GraphSpecs::undirected_create_missing(),
        )
        .unwrap();
        let result = cluster::transitivity(&graph).unwrap();
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_triangles_1() {
        let graph = generators::social::karate_club_graph();
        let result = cluster::triangles(&graph, None).unwrap();
        let expected = vec![
            (0, 18),
            (1, 12),
            (2, 11),
            (3, 10),
            (4, 2),
            (5, 3),
            (6, 3),
            (7, 6),
            (8, 5),
            (9, 0),
            (10, 2),
            (11, 0),
            (12, 1),
            (13, 6),
            (14, 1),
            (15, 1),
            (16, 1),
            (17, 1),
            (18, 1),
            (19, 1),
            (20, 1),
            (21, 1),
            (22, 1),
            (23, 4),
            (24, 1),
            (25, 1),
            (26, 1),
            (27, 1),
            (28, 1),
            (29, 4),
            (30, 3),
            (31, 3),
            (32, 13),
            (33, 15),
        ]
        .into_iter()
        .collect::<HashMap<i32, usize>>();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_triangles_2() {
        let edges = vec![
            Edge::with_weight("n0", "n1", 1.1),
            Edge::with_weight("n0", "n2", 1.3),
            Edge::with_weight("n0", "n3", 1.4),
            Edge::with_weight("n3", "n2", 1.5),
        ];
        let graph: Graph<&str, ()> =
            Graph::new_from_nodes_and_edges(vec![], edges, GraphSpecs::directed_create_missing())
                .unwrap();
        let result = cluster::triangles(&graph, None);
        assert!(result.is_err());
    }
}
