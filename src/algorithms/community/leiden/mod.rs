use crate::{
    algorithms::community::partitions, algorithms::community::utility,
    algorithms::cuts::cut_size_by_indexes, ext::hashset::IntSetExt, Error, Graph,
};
use core::f64;
use itertools::Itertools;
use nohash::IntSet;
use rand::distributions::WeightedIndex;
use rand::{distributions::Distribution, RngCore};
use std::collections::{HashSet, VecDeque};
use std::fmt::Debug;
use std::fmt::Display;
use std::hash::Hash;

mod partition;
use partition::Partition;

mod aggregate_graph;
use aggregate_graph::AggregateGraph;

pub enum QualityFunction {
    Modularity,
    CPM,
}

pub fn leiden<T, A>(
    graph: &Graph<T, A>,
    weighted: bool,
    quality_function: QualityFunction,
    resolution: Option<f64>,
    theta: Option<f64>,
    gamma: Option<f64>,
) -> Result<Vec<HashSet<T>>, Error>
where
    T: Hash + Eq + Clone + Ord + Debug + Display + Send + Sync,
    A: Clone + Send + Sync,
{
    let _resolution = resolution.unwrap_or(0.25);
    let _theta = theta.unwrap_or(0.3);
    let _gamma = gamma.unwrap_or(0.05);
    let mut aggregate_graph = AggregateGraph::initial(graph, weighted);
    let mut partition = get_singleton_partition(graph, weighted);
    let mut prev_partition: Option<Partition> = None;
    loop {
        partition = move_nodes_fast(
            &aggregate_graph.graph,
            &mut partition,
            weighted,
            &quality_function,
            _resolution,
        );
        if partitions::partition_is_singleton(&partition.partition, graph.number_of_nodes())
            || (prev_partition.is_some()
                && partitions::partitions_eq(
                    &partition.partition,
                    &prev_partition.unwrap().partition,
                ))
        {
            let flattened = partition.flatten(&aggregate_graph);
            return Ok(partitions::convert_usize_partitions_to_t(
                flattened.partition,
                &graph,
            ));
        }
        prev_partition = Some(partition.clone());
        // println!("\n");
        let refined_partition = refine_partition(
            &aggregate_graph,
            &partition,
            &quality_function,
            _resolution,
            _theta,
            _gamma,
        );
        // println!("refined_partition {:?}", refined_partition);
        // println!("\n");
        aggregate_graph = aggregate_graph.from_partition(&refined_partition);
        // println!(
        //     "aggregate_graph {:?}",
        //     aggregate_graph
        //         .graph
        //         .get_all_nodes()
        //         .into_iter()
        //         .map(|n| (n.name, n.attributes.unwrap()))
        //         .collect::<Vec<(usize, f64)>>()
        // ); // MALCOLM
        let partitions: Vec<IntSet<usize>> = partition
            .partition
            .iter()
            .map(|c| {
                aggregate_graph
                    .node_nodes
                    .as_ref()
                    .unwrap()
                    .iter()
                    .enumerate()
                    .filter(|(_i, nodes)| nodes.is_subset(c))
                    .map(|(i, _nodes)| i)
                    .collect()
            })
            .collect();
        partition = Partition::from_partition(&aggregate_graph.graph, partitions);
        // println!("partition {:?}", partition);
    }
}

fn move_nodes_fast(
    graph: &Graph<usize, f64>,
    partition: &mut Partition,
    weighted: bool,
    quality_function: &QualityFunction,
    resolution: f64,
) -> Partition {
    let mut queue: VecDeque<usize> = utility::get_shuffled_node_indexes(graph, None).into();
    // let mut queue: VecDeque<usize> = (0..graph.number_of_nodes()).collect::<Vec<usize>>().into();
    while let Some(v) = queue.pop_front() {
        println!("v: {:?}", v);
        let empty = IntSet::default();
        let adjacent_communities = get_adjacent_communities(v, graph, partition, &empty);
        let (max_community, max_delta) = argmax(
            v,
            partition,
            &adjacent_communities,
            graph,
            weighted,
            &quality_function,
            resolution,
        );
        // println!(
        //     "max_community: {:?} max_delta: {}",
        //     max_community, max_delta
        // );
        if max_delta > 0.0 {
            partition.move_node(v, &max_community, graph, weighted);
            let queue_set: IntSet<usize> = queue.iter().cloned().collect();
            for u in graph.get_successor_nodes_by_index(&v) {
                if !max_community.contains(&u.node_index) && !queue_set.contains(&u.node_index) {
                    queue.push_back(u.node_index);
                }
            }
        }
    }
    // println!("done move_nodes_fast");
    // println!("{:?}", partition);
    partition.clone()
}

