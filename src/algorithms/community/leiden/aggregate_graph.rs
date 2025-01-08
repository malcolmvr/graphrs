use super::Partition;
use crate::{Edge, Graph, GraphSpecs, Node};
use nohash::{IntMap, IntSet};
use std::fmt::Display;
use std::hash::Hash;
use std::sync::Arc;

pub(crate) struct AggregateGraph {
    pub graph: Graph<usize, f64>,
    pub node_nodes: Option<Vec<IntSet<usize>>>,
    pub node_weights: Option<Vec<f64>>,
    pub parent_graph: Option<Box<AggregateGraph>>,
}

impl AggregateGraph {
    pub fn initial<T, A>(graph: &Graph<T, A>, weighted: bool) -> Self
    where
        T: Hash + Eq + Clone + Ord + Display + Send + Sync + PartialOrd,
        A: Clone + Send + Sync,
    {
        let nodes: Vec<Arc<Node<usize, f64>>> = (0..graph.number_of_nodes())
            .into_iter()
            .map(|node_index| Node::from_name_and_attributes(node_index, 1.0))
            .collect();
        let edges: Vec<Arc<Edge<usize, f64>>> = graph
            .get_all_edges()
            .into_iter()
            .map(|edge| {
                let u = graph.get_node_index(&edge.u).unwrap();
                let v = graph.get_node_index(&edge.v).unwrap();
                let weight = match weighted {
                    true => edge.weight,
                    false => 1.0,
                };
                Arc::new(Edge {
                    u,
                    v,
                    weight,
                    attributes: Some(f64::NAN),
                })
            })
            .collect();
        let weighted_graph =
            Graph::<usize, f64>::new_from_nodes_and_edges(nodes, edges, graph.specs.clone())
                .unwrap();

        AggregateGraph {
            graph: weighted_graph,
            node_nodes: None,
            node_weights: None,
            parent_graph: None,
        }
    }

    pub fn find_original_graph(&self) -> &Graph<usize, f64> {
        match self.parent_graph {
            Some(ref parent) => parent.find_original_graph(),
            None => &self.graph,
        }
    }

    pub fn collect_nodes(&self, nodes: &IntSet<usize>) -> IntSet<usize> {
        if self.parent_graph.is_none() {
            return nodes.clone();
        }
        let parent = self.parent_graph.as_ref().unwrap();
        nodes
            .into_iter()
            .flat_map(|node| parent.collect_nodes(&self.node_nodes.as_ref().unwrap()[*node]))
            .collect()
    }

    pub fn node_total(&self, community: &IntSet<usize>) -> f64 {
        if self.node_weights.is_none() {
            return community.len() as f64;
        }
        community
            .iter()
            .map(|node| self.node_weights.as_ref().unwrap()[*node])
            .sum()
    }

