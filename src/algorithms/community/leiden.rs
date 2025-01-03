use crate::{
    algorithms::community::partitions, algorithms::community::utility, ext::hashset::IntSetExt,
    AdjacentNode, Edge, EdgeDedupeStrategy, Error, ErrorKind, Graph, GraphSpecs, Node,
};
use nohash::IntSet;
use std::collections::{HashSet, VecDeque};
use std::fmt::Display;
use std::hash::Hash;

struct Partition {
    pub node_partition: Vec<usize>,
    pub partition: Vec<IntSet<usize>>,
    pub degree_sums: Vec<f64>,
}

impl Partition {
    pub fn node_community(&self, node: usize) -> &IntSet<usize> {
        &self.partition[self.node_partition[node]]
    }
    pub fn degree_sum(&self, node: usize) -> f64 {
        self.degree_sums[self.node_partition[node]]
    }
}

pub fn leiden<T, A>(
    graph: &Graph<T, A>,
    weighted: bool,
    resolution: Option<f64>,
) -> Result<Vec<HashSet<T>>, Error>
where
    T: Hash + Eq + Clone + Ord + Display + Send + Sync,
    A: Clone + Send + Sync,
{
    let _resolution = resolution.unwrap_or(0.05);
    let partition = get_singleton_partition(graph, weighted);
    // Ok(partitions::convert_usize_partitions_to_t(partition, &graph))
    let mut prev_partition: Option<Partition> = None;
    loop {
        let new_partition = move_nodes_fast(graph, &partition, weighted, _resolution);
        if partitions::partition_is_singleton(&new_partition.partition, graph.number_of_nodes())
            || (prev_partition.is_some()
                && partitions::partitions_eq(
                    &new_partition.partition,
                    &prev_partition.unwrap().partition,
                ))
        {
            return Ok(partitions::convert_usize_partitions_to_t(
                new_partition.partition,
                &graph,
            ));
        }
        prev_partition = Some(new_partition);
    }
}

fn move_nodes_fast<T, A>(
    graph: &Graph<T, A>,
    partition: &Partition,
    weighted: bool,
    resolution: f64,
) -> Partition
where
    T: Hash + Eq + Clone + Ord + Display + Send + Sync,
    A: Clone + Send + Sync,
{
    let mut shuffled_indexes: VecDeque<usize> =
        utility::get_shuffled_node_indexes(graph, None).into();
    while let Some(v) = shuffled_indexes.pop_front() {
        let adjacent_communities = get_adjacent_communities(v, graph, &partition);
        let (max_community, max_delta) = argmax(
            v,
            &adjacent_communities,
            partition,
            graph,
            weighted,
            resolution,
        );
        // argmax
    }
    Partition {
        partition: partition.partition.clone(),
        node_partition: partition.node_partition.clone(),
        degree_sums: partition.degree_sums.clone(),
    }
}

// fn argmax()

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

fn get_adjacent_communities<T, A>(
    node: usize,
    graph: &Graph<T, A>,
    partition: &Partition,
) -> IntSet<usize>
where
    T: Hash + Eq + Clone + Ord + Display + Send + Sync,
    A: Clone + Send + Sync,
{
    let mut adjacent_communities = IntSet::default();
    adjacent_communities.insert(partition.node_partition[node]);
    for u in graph.get_successor_nodes_by_index(&node) {
        adjacent_communities.insert(partition.node_partition[u.node_index]);
    }
    adjacent_communities
}

fn argmax<T, A>(
    node: usize,
    communities: &IntSet<usize>,
    partition: &Partition,
    graph: &Graph<T, A>,
    weighted: bool,
    resolution: f64,
) -> (usize, f64)
where
    T: Hash + Eq + Clone + Ord + Display + Send + Sync,
    A: Clone + Send + Sync,
{
    let mut max_community = partition.node_partition[node];
    let mut max_delta = 0.0;
    for community_index in communities.into_iter() {
        let community = &partition.partition[*community_index];
        let delta = get_delta(node, partition, community, graph, weighted, resolution);
        if delta > max_delta {
            max_delta = delta;
            max_community = *community_index;
        }
    }
    (max_community, max_delta)
}

fn get_delta<T, A>(
    v: usize,
    partition: &Partition,
    target: &IntSet<usize>,
    graph: &Graph<T, A>,
    weighted: bool,
    resolution: f64,
) -> f64
where
    T: Hash + Eq + Clone + Ord + Display + Send + Sync,
    A: Clone + Send + Sync,
{
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

    ((diff_target - diff_source)
        - resolution / (2.0 * m) * (deg_v.powf(2.0) + deg_v * (degs_target - degs_source)))
        / m
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