fn refine_partition(
    aggregate_graph: &AggregateGraph,
    partition: &Partition,
    quality_function: &QualityFunction,
    resolution: f64,
    theta: f64,
    gamma: f64,
) -> Partition {
    let mut refined_partition = get_singleton_partition(&aggregate_graph.graph, true);
    let mut rng: Box<dyn RngCore> = Box::new(rand::thread_rng());
    for community in partition.partition.iter() {
        merge_nodes_subset(
            &mut refined_partition,
            &community,
            aggregate_graph,
            quality_function,
            resolution,
            theta,
            gamma,
            &mut rng,
        );
    }
    refined_partition
}

fn merge_nodes_subset(
    partition: &mut Partition,
    community: &IntSet<usize>,
    aggregate_graph: &AggregateGraph,
    quality_function: &QualityFunction,
    resolution: f64,
    theta: f64,
    gamma: f64,
    rng: &mut Box<dyn RngCore>,
) {
    let size_s = aggregate_graph.node_total(community);
    let R: IntSet<usize> = community
        .iter()
        .map(|v| v.clone())
        .filter(|v| {
            let community_without_v: Vec<usize> = community.without(v).iter().cloned().collect();
            let x = cut_size_by_indexes(&aggregate_graph.graph, &[*v], &community_without_v, true);
            let v_set = vec![*v].into_iter().collect();
            let v_node_total = aggregate_graph.node_total(&v_set);
            x >= gamma * v_node_total * (size_s - v_node_total)
        })
        .collect();
    // println!("R: {:?}", R);
    for v in R.into_iter().sorted() {
        if partition.node_community(v).len() != 1 {
            continue;
        }
        // println!("v: {:?}", v);
        let T: Vec<IntSet<usize>> = partition
            .partition
            .iter()
            .cloned()
            .filter(|C| {
                let nbunch1: Vec<usize> = C.iter().map(|n| n.clone()).collect();
                let nbunch2: Vec<usize> = (community - C).iter().map(|n| n.clone()).collect();
                let cs = cut_size_by_indexes(
                    &aggregate_graph.graph,
                    nbunch1.as_slice(),
                    nbunch2.as_slice(),
                    true,
                );
                let C_node_total = aggregate_graph.node_total(C);
                C.is_subset(community) && cs >= gamma * C_node_total * (size_s - C_node_total)
            })
            .collect();
        // println!("  T: {:?}", T);
        let mut communities: Vec<(&IntSet<usize>, f64)> = T
            .iter()
            .map(|C| {
                (
                    C,
                    get_delta(
                        v,
                        partition,
                        C,
                        &aggregate_graph.graph,
                        true,
                        &quality_function,
                        resolution,
                    ),
                )
            })
            .filter(|(_C, delta)| *delta >= 0.0)
            .collect();
        let weights: Vec<f64> = communities
            .iter()
            .map(|(_C, delta)| (delta / theta).exp())
            .collect();
        let dist = WeightedIndex::new(&weights).unwrap();
        let new_community = communities[dist.sample(rng)];
        // communities.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap()); // MALCOLM
        // let new_community = communities.last().unwrap(); // MALCOLM
        // println!("  new_community: {:?}", new_community);
        partition.move_node(v, new_community.0, &aggregate_graph.graph, true);
    }
}

fn get_singleton_partition<T, A>(graph: &Graph<T, A>, weighted: bool) -> Partition
where
    T: Hash + Eq + Clone + Ord + Display + Send + Sync,
    A: Clone + Send + Sync,
{
    let partition = partitions::get_singleton_partition(graph);
    let node_partition: Vec<usize> = (0..graph.number_of_nodes()).collect();

    let degree_sums: Vec<f64> = match weighted {
        false => graph
            .get_degree_for_all_node_indexes()
            .into_iter()
            .map(|x| x as f64)
            .collect(),
        true => graph.get_weighted_degree_for_all_node_indexes(),
    };
    Partition {
        partition,
        node_partition,
        degree_sums,
    }
}

