use crate::{Error, ErrorKind, Graph};
use rayon::prelude::*;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display};
use std::hash::Hash;

/**
Compute the group betweenness centrality for a group of nodes using the Borgatti-Everett algorithm.

This implementation follows the NetworkX reference implementation exactly.

# References

1. M G Everett and S P Borgatti: The Centrality of Groups and Classes.
   Journal of Mathematical Sociology. 23(3): 181-201. 1999.
2. NetworkX implementation
*/
pub fn group_betweenness_centrality<T, A>(
    graph: &Graph<T, A>,
    group: &HashSet<T>,
    normalized: bool,
    endpoints: bool,
) -> Result<f64, Error>
where
    T: Hash + Eq + Clone + Ord + Debug + Display + Send + Sync,
    A: Clone + Send + Sync,
{
    // Input validation and setup
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

    let v = all_nodes.len();
    let c = group_set.len();

    if c >= v - 1 {
        return Err(Error {
            kind: ErrorKind::InvalidArgument,
            message: format!(
                "Group size {} must be at most {} (n-2 where n={})",
                c,
                v - 2,
                v
            ),
        });
    }

    if c == 0 {
        return Err(Error {
            kind: ErrorKind::InvalidArgument,
            message: "Group cannot be empty".to_string(),
        });
    }

    // Pre-processing step: follows NetworkX exactly
    let (pb, sigma, distances) = group_preprocessing(graph, &all_nodes, &group_set)?;

    // Convert group to indices for matrix operations
    let node_to_index: HashMap<T, usize> = all_nodes
        .iter()
        .enumerate()
        .map(|(i, node)| (node.clone(), i))
        .collect();

    let group_indices: HashSet<usize> = group_set
        .iter()
        .map(|node| *node_to_index.get(node).unwrap())
        .collect();

    // Apply the Borgatti-Everett algorithm with optimized memory management
    let mut gbc_group = 0.0;
    let mut sigma_m = sigma.clone();
    let mut pb_m = pb.clone();

    // Pre-allocate matrices to reduce heap allocations in the loop
    let matrix_size = sigma.len();
    let mut sigma_m_v = vec![vec![0.0; matrix_size]; matrix_size];
    let mut pb_m_v = vec![vec![0.0; matrix_size]; matrix_size];

    for &v_idx in &group_indices {
        gbc_group += pb_m[v_idx][v_idx];

        // Reuse pre-allocated matrices instead of cloning
        for i in 0..matrix_size {
            for j in 0..matrix_size {
                sigma_m_v[i][j] = sigma_m[i][j];
                pb_m_v[i][j] = pb_m[i][j];
            }
        }

        for &x in &group_indices {
            for &y in &group_indices {
                let mut dxvy = 0.0;
                let mut dxyv = 0.0;
                let mut dvxy = 0.0;

                if sigma_m[x][y] != 0.0 && sigma_m[x][v_idx] != 0.0 && sigma_m[v_idx][y] != 0.0 {
                    if (distances[x][v_idx] - distances[x][y] - distances[y][v_idx]).abs() < 1e-10 {
                        dxyv = sigma_m[x][y] * sigma_m[y][v_idx] / sigma_m[x][v_idx];
                    }
                    if (distances[x][y] - distances[x][v_idx] - distances[v_idx][y]).abs() < 1e-10 {
                        dxvy = sigma_m[x][v_idx] * sigma_m[v_idx][y] / sigma_m[x][y];
                    }
                    if (distances[v_idx][y] - distances[v_idx][x] - distances[x][y]).abs() < 1e-10 {
                        dvxy = sigma_m[v_idx][x] * sigma[x][y] / sigma[v_idx][y];
                    }
                }

                sigma_m_v[x][y] = sigma_m[x][y] * (1.0 - dxvy);
                pb_m_v[x][y] = pb_m[x][y] - pb_m[x][y] * dxvy;

                if y != v_idx {
                    pb_m_v[x][y] -= pb_m[x][v_idx] * dxyv;
                }
                if x != v_idx {
                    pb_m_v[x][y] -= pb_m[v_idx][y] * dvxy;
                }
            }
        }

        // Use swap to avoid expensive moves and maintain pre-allocated matrices
        std::mem::swap(&mut sigma_m, &mut sigma_m_v);
        std::mem::swap(&mut pb_m, &mut pb_m_v);
    }

    // Endpoints handling (NetworkX defaults to endpoints=False)
    if !endpoints {
        let mut scale = 0.0;

        // For connected graphs
        if !graph.specs.directed && is_connected(&distances, v) {
            scale = (c * (2 * v - c - 1)) as f64;
        } else if graph.specs.directed && is_strongly_connected(&distances, v) {
            scale = (c * (2 * v - c - 1)) as f64;
        } else {
            // Count actual connections for non-connected graphs
            for &group_node1 in &group_indices {
                for node in 0..v {
                    if node != group_node1 && distances[group_node1][node] != f64::INFINITY {
                        if group_indices.contains(&node) {
                            scale += 1.0;
                        } else {
                            scale += 2.0;
                        }
                    }
                }
            }
        }

        gbc_group -= scale;
    }

    // Normalization
    let final_result = if normalized {
        let scale = 1.0 / ((v - c) * (v - c - 1)) as f64;
        gbc_group * scale
    } else {
        // For undirected graphs, divide by 2 when not normalized
        if graph.specs.directed {
            gbc_group
        } else {
            gbc_group / 2.0
        }
    };

    Ok(final_result)
}

