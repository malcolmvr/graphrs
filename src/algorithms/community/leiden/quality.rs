use crate::{ext::hashset::IntSetExt, Graph};
use core::f64;
use nohash::IntSet;

use super::partition::Partition;

/**
The quality function to use for the Leiden algorithm.
[Modularity](<https://en.wikipedia.org/wiki/Leiden_algorithm#Modularity>) is a traditional
method of assessing how well a set of communities partition a graph.
[Constant Potts Model](<https://en.wikipedia.org/wiki/Leiden_algorithm#Constant_Potts_Model_(CPM)>)
is similar to modularity.
*/
pub enum QualityFunction {
    Modularity,
    CPM,
}

pub fn argmax(
    v: usize,
    partition: &Partition,
    communities: &[&IntSet<usize>],
    graph: &Graph<usize, f64>,
    weighted: bool,
    quality_function: &QualityFunction,
    resolution: f64,
) -> (IntSet<usize>, f64) {
    let mut opt: IntSet<usize> = communities[0].iter().cloned().collect();
    let mut val = get_delta(
        v,
        partition,
        &opt,
        graph,
        weighted,
        &quality_function,
        resolution,
    );
    for k in 1..communities.len() {
        let optk = &communities[k];
        let valk = get_delta(
            v,
            partition,
            optk,
            graph,
            weighted,
            &quality_function,
            resolution,
        );
        if valk > val {
            opt = optk.iter().cloned().collect();
            val = valk;
        }
    }
    (opt, val)
}

pub fn get_delta(
    v: usize,
    partition: &Partition,
    target: &IntSet<usize>,
    graph: &Graph<usize, f64>,
    weighted: bool,
    quality_function: &QualityFunction,
    resolution: f64,
) -> f64 {
    match quality_function {
        QualityFunction::Modularity => {
            get_delta_modularity(v, partition, target, graph, weighted, resolution)
        }
        QualityFunction::CPM => get_delta_cpm(v, partition, target, graph, weighted, resolution),
    }
}

fn get_delta_modularity(
    v: usize,
    partition: &Partition,
    target: &IntSet<usize>,
    graph: &Graph<usize, f64>,
    weighted: bool,
    resolution: f64,
) -> f64 {
    if target.contains(&v) {
        return 0.0;
    }
    let m = graph.size(weighted);
    let source_community = partition.node_community(v);
    let diff_source =
        single_node_neighbor_cut_size(graph, v, &source_community.without(&v), weighted);
    let diff_target = single_node_neighbor_cut_size(graph, v, &target, weighted);
    let deg_v = match weighted {
        true => graph.get_node_weighted_degree_by_index(v),
        false => graph.get_node_degree_by_index(v) as f64,
    };
    let degs_source = partition.degree_sum(v);
    let degs_target = match target.len() == 0 {
        true => 0.0,
        false => partition.degree_sum(*target.into_iter().next().unwrap()),
    };

    let delta = ((diff_target - diff_source)
        - resolution / (2.0 * m) * (deg_v.powf(2.0) + deg_v * (degs_target - degs_source)))
        / m;

    delta
}

fn get_delta_cpm(
    v: usize,
    partition: &Partition,
    target: &IntSet<usize>,
    graph: &Graph<usize, f64>,
    weighted: bool,
    resolution: f64,
) -> f64 {
    if target.contains(&v) {
        return 0.0;
    }
    let source_community = partition.node_community(v);
    let diff_source =
        single_node_neighbor_cut_size(graph, v, &source_community.without(&v), weighted);
    let diff_target = single_node_neighbor_cut_size(graph, v, &target, weighted);

    let node_weights = graph
        .get_all_nodes()
        .into_iter()
        .map(|n| n.attributes.unwrap())
        .collect::<Vec<f64>>();
    let v_weight = node_weights[v];
    let source_weight = source_community
        .iter()
        .map(|n| node_weights[*n])
        .sum::<f64>();
    let target_weight = target.iter().map(|n| node_weights[*n]).sum::<f64>();

    let delta = diff_target
        - diff_source
        - resolution * v_weight * (v_weight + target_weight - source_weight);

    delta
}