fn get_adjacent_communities<'a, T, A>(
    node: usize,
    graph: &Graph<T, A>,
    partition: &'a Partition,
    empty: &'a IntSet<usize>,
) -> Vec<&'a IntSet<usize>>
where
    T: Hash + Eq + Clone + Ord + Display + Send + Sync,
    A: Clone + Send + Sync,
{
    let mut adjacent_community_ids: IntSet<usize> = IntSet::default();
    // let mut adjacent_communities: Vec<&IntSet<usize>> = vec![];
    adjacent_community_ids.insert(partition.node_partition[node]);
    for u in graph.get_successor_nodes_by_index(&node) {
        adjacent_community_ids.insert(partition.node_partition[u.node_index]);
    }
    // if node == 2 {
    //     println!("{:?}", partition);
    //     println!("{:?}", adjacent_community_ids);
    // } // MALCOLM
    let mut adjacent_communities: Vec<&IntSet<usize>> = adjacent_community_ids
        .into_iter()
        .map(|i| &partition.partition[i])
        .collect();
    adjacent_communities.push(&empty);
    // println!(
    //     "adjacent_communities for {}: {:?}",
    //     node,
    //     adjacent_communities.len()
    // ); // MALCOLM
    adjacent_communities
}

fn argmax(
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

fn get_delta(
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

    // MALCOLM
    // println!("partition: {:?}", partition);
    // println!("target: {:?}", target);
    // println!("delta | v: {} target: {:?} delta: {}", v, target, delta);
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
    let m = graph.size(weighted);
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

    // let deg_v = match weighted {
    //     true => graph.get_node_weighted_degree_by_index(v),
    //     false => graph.get_node_degree_by_index(v) as f64,
    // };
    // let degs_source = partition.degree_sum(v);
    // let degs_target = match target.len() == 0 {
    //     true => 0.0,
    //     false => partition.degree_sum(*target.into_iter().next().unwrap()),
    // };

    let delta = diff_target
        - diff_source
        - resolution * v_weight * (v_weight + target_weight - source_weight);

    // MALCOLM
    // println!("partition: {:?}", partition);
    // println!("target: {:?}", target);
    // println!("delta | v: {} target: {:?} delta: {}", v, target, delta);
    delta
}

fn single_node_neighbor_cut_size<T, A>(
    graph: &Graph<T, A>,
    v: usize,
    community: &IntSet<usize>,
    weighted: bool,
) -> f64
where
    T: Hash + Eq + Clone + Ord + Display + Send + Sync,
    A: Clone + Send + Sync,
{
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
    use rand::SeedableRng;
    use rand_chacha::ChaCha20Rng;
    use std::sync::Arc;

    #[test]
    fn test_single_node_neighbor_cut_size_1() {
        let edges: Vec<Arc<Edge<i32, ()>>> = vec![
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
        let edges: Vec<Arc<Edge<i32, ()>>> = vec![
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
    fn test_get_adjacent_communities() {
        let nodes = vec![
            Node::from_name(0),
            Node::from_name(1),
            Node::from_name(2),
            Node::from_name(3),
            Node::from_name(4),
        ];
        let edges: Vec<Arc<Edge<i32, ()>>> = vec![
            Edge::new(0, 2),
            Edge::new(1, 2),
            Edge::new(2, 3),
            Edge::new(2, 4),
        ];
        let specs = GraphSpecs::directed_create_missing();
        let graph = Graph::new_from_nodes_and_edges(nodes, edges, specs).unwrap();
        let partition = Partition {
            partition: vec![
                vec![0, 1].into_iter().collect(),
                vec![2].into_iter().collect(),
                vec![3].into_iter().collect(),
                vec![4].into_iter().collect(),
            ],
            node_partition: vec![0, 0, 1, 2, 3],
            degree_sums: vec![0.0, 0.0, 0.0, 0.0],
        };
        let empty = IntSet::default();
        let result = get_adjacent_communities(0, &graph, &partition, &empty);
        assert_eq!(result.len(), 3);
        assert!(result == vec![&partition.partition[0], &partition.partition[1], &empty]);
        let result = get_adjacent_communities(1, &graph, &partition, &empty);
        assert!(result == vec![&partition.partition[0], &partition.partition[1], &empty]);
        let result = get_adjacent_communities(2, &graph, &partition, &empty);
        assert!(
            result
                == vec![
                    &partition.partition[1],
                    &partition.partition[2],
                    &partition.partition[3],
                    &empty
                ]
        );
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

    #[test]
    fn test_move_node() {
        let graph = get_graph_for_argmax(true);
        let mut partition = get_partition_for_argmax();
        let mut target = IntSet::default();
        target.insert(2);
        partition.move_node(0, &target, &graph, true);
        assert_eq!(partition.partition.len(), 4);
        assert!(partition.partition[0] == vec![1].into_iter().collect());
        assert!(partition.partition[1] == vec![0, 2].into_iter().collect());
        assert!(partition.partition[2] == vec![3].into_iter().collect());
        assert!(partition.partition[3] == vec![4].into_iter().collect());
        assert_eq!(partition.node_partition[0], 1);
        assert_eq!(partition.node_partition[1], 0);
        assert_eq!(partition.node_partition[2], 1);
        assert_eq!(partition.node_partition[3], 2);
        assert_eq!(partition.node_partition[4], 3);
        assert!(partition.degree_sums == vec![-1.1, 1.1, 0.0, 0.0]);
    }

    #[test]
    fn test_merge_nodes_subset_1() {
        let (mut partition, community, aggregate_graph) = get_params_for_merge_nodes_subset();
        let mut rng: Box<dyn RngCore> = Box::new(ChaCha20Rng::seed_from_u64(1));
        merge_nodes_subset(
            &mut partition,
            &community,
            &aggregate_graph,
            &QualityFunction::Modularity,
            0.25,
            0.3,
            0.05,
            &mut rng,
        );
        assert_eq!(partition.node_partition, vec![1, 0, 1, 2, 2, 2]);
        assert_eq!(
            partition.partition,
            vec![
                vec![1].into_iter().collect(),
                vec![0, 2].into_iter().collect(),
                vec![3, 4, 5].into_iter().collect(),
            ]
        );
        assert_eq!(partition.degree_sums, vec![3.3, 12.3, 20.5]);
    }

    #[test]
    fn test_merge_nodes_subset_2() {
        let (mut partition, community, aggregate_graph) = get_params_for_merge_nodes_subset();
        let mut rng: Box<dyn RngCore> = Box::new(ChaCha20Rng::seed_from_u64(4));
        merge_nodes_subset(
            &mut partition,
            &community,
            &aggregate_graph,
            &QualityFunction::Modularity,
            0.25,
            0.3,
            0.05,
            &mut rng,
        );
        assert_eq!(partition.node_partition, vec![0, 0, 0, 1, 2, 2]);
        assert_eq!(
            partition.partition,
            vec![
                vec![0, 1, 2].into_iter().collect(),
                vec![3].into_iter().collect(),
                vec![4, 5].into_iter().collect(),
            ]
        );
        assert_eq!(partition.degree_sums, vec![15.600000000000001, 6.2, 12.6]);
    }

    fn get_params_for_merge_nodes_subset<'a>() -> (Partition, IntSet<usize>, AggregateGraph) {
        let nodes = vec![
            Node::from_name(0),
            Node::from_name(1),
            Node::from_name(2),
            Node::from_name(3),
            Node::from_name(4),
            Node::from_name(5),
        ];
        let edges: Vec<Arc<Edge<i32, ()>>> = vec![
            Edge::with_weight(0, 1, 1.1),
            Edge::with_weight(1, 2, 2.2),
            Edge::with_weight(0, 2, 3.7),
            Edge::with_weight(2, 3, 1.7),
            Edge::with_weight(3, 4, 2.1),
            Edge::with_weight(4, 5, 3.2),
            Edge::with_weight(3, 5, 4.1),
        ];
        let graph =
            Graph::new_from_nodes_and_edges(nodes, edges, GraphSpecs::undirected()).unwrap();
        let partition = Partition {
            partition: vec![
                vec![0].into_iter().collect(),
                vec![1].into_iter().collect(),
                vec![2].into_iter().collect(),
                vec![3].into_iter().collect(),
                vec![4].into_iter().collect(),
                vec![5].into_iter().collect(),
            ],
            node_partition: vec![0, 1, 2, 3, 4, 5],
            degree_sums: vec![4.8, 3.3, 7.5, 6.2, 5.3, 7.3],
        };
        let community = vec![0, 1, 2, 3, 4, 5].into_iter().collect();
        let aggregate_graph = AggregateGraph::initial(&graph, true);
        (partition, community, aggregate_graph)
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
        get_adjacent_communities(0, &graph, &partition, empty)
    }
}
