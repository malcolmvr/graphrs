use crate::algorithms::shortest_path::dijkstra;
use crate::algorithms::shortest_path::ShortestPathInfo;
use crate::{Error, Graph, Node};
use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;

/**
Compute the shortest-path (Dijkstra) betweenness centrality for nodes.

# Arguments

* `graph`: a [Graph](../../../struct.Graph.html) instance
* `weighted`: set to `true` to use edge weights when computing the betweenness centrality
* `normalized`: set to `true` to normalize the node centrality values

# Examples

```
use graphrs::{algorithms::{centrality::{betweenness}}, generators};
let graph = generators::social::karate_club_graph();
let centralities = betweenness::betweenness_centrality(&graph, false, true);
```

# References

1. Ulrik Brandes: A Faster Algorithm for Betweenness Centrality. Journal of Mathematical Sociology 25(2):163-177, 2001.
*/
pub fn betweenness_centrality<T, A>(
    graph: &Graph<T, A>,
    weighted: bool,
    normalized: bool,
) -> Result<HashMap<T, f64>, Error>
where
    T: Hash + Eq + Clone + Ord + Display + Send + Sync,
    A: Clone + Send + Sync,
{
    let all_pairs = dijkstra::all_pairs(graph, weighted, None, false);
    match all_pairs {
        Err(e) => Err(e),
        Ok(ap) => {
            let mut between_counts = get_between_counts(&ap);
            add_missing_nodes_to_between_counts(&mut between_counts, &graph.get_all_nodes());
            let rescaled = rescale(
                between_counts,
                graph.get_all_nodes().len(),
                normalized,
                graph.specs.directed,
            );
            Ok(rescaled)
        }
    }
}

fn add_missing_nodes_to_between_counts<T, A>(
    between_counts: &mut HashMap<T, f64>,
    nodes: &[&Node<T, A>],
) where
    T: Hash + Eq + Clone + Ord + Display + Send + Sync,
{
    for node in nodes {
        between_counts.entry(node.name.clone()).or_insert(0.0);
    }
}

fn get_between_counts<T>(pairs: &HashMap<T, HashMap<T, ShortestPathInfo<T>>>) -> HashMap<T, f64>
where
    T: Hash + Eq + Clone + Ord + Display,
{
    let short_paths = pairs.values().flat_map(|x| x.values());
    short_paths.flat_map(|sp| get_node_counts(&sp.paths)).fold(
        HashMap::<T, f64>::new(),
        |mut acc, (node, count)| {
            *acc.entry(node).or_insert(0.0) += count;
            acc
        },
    )
}

fn get_node_counts<T>(paths: &[Vec<T>]) -> Vec<(T, f64)>
where
    T: Clone,
{
    let paths_count = paths.len() as f64;
    paths
        .iter()
        .filter(|path| path.len() > 2)
        .flat_map(|path| &path[1..(path.len() - 1)])
        .map(|node| (node.clone(), 1.0 / paths_count))
        .collect()
}

fn rescale<T>(
    node_counts: HashMap<T, f64>,
    num_nodes: usize,
    normalized: bool,
    directed: bool,
) -> HashMap<T, f64>
where
    T: Hash + Eq + Clone + Ord + Display,
{
    let scale = get_scale(num_nodes, normalized, directed);
    match scale {
        None => node_counts,
        Some(s) => node_counts
            .iter()
            .map(|(k, v)| (k.clone(), v * s))
            .collect(),
    }
}

fn get_scale(num_nodes: usize, normalized: bool, directed: bool) -> Option<f64> {
    match normalized {
        true => match num_nodes <= 2 {
            true => None,
            false => Some(1.0 / ((num_nodes as f64 - 1.0) * (num_nodes as f64 - 2.0))),
        },
        false => match directed {
            true => None,
            false => Some(0.5),
        },
    }
}