fn single_node_neighbor_cut_size(
    graph: &Graph<usize, f64>,
    v: usize,
    community: &IntSet<usize>,
    weighted: bool,
) -> f64 {
    graph
        .get_successor_nodes_by_index(&v)
        .into_iter()
        .filter(|x| community.contains(&x.node_index))
        .map(|x| match weighted {
            true => x.weight,
            false => 1.0,
        })
        .sum()
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::{Edge, Graph, GraphSpecs, Node};
    use assert_approx_eq::assert_approx_eq;
    use std::sync::Arc;

    #[test]
    fn test_single_node_neighbor_cut_size_1() {
        let edges: Vec<Arc<Edge<usize, f64>>> = vec![
            Edge::new(0, 1),
            Edge::new(1, 2),
            Edge::new(1, 3),
            Edge::new(1, 4),
        ];
        let specs = GraphSpecs::directed_create_missing();
        let graph = Graph::new_from_nodes_and_edges(vec![], edges, specs).unwrap();
        let community = vec![1, 2, 3].into_iter().collect();
        let result = single_node_neighbor_cut_size(&graph, 0, &community, false);
        assert_eq!(result, 1.0);
        let result = single_node_neighbor_cut_size(&graph, 1, &community, false);
        assert_eq!(result, 2.0);
        let result = single_node_neighbor_cut_size(&graph, 2, &community, false);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_single_node_neighbor_cut_size_2() {
        let edges: Vec<Arc<Edge<usize, f64>>> = vec![
            Edge::with_weight(0, 1, 1.1),
            Edge::with_weight(1, 2, 2.3),
            Edge::with_weight(1, 3, 3.5),
            Edge::with_weight(1, 4, 4.7),
        ];
        let specs = GraphSpecs::directed_create_missing();
        let graph = Graph::new_from_nodes_and_edges(vec![], edges, specs).unwrap();
        let community = vec![1, 2, 3].into_iter().collect();
        let result = single_node_neighbor_cut_size(&graph, 0, &community, true);
        assert_eq!(result, 1.1);
        let result = single_node_neighbor_cut_size(&graph, 1, &community, true);
        assert_eq!(result, 5.8);
        let result = single_node_neighbor_cut_size(&graph, 2, &community, true);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_get_delta_1() {
        let edges: Vec<Arc<Edge<usize, f64>>> = vec![
            Edge::with_weight(0, 1, 1.1),
            Edge::with_weight(1, 2, 2.3),
            Edge::with_weight(1, 3, 3.5),
            Edge::with_weight(1, 4, 4.7),
        ];
        let specs = GraphSpecs::directed_create_missing();
        let graph = Graph::new_from_nodes_and_edges(vec![], edges, specs).unwrap();
        let partition = Partition {
            partition: vec![
                vec![0, 1].into_iter().collect(),
                vec![2, 3, 4].into_iter().collect(),
            ],
            node_partition: vec![0, 0, 1, 1, 1],
            degree_sums: vec![12.0, 24.0],
        };
        let target = vec![2, 3, 4].into_iter().collect();
        let result = get_delta(
            1,
            &partition,
            &target,
            &graph,
            true,
            &QualityFunction::Modularity,
            1.0,
        );
        assert_approx_eq!(result, -0.11206896551724145);
    }

    #[test]
    fn test_get_delta_2() {
        let edges: Vec<Arc<Edge<usize, f64>>> = vec![
            Edge::with_weight(0, 1, 1.1),
            Edge::with_weight(1, 2, 2.3),
            Edge::with_weight(1, 3, 3.5),
            Edge::with_weight(1, 4, 4.7),
        ];
        let specs = GraphSpecs::undirected_create_missing();
        let graph = Graph::new_from_nodes_and_edges(vec![], edges, specs).unwrap();
        let partition = Partition {
            partition: vec![
                vec![0, 1].into_iter().collect(),
                vec![2, 3, 4].into_iter().collect(),
            ],
            node_partition: vec![0, 0, 1, 1, 1],
            degree_sums: vec![12.0, 24.0],
        };
        let target = vec![2, 3, 4].into_iter().collect();
        let result = get_delta(
            1,
            &partition,
            &target,
            &graph,
            true,
            &QualityFunction::Modularity,
            1.0,
        );
        assert_approx_eq!(result, -0.20689655172413812);
    }

    #[test]
    fn test_argmax_1() {
        let graph = get_graph_for_argmax(true);
        let partition = get_partition_for_argmax();
        let empty = IntSet::default();
        let communities = get_communities_for_argmax(&partition, &graph, &empty);
        let result = argmax(
            0,
            &partition,
            &communities,
            &graph,
            true,
            &QualityFunction::Modularity,
            1.0,
        );
        assert_eq!(result.0.len(), 1);
        assert!(result.0.contains(&2));
        assert_approx_eq!(result.1, 0.09033145065398336);
        let result = argmax(
            0,
            &partition,
            &communities,
            &graph,
            false,
            &QualityFunction::Modularity,
            1.0,
        );
        assert_eq!(result.0.len(), 1);
        assert!(result.0.contains(&2));
        assert_approx_eq!(result.1, 0.21875);
    }

    #[test]
    fn test_argmax_2() {
        let graph = get_graph_for_argmax(false);
        let partition = get_partition_for_argmax();
        let empty = IntSet::default();
        let communities = get_communities_for_argmax(&partition, &graph, &empty);
        let result = argmax(
            0,
            &partition,
            &communities,
            &graph,
            true,
            &QualityFunction::Modularity,
            1.0,
        );
        assert_eq!(result.0.len(), 1);
        assert!(result.0.contains(&2));
        assert_approx_eq!(result.1, 0.09033145065398336);
        let result = argmax(
            0,
            &partition,
            &communities,
            &graph,
            false,
            &QualityFunction::Modularity,
            1.0,
        );
        assert_eq!(result.0.len(), 1);
        assert!(result.0.contains(&2));
        assert_approx_eq!(result.1, 0.21875);
    }

    fn get_graph_for_argmax(directed: bool) -> Graph<usize, f64> {
        let nodes = vec![
            Node::from_name(0),
            Node::from_name(1),
            Node::from_name(2),
            Node::from_name(3),
            Node::from_name(4),
        ];
        let edges: Vec<Arc<Edge<usize, f64>>> = vec![
            Edge::with_weight(0, 2, 1.1),
            Edge::with_weight(1, 2, 2.3),
            Edge::with_weight(2, 3, 3.5),
            Edge::with_weight(2, 4, 4.7),
        ];
        let specs = if directed {
            GraphSpecs::directed_create_missing()
        } else {
            GraphSpecs::undirected_create_missing()
        };
        Graph::new_from_nodes_and_edges(nodes, edges, specs).unwrap()
    }

    fn get_partition_for_argmax() -> Partition {
        Partition {
            partition: vec![
                vec![0, 1].into_iter().collect(),
                vec![2].into_iter().collect(),
                vec![3].into_iter().collect(),
                vec![4].into_iter().collect(),
            ],
            node_partition: vec![0, 0, 1, 2, 3],
            degree_sums: vec![0.0, 0.0, 0.0, 0.0],
        }
    }

    fn get_communities_for_argmax<'a>(
        partition: &'a Partition,
        graph: &Graph<usize, f64>,
        empty: &'a IntSet<usize>,
    ) -> Vec<&'a IntSet<usize>> {
        partition.get_adjacent_communities(0, &graph, empty)
    }
}