/// Pre-processing step that matches NetworkX exactly
fn group_preprocessing<T, A>(
    graph: &Graph<T, A>,
    all_nodes: &[T],
    group_set: &HashSet<T>,
) -> Result<(Vec<Vec<f64>>, Vec<Vec<f64>>, Vec<Vec<f64>>), Error>
where
    T: Hash + Eq + Clone + Ord + Debug + Display + Send + Sync,
    A: Clone + Send + Sync,
{
    let n = all_nodes.len();

    // Create node mapping
    let node_to_index: HashMap<T, usize> = all_nodes
        .iter()
        .enumerate()
        .map(|(i, node)| (node.clone(), i))
        .collect();

    let group_indices: HashSet<usize> = group_set
        .iter()
        .map(|node| *node_to_index.get(node).unwrap())
        .collect();

    let mut sigma = vec![vec![0.0; n]; n];
    let mut distances = vec![vec![f64::INFINITY; n]; n];
    let mut delta: HashMap<usize, Vec<f64>> = HashMap::new();
    let mut betweenness = vec![0.0; n];

    // Initialize diagonal
    for i in 0..n {
        distances[i][i] = 0.0;
        sigma[i][i] = 1.0;
    }

    // For each source node, compute shortest paths and accumulate betweenness
    // Use parallel processing for larger graphs while maintaining deterministic output
    let use_parallel = n > 50 && rayon::current_num_threads() > 1;

    if use_parallel {
        // Parallel processing for large graphs with deterministic result ordering
        let mut results: Vec<_> = (0..n)
            .into_par_iter()
            .map(|s| {
                let (s_stack, p, s_sigma, d) = single_source_shortest_path_basic(
                    graph,
                    &all_nodes[s],
                    all_nodes,
                    &node_to_index,
                )
                .unwrap_or_else(|_| (Vec::new(), Vec::new(), Vec::new(), Vec::new()));

                let (node_bet, node_delta) = accumulate_endpoints(&s_stack, &p, &s_sigma, s);
                (s, s_stack, s_sigma, d, node_bet, node_delta)
            })
            .collect();

        // Sort results by source index to ensure deterministic order
        results.sort_by_key(|(s, _, _, _, _, _)| *s);

        // Apply results sequentially to maintain deterministic output
        for (s, _s_stack, s_sigma, d, node_bet, node_delta) in results {
            // Update global matrices
            for i in 0..n {
                if i < s_sigma.len() {
                    sigma[s][i] = s_sigma[i];
                }
                if i < d.len() && d[i] >= 0.0 {
                    distances[s][i] = d[i];
                }
            }

            for i in 0..n {
                if i < node_bet.len() {
                    betweenness[i] += node_bet[i];
                }
            }

            // Store delta for this source
            delta.insert(s, node_delta);

            // Add path endpoints as per NetworkX
            let delta_len = delta.get(&s).unwrap().len();
            for i in 0..delta_len {
                if s != i {
                    delta.get_mut(&s).unwrap()[i] += 1.0;
                }
            }
        }
    } else {
        // Sequential processing for smaller graphs (preserves exact behavior)
        for s in 0..n {
            let (s_stack, p, s_sigma, d) =
                single_source_shortest_path_basic(graph, &all_nodes[s], all_nodes, &node_to_index)?;

            // Update global matrices
            for i in 0..n {
                if i < s_sigma.len() {
                    sigma[s][i] = s_sigma[i];
                }
                if i < d.len() && d[i] >= 0.0 {
                    distances[s][i] = d[i];
                }
            }

            // Accumulate endpoints - this follows NetworkX's _accumulate_endpoints exactly
            let (node_bet, node_delta) = accumulate_endpoints(&s_stack, &p, &s_sigma, s);

            for i in 0..n {
                if i < node_bet.len() {
                    betweenness[i] += node_bet[i];
                }
            }

            // Store delta for this source
            delta.insert(s, node_delta);

            // Add path endpoints as per NetworkX
            let delta_len = delta.get(&s).unwrap().len();
            for i in 0..delta_len {
                if s != i {
                    delta.get_mut(&s).unwrap()[i] += 1.0;
                }
            }
        }
    }

    // Build path betweenness matrix specifically for group nodes
    let mut pb = vec![vec![0.0; n]; n];

    // Use parallel processing for group pair calculations when beneficial
    let group_indices_vec: Vec<usize> = group_indices.iter().cloned().collect();
    let use_parallel_groups =
        group_indices_vec.len() > 5 && n > 30 && rayon::current_num_threads() > 1;

    if use_parallel_groups {
        // Create all group pairs for parallel processing
        let group_pairs: Vec<(usize, usize)> = group_indices_vec
            .iter()
            .flat_map(|&i| group_indices_vec.iter().map(move |&j| (i, j)))
            .collect();

        // Parallel processing for larger groups with deterministic ordering
        let mut pb_updates: Vec<(usize, usize, f64)> = group_pairs
            .par_iter()
            .filter_map(|&(group_node1, group_node2)| {
                if distances[group_node1][group_node2] == f64::INFINITY {
                    return None;
                }

                let mut pb_value = 0.0;
                for node in 0..n {
                    if let Some(node_delta) = delta.get(&node) {
                        if group_node2 < node_delta.len()
                            && distances[node][group_node2] != f64::INFINITY
                            && distances[group_node1][node] != f64::INFINITY
                            && sigma[node][group_node2] > 0.0
                        {
                            if (distances[node][group_node1] + distances[group_node1][group_node2]
                                - distances[node][group_node2])
                                .abs()
                                < 1e-10
                            {
                                pb_value += node_delta[group_node2]
                                    * sigma[node][group_node1]
                                    * sigma[group_node1][group_node2]
                                    / sigma[node][group_node2];
                            }
                        }
                    }
                }
                Some((group_node1, group_node2, pb_value))
            })
            .collect();

        // Sort updates by indices to ensure deterministic application
        pb_updates.sort_by_key(|(i, j, _)| (*i, *j));

        // Apply updates sequentially to maintain determinism
        for (i, j, value) in pb_updates {
            pb[i][j] = value;
        }
    } else {
        // Sequential processing for smaller groups (preserves exact behavior)
        for &group_node1 in &group_indices {
            for &group_node2 in &group_indices {
                if distances[group_node1][group_node2] == f64::INFINITY {
                    continue;
                }

                for node in 0..n {
                    if let Some(node_delta) = delta.get(&node) {
                        if group_node2 < node_delta.len()
                            && distances[node][group_node2] != f64::INFINITY
                            && distances[group_node1][node] != f64::INFINITY
                            && sigma[node][group_node2] > 0.0
                        {
                            if (distances[node][group_node1] + distances[group_node1][group_node2]
                                - distances[node][group_node2])
                                .abs()
                                < 1e-10
                            {
                                pb[group_node1][group_node2] += node_delta[group_node2]
                                    * sigma[node][group_node1]
                                    * sigma[group_node1][group_node2]
                                    / sigma[node][group_node2];
                            }
                        }
                    }
                }
            }
        }
    }

    Ok((pb, sigma, distances))
}

