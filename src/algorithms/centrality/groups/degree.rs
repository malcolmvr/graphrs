use crate::{Error, ErrorKind, Graph};
use rayon::prelude::*;
use std::collections::HashSet;
use std::fmt::{Debug, Display};
use std::hash::Hash;

/**
Compute the group degree centrality for a group of nodes.

Group degree centrality measures the fraction of non-group members connected to group members.
It counts the unique non-group nodes that have at least one connection to any group member.

The formula is:
    c_D(C) = |N(C) ∩ (V-C)| / |V-C|

where C is the group, V-C is the set of nodes not in the group, N(C) is the set of
all neighbors of group members, and the intersection gives unique non-group nodes
connected to the group.

For normalized group degree centrality (default), the value is divided by the total
number of non-group nodes: |V-C|.

# Arguments

* `graph`: A Graph instance
* `group`: A group of nodes for which group degree centrality is to be calculated
* `normalized`: If true, group degree centrality is normalized by total non-group nodes

# Returns

Returns the group degree centrality value as a float between 0.0 and 1.0 (when normalized).

# Errors

Returns an error if:
- Any node in the group is not present in the graph
- The group is empty or contains all nodes

# Examples

```
use graphrs::{algorithms::centrality::groups::degree, generators, GraphSpecs};
use std::collections::HashSet;

let graph = generators::social::karate_club_graph();
let mut group = HashSet::new();
group.insert(0);
group.insert(1);
group.insert(2);

let centrality = degree::group_degree_centrality(
    &graph, &group, true
).unwrap();
```

# References

1. M G Everett and S P Borgatti: The Centrality of Groups and Classes.
   Journal of Mathematical Sociology. 23(3): 181-201. 1999.
   http://www.analytictech.com/borgatti/group_centrality.htm
2. NetworkX implementation: https://networkx.org/documentation/stable/reference/algorithms/generated/networkx.algorithms.centrality.group_degree_centrality.html
*/
pub fn group_degree_centrality<T, A>(
    graph: &Graph<T, A>,
    group: &HashSet<T>,
    normalized: bool,
) -> Result<f64, Error>
where
    T: Hash + Eq + Clone + Ord + Debug + Display + Send + Sync,
    A: Clone + Send + Sync,
{
    // Create deterministic collections by sorting nodes
    let mut all_nodes: Vec<T> = graph
        .get_all_nodes()
        .iter()
        .map(|n| n.name.clone())
        .collect();
    all_nodes.sort();
    let all_nodes_set: HashSet<T> = all_nodes.iter().cloned().collect();

    let mut group_vec: Vec<T> = group.iter().cloned().collect();
    group_vec.sort();
    let group_set: HashSet<T> = group_vec.iter().cloned().collect();

    // Validate that all group nodes are in the graph
    let missing_nodes: Vec<T> = group_set.difference(&all_nodes_set).cloned().collect();
    if !missing_nodes.is_empty() {
        return Err(Error {
            kind: ErrorKind::NodeNotFound,
            message: format!("The node(s) {:?} are not in the graph", missing_nodes),
        });
    }

    let n = all_nodes.len();
    let c = group_set.len();

    // Validate group size (must be non-empty and not contain all nodes)
    if c == 0 {
        return Err(Error {
            kind: ErrorKind::InvalidArgument,
            message: "Group cannot be empty".to_string(),
        });
    }

    if c >= n {
        return Err(Error {
            kind: ErrorKind::InvalidArgument,
            message: "Group cannot contain all nodes".to_string(),
        });
    }

    // Get nodes not in the group (V - C) in sorted order for deterministic results
    let mut non_group_nodes: Vec<T> = all_nodes_set.difference(&group_set).cloned().collect();
    non_group_nodes.sort();
    let non_group_set: HashSet<T> = non_group_nodes.iter().cloned().collect();

    // Calculate connected non-group nodes with optimized parallel processing
    let connected_non_group_nodes = if group_vec.len() > 10 && rayon::current_num_threads() > 1 {
        calculate_connected_non_group_nodes_parallel(graph, &group_vec, &non_group_set)?
    } else {
        calculate_connected_non_group_nodes_sequential(graph, &group_vec, &non_group_set)?
    };

    // Apply normalization if requested
    let degree_centrality = if normalized {
        let total_non_group_nodes = n - c;

        if total_non_group_nodes == 0 {
            0.0
        } else {
            connected_non_group_nodes.len() as f64 / total_non_group_nodes as f64
        }
    } else {
        connected_non_group_nodes.len() as f64
    };

    Ok(degree_centrality)
}

