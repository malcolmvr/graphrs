use crate::{
    algorithms::community::partitions, algorithms::community::utility,
    algorithms::cuts::cut_size_by_indexes, ext::hashset::IntSetExt, Error, ErrorKind, Graph,
};
use core::f64;
use nohash::IntSet;
use rand::distributions::Distribution;
use rand::distributions::WeightedIndex;
use rand::prelude::StdRng;
use std::collections::{HashSet, VecDeque};
use std::fmt::Debug;
use std::fmt::Display;
use std::hash::Hash;

mod partition;
use partition::Partition;

mod aggregate_graph;
use aggregate_graph::AggregateGraph;

mod quality;
pub use quality::QualityFunction;
use quality::{argmax, get_delta};

/**
Returns the best partition of a graph, using the Leiden algorithm.

The Leiden algorithm is considered better than the Louvain algorithm,
as it is more accurate and faster. See the paper "From Louvain to Leiden:
guaranteeing well-connected communities" by V.A. Traag, L. Waltman and N.J. van Eck.

# Arguments

* `graph`: a [Graph](../../../struct.Graph.html) instance
* `weighted`: set to `true` to use edge weights when determining communities
* `quality_function`: the quality function to use, either modularity or Constant Potts Model (CPM)
* `resolution`: larger values result in smaller communities; default 0.25
* `theta`: the θ parameter of the Leiden method, which determines the randomness in the refinement phase of the Leiden algorithm; default 0.3
* `gamma`: the γ parameter of the Leiden method, which also controls the granularity of the communities; default 0.05

# Examples

```
use graphrs::{algorithms::community::leiden::{leiden, QualityFunction}, generators};
let graph = generators::social::karate_club_graph();
let communities = leiden(&graph, true, QualityFunction::CPM, None, None, None);
assert_eq!(communities.unwrap().len(), 4);
```
*/
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
    if graph.specs.directed {
        return Err(Error {
            kind: ErrorKind::WrongMethod,
            message: "The Leiden algorithm does not supported drected graphs. \
            Consider using the `to_undirected` method to convert your graph."
                .to_string(),
        });
    }
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
        let refined_partition = refine_partition(
            &aggregate_graph,
            &partition,
            &quality_function,
            _resolution,
            _theta,
            _gamma,
        );
        aggregate_graph = aggregate_graph.from_partition(&refined_partition);
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
    while let Some(v) = queue.pop_front() {
        let empty = IntSet::default();
        let adjacent_communities = partition.get_adjacent_communities(v, graph, &empty);
        let (max_community, max_delta) = argmax(
            v,
            partition,
            &adjacent_communities,
            graph,
            weighted,
            &quality_function,
            resolution,
        );
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
    let mut rng: StdRng = utility::get_rng(None);
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
    rng: &mut StdRng,
) {
    let size_s = aggregate_graph.node_total(community);
    let communities_of_size: IntSet<usize> = community
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
    for v in communities_of_size {
        if partition.node_community(v).len() != 1 {
            continue;
        }
        let filtered: Vec<IntSet<usize>> = partition
            .partition
            .iter()
            .cloned()
            .filter(|part| {
                let nbunch1: Vec<usize> = part.iter().map(|n| n.clone()).collect();
                let nbunch2: Vec<usize> = (community - part).iter().map(|n| n.clone()).collect();
                let cs = cut_size_by_indexes(
                    &aggregate_graph.graph,
                    nbunch1.as_slice(),
                    nbunch2.as_slice(),
                    true,
                );
                let part_node_total = aggregate_graph.node_total(part);
                part.is_subset(community)
                    && cs >= gamma * part_node_total * (size_s - part_node_total)
            })
            .collect();
        let communities: Vec<(&IntSet<usize>, f64)> = filtered
            .iter()
            .map(|fc| {
                (
                    fc,
                    get_delta(
                        v,
                        partition,
                        fc,
                        &aggregate_graph.graph,
                        true,
                        &quality_function,
                        resolution,
                    ),
                )
            })
            .filter(|(_fc, delta)| *delta >= 0.0)
            .collect();
        let weights: Vec<f64> = communities
            .iter()
            .map(|(_fc, delta)| (delta / theta).exp())
            .collect();
        let dist = WeightedIndex::new(&weights).unwrap();
        let new_community = communities[dist.sample(rng)];
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

#[cfg(test)]
mod tests {

    use super::*;
    use crate::{Edge, Graph, GraphSpecs, Node};
    use std::sync::Arc;

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
        let mut rng: StdRng = utility::get_rng(Some(1));
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
        assert_eq!(partition.node_partition, vec![0, 0, 0, 1, 1, 1]);
        assert_eq!(
            partition.partition,
            vec![
                vec![0, 1, 2].into_iter().collect(),
                vec![3, 4, 5].into_iter().collect(),
            ]
        );
        assert_eq!(partition.degree_sums, vec![15.600000000000001, 18.9]);
    }

    #[test]
    fn test_merge_nodes_subset_2() {
        let (mut partition, community, aggregate_graph) = get_params_for_merge_nodes_subset();
        let mut rng: StdRng = utility::get_rng(Some(3));
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
        assert_eq!(partition.node_partition, vec![0, 1, 0, 2, 2, 2]);
        assert_eq!(
            partition.partition,
            vec![
                vec![0, 2].into_iter().collect(),
                vec![1].into_iter().collect(),
                vec![3, 4, 5].into_iter().collect(),
            ]
        );
        assert_eq!(partition.degree_sums, vec![10.8, 3.3, 18.9]);
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
            Edge::with_weight(2, 3, 0.1),
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
}