// tests for private methods only; other tests are in:
// tests/test_algorithms_centrality_betweenness
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_get_node_counts_1() {
        let result = get_node_counts::<&str>(&[]);
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_get_node_counts_2() {
        let result = get_node_counts(&[vec!["n1", "n3"]]);
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_get_node_counts_3() {
        let result = get_node_counts(&[vec!["n1", "n2", "n3"]]);
        assert_eq!(result, vec![("n2", 1.0)]);
    }

    #[test]
    fn test_get_node_counts_4() {
        let result = get_node_counts(&[vec!["n1", "n3"], vec!["n1", "n2", "n3"]]);
        assert_eq!(result, vec![("n2", 0.5)]);
    }

    #[test]
    fn test_get_node_counts_5() {
        let result = get_node_counts(&[vec!["n1", "n2", "n3", "n4", "n5"]]);
        assert_eq!(result, vec![("n2", 1.0), ("n3", 1.0), ("n4", 1.0)]);
    }

    #[test]
    fn test_get_node_counts_6() {
        let result = get_node_counts(&[
            vec!["n1", "n2", "n3", "n4", "n5"],
            vec!["n1", "n2", "n6", "n5"],
        ]);
        assert_eq!(
            result,
            vec![
                ("n2", 0.5),
                ("n3", 0.5),
                ("n4", 0.5),
                ("n2", 0.5),
                ("n6", 0.5)
            ]
        );
    }

    #[test]
    fn test_get_between_counts_1() {
        let mut pairs: HashMap<&str, HashMap<&str, ShortestPathInfo<&str>>> = HashMap::new();
        let mut hm1: HashMap<&str, ShortestPathInfo<&str>> = HashMap::new();
        hm1.insert(
            "n3",
            ShortestPathInfo {
                distance: 3.0,
                paths: vec![vec!["n1", "n2", "n3"], vec!["n1", "n4", "n3"]],
            },
        );
        pairs.insert("n1", hm1);
        let mut hm1: HashMap<&str, ShortestPathInfo<&str>> = HashMap::new();
        hm1.insert(
            "n9",
            ShortestPathInfo {
                distance: 3.0,
                paths: vec![vec!["n7", "n8", "n9"], vec!["n7", "n2", "n9"]],
            },
        );
        pairs.insert("n7", hm1);
        let result = get_between_counts(&pairs);
        assert!(result.get("n1").is_none());
        assert_eq!(result.get("n2").unwrap(), &1.0);
        assert!(result.get("n3").is_none());
        assert_eq!(result.get("n4").unwrap(), &0.5);
        assert!(result.get("n7").is_none());
        assert_eq!(result.get("n8").unwrap(), &0.5);
        assert!(result.get("n9").is_none());
    }

    #[test]
    fn test_get_scale_1() {
        let result = get_scale(10, true, true).unwrap();
        assert_eq!(result, 1.0 / 72.0);
    }

    #[test]
    fn test_get_scale_2() {
        let result = get_scale(2, true, true);
        assert!(result.is_none());
    }

    #[test]
    fn test_get_scale_3() {
        let result = get_scale(2, false, true);
        assert!(result.is_none());
    }

    #[test]
    fn test_get_scale_4() {
        let result = get_scale(10, true, false).unwrap();
        assert_eq!(result, 1.0 / 72.0);
    }

    #[test]
    fn test_get_scale_5() {
        let result = get_scale(10, false, false).unwrap();
        assert_eq!(result, 0.5);
    }

    #[test]
    fn test_add_missing_nodes_to_between_counts() {
        let mut between_counts: HashMap<&str, f64> = HashMap::new();
        between_counts.insert("n1", 1.0);
        between_counts.insert("n4", 4.0);
        let n2 = Node::from_name("n2");
        let n3 = Node::from_name("n3");
        let nodes: Vec<&Node<&str, ()>> = vec![&n2, &n3];
        add_missing_nodes_to_between_counts(&mut between_counts, &nodes);
        assert_eq!(between_counts.get("n1").unwrap(), &1.0);
        assert_eq!(between_counts.get("n2").unwrap(), &0.0);
        assert_eq!(between_counts.get("n3").unwrap(), &0.0);
        assert_eq!(between_counts.get("n4").unwrap(), &4.0);
    }
}