/// Single source shortest path using BFS (matches NetworkX)
fn single_source_shortest_path_basic<T, A>(
    graph: &Graph<T, A>,
    source: &T,
    all_nodes: &[T],
    node_to_index: &HashMap<T, usize>,
) -> Result<(Vec<usize>, Vec<Vec<usize>>, Vec<f64>, Vec<f64>), Error>
where
    T: Hash + Eq + Clone + Ord + Debug + Display + Send + Sync,
    A: Clone + Send + Sync,
{
    let n = all_nodes.len();
    let s = *node_to_index.get(source).unwrap();

    let mut s_stack = Vec::new();
    let mut p = vec![Vec::new(); n];
    let mut sigma = vec![0.0; n];
    let mut d = vec![-1.0; n];

    sigma[s] = 1.0;
    d[s] = 0.0;

    let mut queue = VecDeque::new();
    queue.push_back(s);

    while let Some(v) = queue.pop_front() {
        s_stack.push(v);

        if let Ok(neighbors) = graph.get_neighbor_nodes(all_nodes[v].clone()) {
            for neighbor in neighbors {
                if let Some(&w) = node_to_index.get(&neighbor.name) {
                    // First time we see w?
                    if d[w] < 0.0 {
                        queue.push_back(w);
                        d[w] = d[v] + 1.0;
                    }

                    // Shortest path to w via v?
                    if (d[w] - d[v] - 1.0_f64).abs() < 1e-10_f64 {
                        sigma[w] += sigma[v];
                        p[w].push(v);
                    }
                }
            }
        }
    }

    Ok((s_stack, p, sigma, d))
}

