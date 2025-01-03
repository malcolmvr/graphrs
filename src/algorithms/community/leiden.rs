use crate::{
    algorithms::community::partitions, algorithms::community::utility, ext::hashset::IntSetExt,
    AdjacentNode, Edge, EdgeDedupeStrategy, Error, ErrorKind, Graph, GraphSpecs, Node,
};
use nohash::IntSet;
use serde::de;
use std::collections::{HashSet, VecDeque};
use std::fmt::Display;
use std::hash::Hash;

use super::partitions::modularity_by_indexes;

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
    pub fn move_node<T, A>(
        &mut self,
        v: usize,
        target: IntSet<usize>,
        graph: &Graph<T, A>,
        weighted: bool,
    ) where
        T: Hash + Eq + Clone + Ord + Display + Send + Sync,
        A: Clone + Send + Sync,
    {
        let source_partition_idx = self.node_partition[v];
        let target_partition_idx: usize;
        if target.len() > 0 {
            let el = target.iter().next().unwrap();
            target_partition_idx = self.node_partition[*el];
        } else {
            target_partition_idx = self.partition.len();
            self.degree_sums.push(0.0);
        }

        // Remove `v` from its old community and place it into the target partition
        self.partition[source_partition_idx].remove(&v);
        self.partition[target_partition_idx].insert(v);

        // Also update the sum of node degrees in that partition
        let deg_v = match weighted {
            true => graph.get_node_weighted_degree_by_index(v),
            false => graph.get_node_degree_by_index(v) as f64,
        };
        self.degree_sums[source_partition_idx] -= deg_v;
        self.degree_sums[target_partition_idx] += deg_v;

        // Update v's entry in the index lookup table
        self.node_partition[v] = target_partition_idx;

        // If the original partition is empty now, that we removed v from it, remove it and adjust the indexes in _node_part
        if self.partition[source_partition_idx].len() == 0 {
            self.partition.remove(source_partition_idx);
            self.degree_sums.remove(source_partition_idx);
            self.node_partition = self
                .node_partition
                .iter()
                .map(|i| {
                    if *i < source_partition_idx {
                        *i
                    } else {
                        *i - 1
                    }
                })
                .collect();
        }
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
        let adjacent_community_indexes = get_adjacent_communities(v, graph, &partition);
        let mut adjacent_communities: Vec<&IntSet<usize>> = adjacent_community_indexes
            .into_iter()
            .map(|x| &partition.partition[x])
            .collect();
        let empty = IntSet::default();
        adjacent_communities.push(&empty);
        let (max_community, max_delta) = argmax(
            v,
            partition,
            &adjacent_communities,
            graph,
            weighted,
            resolution,
        );
        if max_delta > 0.0 {}
    }
    Partition {
        partition: partition.partition.clone(),
        node_partition: partition.node_partition.clone(),
        degree_sums: partition.degree_sums.clone(),
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

fn argmax<'a, T, A>(
    v: usize,
    partition: &Partition,
    communities: &'a [&IntSet<usize>],
    graph: &Graph<T, A>,
    weighted: bool,
    resolution: f64,
) -> (&'a IntSet<usize>, f64)
where
    T: Hash + Eq + Clone + Ord + Display + Send + Sync,
    A: Clone + Send + Sync,
{
    let mut idx = 0;
    let mut opt = communities[idx];
    let mut val = get_delta(v, partition, opt, graph, weighted, resolution);
    for k in 1..communities.len() {
        let optk = &communities[k];
        let valk = get_delta(v, partition, optk, graph, weighted, resolution);
        if valk > val {
            idx = k;
            opt = optk;
            val = valk;
        }
    }
    (opt, val)
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

#[cfg(test)]
mod tests {

    use super::*;
    use crate::{Edge, Graph, GraphSpecs, Node};
    use assert_approx_eq::assert_approx_eq;
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
        let edges: Vec<Arc<Edge<i32, ()>>> = vec![
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
        let result = get_delta(1, &partition, &target, &graph, true, 1.0);
        assert_approx_eq!(result, -0.11206896551724145);
    }

    #[test]
    fn test_get_delta_2() {
        let edges: Vec<Arc<Edge<i32, ()>>> = vec![
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
        let result = get_delta(1, &partition, &target, &graph, true, 1.0);
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
        let result = get_adjacent_communities(0, &graph, &partition);
        assert_eq!(result.len(), 2);
        assert!(result.contains(&0));
        assert!(result.contains(&1));
        let result = get_adjacent_communities(1, &graph, &partition);
        assert_eq!(result.len(), 2);
        assert!(result.contains(&0));
        assert!(result.contains(&1));
        let result = get_adjacent_communities(2, &graph, &partition);
        assert_eq!(result.len(), 3);
        assert!(result.contains(&1));
        assert!(result.contains(&2));
        assert!(result.contains(&3));
    }

    #[test]
    fn test_argmax_1() {
        let graph = get_graph_for_argmax(true);
        let partition = get_partition_for_argmax();
        let communities = get_communities_for_argmax(&partition, &graph);
        let result = argmax(0, &partition, &communities, &graph, true, 1.0);
        assert_eq!(result.0.len(), 1);
        assert!(result.0.contains(&2));
        assert_approx_eq!(result.1, 0.09033145065398336);
        let result = argmax(0, &partition, &communities, &graph, false, 1.0);
        assert_eq!(result.0.len(), 1);
        assert!(result.0.contains(&2));
        assert_approx_eq!(result.1, 0.21875);
    }

    #[test]
    fn test_argmax_2() {
        let graph = get_graph_for_argmax(false);
        let partition = get_partition_for_argmax();
        let communities = get_communities_for_argmax(&partition, &graph);
        let result = argmax(0, &partition, &communities, &graph, true, 1.0);
        assert_eq!(result.0.len(), 1);
        assert!(result.0.contains(&2));
        assert_approx_eq!(result.1, 0.09033145065398336);
        let result = argmax(0, &partition, &communities, &graph, false, 1.0);
        assert_eq!(result.0.len(), 1);
        assert!(result.0.contains(&2));
        assert_approx_eq!(result.1, 0.21875);
    }

    #[test]
    fn test_move_node() {
        // TODO
    }

    fn get_graph_for_argmax(directed: bool) -> Graph<i32, ()> {
        let nodes = vec![
            Node::from_name(0),
            Node::from_name(1),
            Node::from_name(2),
            Node::from_name(3),
            Node::from_name(4),
        ];
        let edges: Vec<Arc<Edge<i32, ()>>> = vec![
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
        graph: &Graph<i32, ()>,
    ) -> Vec<&'a IntSet<usize>> {
        let community_indexes = get_adjacent_communities(0, &graph, &partition);
        community_indexes
            .into_iter()
            .map(|x| &partition.partition[x])
            .collect()
    }
}