/// Calculate connected non-group nodes using parallel processing
fn calculate_connected_non_group_nodes_parallel<T, A>(
    graph: &Graph<T, A>,
    group_nodes: &[T],
    non_group_set: &HashSet<T>,
) -> Result<HashSet<T>, Error>
where
    T: Hash + Eq + Clone + Ord + Debug + Display + Send + Sync,
    A: Clone + Send + Sync,
{
    let connected_sets: Result<Vec<HashSet<T>>, Error> = group_nodes
        .par_iter()
        .map(|node| get_connected_non_group_nodes_for_node(graph, node, non_group_set))
        .collect();

    let connected_sets = connected_sets?;

    // Merge all sets to get unique connected non-group nodes
    let mut all_connected = HashSet::new();
    for set in connected_sets {
        all_connected.extend(set);
    }

    Ok(all_connected)
}

/// Calculate connected non-group nodes using sequential processing
fn calculate_connected_non_group_nodes_sequential<T, A>(
    graph: &Graph<T, A>,
    group_nodes: &[T],
    non_group_set: &HashSet<T>,
) -> Result<HashSet<T>, Error>
where
    T: Hash + Eq + Clone + Ord + Debug + Display + Send + Sync,
    A: Clone + Send + Sync,
{
    let mut connected_non_group_nodes = HashSet::new();

    for node in group_nodes {
        let node_connected = get_connected_non_group_nodes_for_node(graph, node, non_group_set)?;
        connected_non_group_nodes.extend(node_connected);
    }

    Ok(connected_non_group_nodes)
}