    pub fn from_partition(self, partition: &Partition) -> AggregateGraph {
        println!(
            "self.graph.nodes {:?}",
            self.graph
                .get_all_nodes()
                .into_iter()
                .map(|n| (n.name, n.attributes.unwrap()))
                .collect::<Vec<(usize, f64)>>()
        ); // MALCOLM
        let node_nodes = partition.partition.iter().map(|c| c.clone()).collect();
        let node_weights: Vec<f64> = partition
            .partition
            .iter()
            .map(|c| {
                c.iter()
                    .map(|n| self.graph.get_node_by_index(n).unwrap().attributes.unwrap())
                    .sum::<f64>()
            })
            .collect();
        println!("node_weights {:?}", node_weights);
        let new_nodes: Vec<Arc<Node<usize, f64>>> = partition
            .partition
            .iter()
            .enumerate()
            .map(|(i, _c)| Node::from_name_and_attributes(i, node_weights[i]))
            .collect();
        let mut new_edge_weights = IntMap::<usize, IntMap<usize, f64>>::default();
        self.graph.get_all_edges().into_iter().for_each(|edge| {
            let mut u_com = partition.node_partition[edge.u];
            let mut v_com = partition.node_partition[edge.v];
            if u_com > v_com {
                (u_com, v_com) = (v_com, u_com);
            }
            let weight = new_edge_weights
                .entry(u_com)
                .or_insert_with(IntMap::default)
                .entry(v_com)
                .or_insert(0.0);
            *weight += edge.weight;
        });
        let new_edges: Vec<Arc<Edge<usize, f64>>> = new_edge_weights
            .into_iter()
            .flat_map(|(u_com, v_weights)| {
                v_weights
                    .into_iter()
                    .map(move |(v_com, weight)| Edge::with_weight(u_com, v_com, weight))
            })
            .collect();
        let new_graph: Graph<usize, f64> = Graph::new_from_nodes_and_edges(
            new_nodes,
            new_edges,
            GraphSpecs {
                directed: false,
                self_loops: true,
                ..self.graph.specs.clone()
            },
        )
        .unwrap();
        for edge in new_graph.get_all_edges().iter() {
            println!("{} {} {}", edge.u, edge.v, edge.weight);
        }
        AggregateGraph {
            graph: new_graph,
            node_nodes: Some(node_nodes),
            node_weights: Some(node_weights),
            parent_graph: Some(Box::new(self)),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::GraphSpecs;

    #[test]
    fn test_from_partition() {
        let graph = get_graph(false);
        let partition = Partition {
            node_partition: vec![0, 0, 1, 1, 1],
            partition: vec![
                vec![0, 1].into_iter().collect(),
                vec![2, 3, 4].into_iter().collect(),
            ],
            degree_sums: vec![0.0, 0.0, 0.0, 0.0, 0.0],
        };
        let aggregate_graph = AggregateGraph::initial(&graph, true);
        let aggregate_graph = aggregate_graph.from_partition(&partition);
        assert_eq!(aggregate_graph.graph.number_of_nodes(), 2);
        assert_eq!(
            aggregate_graph.node_nodes,
            Some(vec![
                vec![0, 1].into_iter().collect(),
                vec![2, 3, 4].into_iter().collect(),
            ])
        );
        assert_eq!(aggregate_graph.node_weights, Some(vec![2.0, 3.0]));
        assert_eq!(
            aggregate_graph
                .parent_graph
                .unwrap()
                .graph
                .number_of_nodes(),
            5
        );
        assert_eq!(aggregate_graph.graph.number_of_nodes(), 2);
        assert_eq!(aggregate_graph.graph.number_of_edges(), 2);
        assert_eq!(aggregate_graph.graph.get_edge(0, 1).unwrap().weight, 3.4);
        assert_eq!(aggregate_graph.graph.get_edge(1, 1).unwrap().weight, 8.2);
    }

    #[test]
    fn test_find_original_graph() {
        let graph = get_graph(false);
        let aggregate_graph = AggregateGraph::initial(&graph, false);
        let partition = Partition {
            node_partition: vec![0, 0, 1, 1, 2],
            partition: vec![
                vec![0, 1].into_iter().collect(),
                vec![2, 3].into_iter().collect(),
                vec![4].into_iter().collect(),
            ],
            degree_sums: vec![0.0, 0.0, 0.0, 0.0, 0.0],
        };
        let aggregate_graph = aggregate_graph.from_partition(&partition);
        let partition = Partition {
            node_partition: vec![0, 1, 2],
            partition: vec![
                vec![0, 1].into_iter().collect(),
                vec![2].into_iter().collect(),
            ],
            degree_sums: vec![0.0, 0.0, 0.0],
        };
        let aggregate_graph = aggregate_graph.from_partition(&partition);
        let original_graph = aggregate_graph.find_original_graph();
        assert_eq!(original_graph.number_of_nodes(), 5);
    }

    #[test]
    fn test_collect_nodes_1() {
        let graph = get_graph(false);
        let aggregate_graph = AggregateGraph::initial(&graph, false);
        let partition = Partition {
            node_partition: vec![0, 0, 1, 1, 1],
            partition: vec![
                vec![0, 1].into_iter().collect(),
                vec![2, 3, 4].into_iter().collect(),
            ],
            degree_sums: vec![0.0, 0.0, 0.0, 0.0, 0.0],
        };
        let aggregate_graph = aggregate_graph.from_partition(&partition);
        let nodes = vec![0].into_iter().collect();
        let result = aggregate_graph.collect_nodes(&nodes);
        assert_eq!(result, vec![0, 1].into_iter().collect());

        let nodes = vec![1].into_iter().collect();
        let result = aggregate_graph.collect_nodes(&nodes);
        assert_eq!(result, vec![2, 3, 4].into_iter().collect());

        let nodes = vec![0, 1].into_iter().collect();
        let result = aggregate_graph.collect_nodes(&nodes);
        assert_eq!(result, vec![0, 1, 2, 3, 4].into_iter().collect());
    }

    #[test]
    fn test_collect_nodes_2() {
        let graph = get_graph(true);
        let aggregate_graph = AggregateGraph::initial(&graph, false);
        let partition = Partition {
            node_partition: vec![0, 0, 1, 1, 1],
            partition: vec![
                vec![0, 1].into_iter().collect(),
                vec![2, 3, 4].into_iter().collect(),
            ],
            degree_sums: vec![0.0, 0.0, 0.0, 0.0, 0.0],
        };
        let aggregate_graph = aggregate_graph.from_partition(&partition);

        let nodes = vec![0].into_iter().collect();
        let result = aggregate_graph.collect_nodes(&nodes);
        assert_eq!(result, vec![0, 1].into_iter().collect());

        let nodes = vec![1].into_iter().collect();
        let result = aggregate_graph.collect_nodes(&nodes);
        assert_eq!(result, vec![2, 3, 4].into_iter().collect());

        let nodes = vec![0, 1].into_iter().collect();
        let result = aggregate_graph.collect_nodes(&nodes);
        assert_eq!(result, vec![0, 1, 2, 3, 4].into_iter().collect());
    }

    #[test]
    fn test_collect_nodes_3() {
        let graph = get_graph(false);
        let aggregate_graph = AggregateGraph::initial(&graph, false);
        let partition = Partition {
            node_partition: vec![0, 0, 1, 1, 2],
            partition: vec![
                vec![0, 1].into_iter().collect(),
                vec![2, 3].into_iter().collect(),
                vec![4].into_iter().collect(),
            ],
            degree_sums: vec![0.0, 0.0, 0.0, 0.0, 0.0],
        };
        let aggregate_graph = aggregate_graph.from_partition(&partition);
        let partition = Partition {
            node_partition: vec![0, 1, 2],
            partition: vec![
                vec![0, 1].into_iter().collect(),
                vec![2].into_iter().collect(),
            ],
            degree_sums: vec![0.0, 0.0, 0.0],
        };
        let aggregate_graph = aggregate_graph.from_partition(&partition);

        let nodes = vec![0].into_iter().collect();
        let result = aggregate_graph.collect_nodes(&nodes);
        assert_eq!(result, vec![0, 1, 2, 3].into_iter().collect());
    }

    fn get_graph(directed: bool) -> Graph<i32, ()> {
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
}
