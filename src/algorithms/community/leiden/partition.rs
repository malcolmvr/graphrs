use crate::Graph;
use itertools::Itertools;
use nohash::IntSet;

use super::AggregateGraph;

#[derive(Debug, Clone)]
pub(crate) struct Partition {
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

    pub fn move_node(
        &mut self,
        v: usize,
        target: &IntSet<usize>,
        graph: &Graph<usize, f64>,
        weighted: bool,
    ) {
        let source_partition_idx = self.node_partition[v];
        let target_partition_idx: usize;
        if target.len() > 0 {
            let el = target.iter().next().unwrap();
            target_partition_idx = self.node_partition[*el];
        } else {
            target_partition_idx = self.partition.len();
            self.partition.push(target.clone());
            self.degree_sums.push(0.0);
        }

        // println!("source_partition_idx: {:?}", source_partition_idx);
        // println!("target_partition_idx: {:?}", target_partition_idx);

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

    pub fn from_partition(graph: &Graph<usize, f64>, partition: Vec<IntSet<usize>>) -> Partition {
        // println!("degrees: {:?}", graph.get_weighted_degree_for_all_nodes());
        let node_partition: Vec<usize> = partition
            .iter()
            .enumerate()
            .flat_map(|(i, c)| c.iter().map(move |n| (*n, i)))
            .sorted()
            .map(|(_n, i)| i)
            .collect();
        let degree_sums: Vec<f64> = partition
            .iter()
            .map(|c| {
                c.iter()
                    .map(|n| graph.get_node_weighted_degree_by_index(*n))
                    .sum()
            })
            .collect();
        Partition {
            node_partition,
            partition,
            degree_sums,
        }
    }

    pub fn flatten(self, aggregate_graph: &AggregateGraph) -> Self {
        if aggregate_graph.parent_graph.is_none() {
            return self;
        }
        let graph = aggregate_graph.find_original_graph();
        let partitions = self
            .partition
            .iter()
            .map(|p| aggregate_graph.collect_nodes(p))
            .collect();
        Partition::from_partition(graph, partitions)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::{Edge, Graph, GraphSpecs, Node};
    use std::sync::Arc;

    #[test]
    fn test_from_partition_1() {
        let graph = get_graph(false);
        let partition = vec![
            vec![0, 1].into_iter().collect(),
            vec![2, 3, 4].into_iter().collect(),
        ];
        let new_partition = Partition::from_partition(&graph, partition);
        assert_eq!(new_partition.node_partition, vec![0, 0, 1, 1, 1]);
        assert_eq!(new_partition.degree_sums, vec![3.4, 19.8]);
    }

    #[test]
    fn test_from_partition_2() {
        let graph = get_graph(true);
        let partition = vec![
            vec![0, 1].into_iter().collect(),
            vec![2, 3, 4].into_iter().collect(),
        ];
        let new_partition = Partition::from_partition(&graph, partition);
        assert_eq!(new_partition.node_partition, vec![0, 0, 1, 1, 1]);
        assert_eq!(new_partition.degree_sums, vec![3.4, 19.799999999999997]);
    }

    fn get_graph(directed: bool) -> Graph<usize, f64> {
        let nodes = vec![
            Node::from_name_and_attributes(0, f64::NAN),
            Node::from_name_and_attributes(1, f64::NAN),
            Node::from_name_and_attributes(2, f64::NAN),
            Node::from_name_and_attributes(3, f64::NAN),
            Node::from_name_and_attributes(4, f64::NAN),
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
}
