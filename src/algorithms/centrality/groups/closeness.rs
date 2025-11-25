use crate::{Error, ErrorKind, Graph};
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display};
use std::hash::Hash;

/**
Compute the group closeness centrality for a group of nodes.

Group closeness centrality of a group of nodes S is a measure of how close
the group is to the other nodes in the graph.

This implementation follows NetworkX's exact algorithm:
c_close(S) = |V-S| / sum(d_S,v for v in V-S)
where d_S,v = min(d_u,v for u in S)

Note: NetworkX does not actually have normalization for group closeness centrality.
The normalized parameter is kept for API compatibility but is ignored.

# Arguments

* `graph`: a Graph instance
* `group`: a HashSet containing the nodes in the group
* `normalized`: kept for API compatibility, but ignored (NetworkX has no normalization)

# References

1. NetworkX group_closeness_centrality source code
*/
pub fn group_closeness_centrality<T, A>(
    graph: &Graph<T, A>,
    group: &HashSet<T>,
    _normalized: bool, // NetworkX doesn't actually normalize, kept for API compatibility
) -> Result<f64, Error>
where
    T: Hash + Eq + Clone + Ord + Debug + Display + Send + Sync,
    A: Clone + Send + Sync,
{
    // Input validation
    let mut all_nodes: Vec<T> = graph
        .get_all_nodes()
        .iter()
        .map(|n| n.name.clone())
        .collect();
    all_nodes.sort();

    let group_vec: Vec<T> = group.iter().cloned().collect();
    let group_set: HashSet<T> = group_vec.iter().cloned().collect();

    let all_nodes_set: HashSet<T> = all_nodes.iter().cloned().collect();
    let missing_nodes: Vec<T> = group_set.difference(&all_nodes_set).cloned().collect();
    if !missing_nodes.is_empty() {
        return Err(Error {
            kind: ErrorKind::NodeNotFound,
            message: format!("The node(s) {:?} are not in the graph", missing_nodes),
        });
    }

    let n = all_nodes.len();
    let c = group_set.len();

    if c == 0 {
        return Err(Error {
            kind: ErrorKind::InvalidArgument,
            message: "Group cannot be empty".to_string(),
        });
    }

    if c >= n {
        return Err(Error {
            kind: ErrorKind::InvalidArgument,
            message: format!(
                "Group size {} cannot be greater than or equal to total nodes {}",
                c, n
            ),
        });
    }

    // Use multi-source shortest path computation (like NetworkX)
    let shortest_paths = multi_source_shortest_paths(graph, &group_vec)?;

    // Get all non-group nodes
    let non_group_nodes: HashSet<T> = all_nodes_set.difference(&group_set).cloned().collect();

    // Sum distances from group to all non-group nodes
    let mut sum_distances = 0.0;
    for node in &non_group_nodes {
        if let Some(&distance) = shortest_paths.get(node) {
            sum_distances += distance;
        }
        // NetworkX treats unreachable nodes as contributing 0 to the sum (not infinity)
    }

    // NetworkX formula: |V-S| / sum_distances
    if sum_distances == 0.0 {
        return Ok(0.0);
    }

    let closeness = non_group_nodes.len() as f64 / sum_distances;
    Ok(closeness)
}

/**
Compute shortest path distances from multiple sources using multi-source BFS.
This is equivalent to NetworkX's multi_source_dijkstra_path_length for unweighted graphs.
*/
fn multi_source_shortest_paths<T, A>(
    graph: &Graph<T, A>,
    sources: &[T],
) -> Result<HashMap<T, f64>, Error>
where
    T: Hash + Eq + Clone + Ord + Debug + Display + Send + Sync,
    A: Clone + Send + Sync,
{
    let mut distances: HashMap<T, f64> = HashMap::new();
    let mut queue = VecDeque::new();

    // Initialize distances from all sources to 0 and add them to queue
    for source in sources {
        distances.insert(source.clone(), 0.0);
        queue.push_back((source.clone(), 0.0));
    }

    // Multi-source BFS
    while let Some((current, current_dist)) = queue.pop_front() {
        // Skip if we've already found a shorter path to this node
        if let Some(&existing_dist) = distances.get(&current) {
            if current_dist > existing_dist {
                continue;
            }
        }

        if let Ok(neighbors) = graph.get_neighbor_nodes(current.clone()) {
            for neighbor in neighbors {
                let new_distance = current_dist + 1.0;

                // Only update if we found a shorter path (or this is first path)
                let should_update = if let Some(&existing_dist) = distances.get(&neighbor.name) {
                    new_distance < existing_dist
                } else {
                    true
                };

                if should_update {
                    distances.insert(neighbor.name.clone(), new_distance);
                    queue.push_back((neighbor.name.clone(), new_distance));
                }
            }
        }
    }

    Ok(distances)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Edge, Graph, GraphSpecs};

    #[test]
    fn test_group_closeness_basic() {
        let mut graph = Graph::<i32, ()>::new(GraphSpecs::undirected_create_missing());

        // Create a simple path: 0-1-2-3-4
        graph.add_edge(Edge::new(0, 1)).unwrap();
        graph.add_edge(Edge::new(1, 2)).unwrap();
        graph.add_edge(Edge::new(2, 3)).unwrap();
        graph.add_edge(Edge::new(3, 4)).unwrap();

        let mut group = HashSet::new();
        group.insert(2);

        let centrality = group_closeness_centrality(&graph, &group, false).unwrap();
        assert!(centrality > 0.0);

        let normalized_centrality = group_closeness_centrality(&graph, &group, true).unwrap();
        assert!(normalized_centrality > 0.0);
        assert!(normalized_centrality <= 1.0);
    }

    #[test]
    fn test_group_closeness_star() {
        let mut graph = Graph::<i32, ()>::new(GraphSpecs::undirected_create_missing());

        // Create a star graph: 0 connected to 1, 2, 3, 4
        graph.add_edge(Edge::new(0, 1)).unwrap();
        graph.add_edge(Edge::new(0, 2)).unwrap();
        graph.add_edge(Edge::new(0, 3)).unwrap();
        graph.add_edge(Edge::new(0, 4)).unwrap();

        let mut group = HashSet::new();
        group.insert(0);

        let centrality = group_closeness_centrality(&graph, &group, false).unwrap();
        // For a star graph with center node in group:
        // Non-group nodes: {1, 2, 3, 4} (4 nodes)
        // Distance from group to each leaf node is 1, so total distance is 4
        // NetworkX formula: |V-S| / sum_distances = 4/4 = 1.0
        assert!((centrality - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_invalid_group_node() {
        let mut graph = Graph::<i32, ()>::new(GraphSpecs::undirected_create_missing());
        graph.add_edge(Edge::new(0, 1)).unwrap();

        let mut group = HashSet::new();
        group.insert(99);

        let result = group_closeness_centrality(&graph, &group, false);
        assert!(result.is_err());
    }

    #[test]
    fn test_empty_group() {
        let mut graph = Graph::<i32, ()>::new(GraphSpecs::undirected_create_missing());
        graph.add_edge(Edge::new(0, 1)).unwrap();

        let group = HashSet::new();

        let result = group_closeness_centrality(&graph, &group, false);
        assert!(result.is_err());
    }

    #[test]
    fn test_deterministic_behavior() {
        let graph = crate::generators::social::karate_club_graph();
        let mut group = HashSet::new();
        group.insert(0);
        group.insert(33);

        let result1 = group_closeness_centrality(&graph, &group, false).unwrap();
        let result2 = group_closeness_centrality(&graph, &group, false).unwrap();

        assert!((result1 - result2).abs() < f64::EPSILON);
    }
}