/// Get connected non-group nodes for a single node in the group
fn get_connected_non_group_nodes_for_node<T, A>(
    graph: &Graph<T, A>,
    node: &T,
    non_group_set: &HashSet<T>,
) -> Result<HashSet<T>, Error>
where
    T: Hash + Eq + Clone + Ord + Debug + Display + Send + Sync,
    A: Clone + Send + Sync,
{
    let neighbors = if graph.specs.directed {
        // For directed graphs, get outgoing neighbors (successors)
        graph.get_successor_nodes(node.clone())
    } else {
        // For undirected graphs, get all neighbors
        graph.get_neighbor_nodes(node.clone())
    };

    match neighbors {
        Ok(neighbors) => {
            let connected_non_group: HashSet<T> = neighbors
                .iter()
                .filter(|neighbor| non_group_set.contains(&neighbor.name))
                .map(|neighbor| neighbor.name.clone())
                .collect();
            Ok(connected_non_group)
        }
        Err(_) => Ok(HashSet::new()), // Node might have no edges
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Edge, Graph, GraphSpecs};

    #[test]
    fn test_group_degree_basic_undirected() {
        let mut graph = Graph::<i32, ()>::new(GraphSpecs::undirected_create_missing());

        // Create a simple graph: 0-1-2-3-4
        graph
            .add_edges(vec![
                Edge::new(0, 1),
                Edge::new(1, 2),
                Edge::new(2, 3),
                Edge::new(3, 4),
            ])
            .unwrap();

        // Group with nodes 1, 2 (middle nodes)
        let mut group = HashSet::new();
        group.insert(1);
        group.insert(2);

        let centrality = group_degree_centrality(&graph, &group, false).unwrap();

        // Group has 2 external edges: 1-0 and 2-3
        assert_eq!(centrality, 2.0);
    }

    #[test]
    fn test_group_degree_basic_directed() {
        let mut graph = Graph::<i32, ()>::new(GraphSpecs::directed_create_missing());

        // Create a directed graph: 0->1->2->3->4
        graph
            .add_edges(vec![
                Edge::new(0, 1),
                Edge::new(1, 2),
                Edge::new(2, 3),
                Edge::new(3, 4),
            ])
            .unwrap();

        // Group with nodes 1, 2
        let mut group = HashSet::new();
        group.insert(1);
        group.insert(2);

        let centrality = group_degree_centrality(&graph, &group, false).unwrap();

        // Group has 1 external outgoing edge: 2->3
        assert_eq!(centrality, 1.0);
    }

    #[test]
    fn test_group_degree_normalization() {
        let mut graph = Graph::<i32, ()>::new(GraphSpecs::undirected_create_missing());

        // Create a graph: 0-1-2-3
        graph
            .add_edges(vec![Edge::new(0, 1), Edge::new(1, 2), Edge::new(2, 3)])
            .unwrap();

        let mut group = HashSet::new();
        group.insert(1);

        let unnormalized = group_degree_centrality(&graph, &group, false).unwrap();
        let normalized = group_degree_centrality(&graph, &group, true).unwrap();

        // Unnormalized: 2 edges (to 0 and 2)
        assert_eq!(unnormalized, 2.0);
        // Normalized: 2 / (1 * 3) = 2/3
        assert!((normalized - (2.0 / 3.0)).abs() < 1e-10);
    }

    #[test]
    fn test_invalid_group_node() {
        let mut graph = Graph::<i32, ()>::new(GraphSpecs::undirected_create_missing());
        graph.add_edges(vec![Edge::new(0, 1)]).unwrap();

        let mut group = HashSet::new();
        group.insert(999); // Node not in graph

        let result = group_degree_centrality(&graph, &group, false);
        assert!(result.is_err());
    }

    #[test]
    fn test_empty_group() {
        let mut graph = Graph::<i32, ()>::new(GraphSpecs::undirected_create_missing());
        graph.add_edges(vec![Edge::new(0, 1)]).unwrap();

        let group = HashSet::new(); // Empty group

        let result = group_degree_centrality(&graph, &group, false);
        assert!(result.is_err());
    }

    #[test]
    fn test_group_contains_all_nodes() {
        let mut graph = Graph::<i32, ()>::new(GraphSpecs::undirected_create_missing());
        graph.add_edges(vec![Edge::new(0, 1)]).unwrap();

        let mut group = HashSet::new();
        group.insert(0);
        group.insert(1);

        let result = group_degree_centrality(&graph, &group, false);
        assert!(result.is_err());
    }

    #[test]
    fn test_deterministic_behavior() {
        let mut graph = Graph::<i32, ()>::new(GraphSpecs::undirected_create_missing());

        // Create a graph with multiple connections
        graph
            .add_edges(vec![
                Edge::new(0, 1),
                Edge::new(1, 2),
                Edge::new(2, 3),
                Edge::new(3, 4),
                Edge::new(0, 3),
                Edge::new(1, 4),
            ])
            .unwrap();

        // Group with nodes 0, 1
        let mut group = HashSet::new();
        group.insert(0);
        group.insert(1);

        // Run multiple times and ensure results are identical
        let mut results = Vec::new();
        for _ in 0..5 {
            let centrality = group_degree_centrality(&graph, &group, false).unwrap();
            results.push(centrality);
        }

        // All results should be identical
        let first_result = results[0];
        for &result in &results[1..] {
            assert!(
                (result - first_result).abs() < 1e-15,
                "Non-deterministic behavior detected: first={}, other={}, diff={}",
                first_result,
                result,
                (result - first_result).abs()
            );
        }

        // Also test normalized version
        let mut norm_results = Vec::new();
        for _ in 0..3 {
            let centrality = group_degree_centrality(&graph, &group, true).unwrap();
            norm_results.push(centrality);
        }

        let first_norm = norm_results[0];
        for &result in &norm_results[1..] {
            assert!(
                (result - first_norm).abs() < 1e-15,
                "Non-deterministic behavior in normalized version"
            );
        }
    }

    #[test]
    fn test_group_degree_parallel_threshold() {
        let mut graph = Graph::<i32, ()>::new(GraphSpecs::undirected_create_missing());

        // Create a larger graph to test parallel processing
        for i in 0..15 {
            graph.add_edge(Edge::new(i, i + 1)).unwrap();
        }

        // Create a group large enough to trigger parallel processing (>10 nodes)
        let mut group = HashSet::new();
        for i in 0..12 {
            group.insert(i);
        }

        let centrality = group_degree_centrality(&graph, &group, false).unwrap();

        // Should have some external connections
        assert!(centrality > 0.0);
    }
}