/// Accumulate endpoints (matches NetworkX's _accumulate_endpoints)
fn accumulate_endpoints(
    s: &[usize],
    p: &[Vec<usize>],
    sigma: &[f64],
    s_idx: usize,
) -> (Vec<f64>, Vec<f64>) {
    let n = sigma.len();
    let mut betweenness = vec![0.0; n];
    let mut delta = vec![0.0; n];

    // Process vertices in reverse topological order
    for &w in s.iter().rev() {
        for &v in &p[w] {
            if v < delta.len()
                && w < delta.len()
                && v < sigma.len()
                && w < sigma.len()
                && sigma[w] > 0.0
            {
                delta[v] += (sigma[v] / sigma[w]) * (1.0 + delta[w]);
            }
        }
        if w != s_idx && w < betweenness.len() && w < delta.len() {
            betweenness[w] += delta[w];
        }
    }

    (betweenness, delta)
}

/// Check if graph is connected (simplified)
fn is_connected(distances: &[Vec<f64>], n: usize) -> bool {
    for i in 0..n {
        for j in 0..n {
            if i < distances.len() && j < distances[i].len() && distances[i][j] == f64::INFINITY {
                return false;
            }
        }
    }
    true
}

/// Check if directed graph is strongly connected (simplified)
fn is_strongly_connected(distances: &[Vec<f64>], n: usize) -> bool {
    is_connected(distances, n)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Edge, Graph, GraphSpecs};

    #[test]
    fn test_group_betweenness_basic() {
        let mut graph = Graph::<i32, ()>::new(GraphSpecs::undirected_create_missing());

        // Create a simple path: 0-1-2-3-4
        graph.add_edge(Edge::new(0, 1)).unwrap();
        graph.add_edge(Edge::new(1, 2)).unwrap();
        graph.add_edge(Edge::new(2, 3)).unwrap();
        graph.add_edge(Edge::new(3, 4)).unwrap();

        let mut group = HashSet::new();
        group.insert(2);

        let centrality = group_betweenness_centrality(&graph, &group, true, false).unwrap();
        assert!(centrality > 0.0);
    }

    #[test]
    fn test_invalid_group_node() {
        let mut graph = Graph::<i32, ()>::new(GraphSpecs::undirected_create_missing());
        graph.add_edge(Edge::new(0, 1)).unwrap();

        let mut group = HashSet::new();
        group.insert(99);

        let result = group_betweenness_centrality(&graph, &group, true, false);
        assert!(result.is_err());
    }

    #[test]
    fn test_empty_group() {
        let mut graph = Graph::<i32, ()>::new(GraphSpecs::undirected_create_missing());
        graph.add_edge(Edge::new(0, 1)).unwrap();

        let group = HashSet::new();

        let result = group_betweenness_centrality(&graph, &group, true, false);
        assert!(result.is_err());
    }

    #[test]
    fn test_group_too_large() {
        let mut graph = Graph::<i32, ()>::new(GraphSpecs::undirected_create_missing());
        graph.add_edge(Edge::new(0, 1)).unwrap();
        graph.add_edge(Edge::new(1, 2)).unwrap();

        let mut group = HashSet::new();
        group.insert(0);
        group.insert(1);
        group.insert(2);

        let result = group_betweenness_centrality(&graph, &group, true, false);
        assert!(result.is_err());
    }

    #[test]
    fn test_deterministic_behavior() {
        let graph = crate::generators::social::karate_club_graph();
        let mut group = HashSet::new();
        group.insert(0);
        group.insert(33);

        let result1 = group_betweenness_centrality(&graph, &group, true, false).unwrap();
        let result2 = group_betweenness_centrality(&graph, &group, true, false).unwrap();

        assert!((result1 - result2).abs() < f64::EPSILON);
    }
}
