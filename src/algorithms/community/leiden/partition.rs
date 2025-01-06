use crate::Graph;
use nohash::IntSet;
use std::fmt::Display;
use std::hash::Hash;

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

    pub fn move_node<T, A>(
        &mut self,
        v: usize,
        target: &IntSet<usize>,
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
