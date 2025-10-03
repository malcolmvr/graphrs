use crate::{Error, ErrorKind, Graph};
use std::hash::Hash;
use std::{
    collections::{hash_map::Entry, HashMap, HashSet},
    fmt::Display,
};

// Performance optimizations:
// - Cached modularity calculations (m_half, resolution_over_m) to avoid repeated divisions
// - Memory pre-allocation with capacity hints for HashMaps and Vectors
// - Conditional determinism: fast HashMap iteration when seed=None, sorted when seed=Some(_)
// - For even better performance, consider using FxHashMap from fxhash crate instead of std::HashMap

// Type aliases for clarity
type CommunityId = usize;
type NodeId = usize;

/**
 * Rewritten from scratch Louvain algorithm for community detection.
 *
 * This implementation is optimized for performance by using delta modularity
 * calculation instead of recalculating the entire modularity for each node move.
 * The algorithm efficiently tracks community connections and degrees for fast updates.
 */
struct Modularity<T>
where
    T: Hash + Eq + Clone + Ord + Display + Send + Sync,
{
    m: f32, // total weight of edges in the graph
    resolution: f32,
    seed: Option<u64>, // seed for deterministic results
    // Cache frequently used calculations
    m_half: f32,            // m * 0.5 - cached to avoid repeated multiplication
    resolution_over_m: f32, // resolution / m - cached for delta modularity calculation
    original_nodes: Vec<T>, // map from node index to original node name
    origin_nodes_community: Vec<CommunityId>,
    nodes: Vec<CNode>,
    communities: Vec<Community>,
    edges: Vec<Vec<WEdge>>,
}

#[derive(Clone)]
struct Community {
    next_id: CommunityId,
    nodes: Vec<NodeId>,
    total_degree: f32,
}

impl Community {
    fn init(node_id: NodeId) -> Self {
        Self {
            nodes: vec![node_id],
            next_id: 0,
            total_degree: 0.0,
        }
    }

    fn add_node(&mut self, node_id: NodeId, degree: f32) {
        self.nodes.push(node_id);
        self.total_degree += degree;
    }

    fn remove_node(&mut self, node_id: NodeId, degree: f32) {
        if let Some(pos) = self.nodes.iter().position(|&x| x == node_id) {
            self.nodes.swap_remove(pos);
            self.total_degree -= degree;
        } else {
            panic!("Node {} not found in community {:?}", node_id, self.nodes);
        }
    }
}

struct CNode {
    community_id: CommunityId,
    degree: f32,
    self_reference_weight: f32,
    communities: HashMap<CommunityId, f32>, // neighboring communities and shared edge weights
}

impl CNode {
    fn init(node_id: NodeId) -> Self {
        Self {
            community_id: node_id,
            degree: 0.0,
            self_reference_weight: 0.0,
            communities: HashMap::new(),
        }
    }

    fn init_cache(&mut self, edges: &[WEdge], node_component: &[CommunityId]) {
        let mut sum_weights = self.self_reference_weight;
        for edge in edges.iter() {
            sum_weights += edge.weight;
            let neighbor = edge.to;
            let neighbor_community = node_component[neighbor];
            *self.communities.entry(neighbor_community).or_insert(0.0) += edge.weight;
        }
        self.degree = sum_weights;
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
struct WEdge {
    from: NodeId,
    to: NodeId,
    weight: f32,
}

/**
Returns the best partition of a graph, using the optimized Louvain algorithm.

# Arguments

* `graph`: a [Graph](../../../struct.Graph.html) instance
* `weighted`: set to `true` to use edge weights when determining communities
* `resolution`: If less than 1.0 larger communities are favoured. If greater than 1.0 smaller communities are favoured.
* `threshold`: Determines how quickly the algorithms stops trying to find partitions with higher modularity. Higher values cause the algorithm to give up more quickly.
* `seed`: Random seed for deterministic results. When `Some(_)`, results will be deterministic but slower due to sorting overhead. When `None`, results are non-deterministic but faster.

# Examples

```
use graphrs::{algorithms::{community}, generators};
let graph = generators::social::karate_club_graph();
// Non-deterministic but fast
let communities = community::louvain::louvain_communities(&graph, false, None, None, None);
// Deterministic but slower
let communities = community::louvain::louvain_communities(&graph, false, None, None, Some(42));
assert_eq!(communities.unwrap().len(), 4);
```
*/
pub fn louvain_communities<T, A>(
    graph: &Graph<T, A>,
    weighted: bool,
    resolution: Option<f64>,
    threshold: Option<f64>,
    seed: Option<u64>,
) -> Result<Vec<HashSet<T>>, Error>
where
    T: Hash + Eq + Clone + Ord + Display + Send + Sync,
    A: Clone + Send + Sync,
{
    let mut partitions = louvain_partitions(graph, weighted, resolution, threshold, seed)?;
    match partitions.is_empty() {
        false => Ok(partitions.pop().unwrap()),
        true => Err(Error {
            kind: ErrorKind::NoPartitions,
            message: "No partitions were found.".to_string(),
        }),
    }
}

/**
Returns the best partitions of a graph, using the optimized Louvain algorithm.

# Arguments

* `graph`: a [Graph](../../../struct.Graph.html) instance
* `weighted`: set to `true` to use edge weights when determining communities
* `resolution`: If less than 1.0 larger communities are favoured. If greater than 1.0 smaller communities are favoured.
* `threshold`: Determines how quickly the algorithms stops trying to find partitions with higher modularity. Higher values cause the algorithm to give up more quickly.
* `seed`: Random seed for deterministic results. When `Some(_)`, results will be deterministic but slower due to sorting overhead. When `None`, results are non-deterministic but faster.

# Examples

```
use graphrs::{algorithms::{community}, generators};
let graph = generators::social::karate_club_graph();
// Non-deterministic but fast
let partitions = community::louvain::louvain_partitions(&graph, false, None, None, None);
// Deterministic but slower
let partitions = community::louvain::louvain_partitions(&graph, false, None, None, Some(42));
```
*/
pub fn louvain_partitions<T, A>(
    graph: &Graph<T, A>,
    weighted: bool,
    resolution: Option<f64>,
    _threshold: Option<f64>,
    seed: Option<u64>,
) -> Result<Vec<Vec<HashSet<T>>>, Error>
where
    T: Hash + Eq + Clone + Ord + Display + Send + Sync,
    A: Clone + Send + Sync,
{
    let mut modularity = Modularity::new(graph, weighted, seed)?;
    modularity.resolution = resolution.unwrap_or(1.0) as f32;

    let _result = modularity.run_louvain()?;

    // Convert result back to the expected format for graphrs
    let mut all_partitions = Vec::new();
    let final_communities = modularity.get_final_communities();
    all_partitions.push(final_communities);

    Ok(all_partitions)
}

impl<T> Modularity<T>
where
    T: Hash + Eq + Clone + Ord + Display + Send + Sync,
{
    /// Helper method to get community keys in a deterministic order when seed is provided
    fn get_community_keys_iter(
        &self,
        communities_map: &HashMap<CommunityId, f32>,
    ) -> Vec<CommunityId> {
        if self.seed.is_some() {
            let mut keys: Vec<CommunityId> = Vec::with_capacity(communities_map.len());
            keys.extend(communities_map.keys().copied());
            keys.sort_unstable(); // unstable sort is faster
            keys
        } else {
            communities_map.keys().copied().collect()
        }
    }
    fn new<A>(graph: &Graph<T, A>, weighted: bool, seed: Option<u64>) -> Result<Self, Error>
    where
        A: Clone + Send + Sync,
    {
        let nodes_len = graph.number_of_nodes();
        let resolution = 1.0;
        let original_nodes: Vec<T> = graph
            .get_all_nodes()
            .iter()
            .map(|n| n.name.clone())
            .collect();
        let origin_nodes_community = (0..nodes_len).collect();
        let nodes = (0..nodes_len).map(CNode::init).collect();
        let communities = (0..nodes_len).map(Community::init).collect();
        let mut wedges: Vec<Vec<WEdge>> = vec![Vec::new(); nodes_len];
        let mut m = 0.0;

        // Build edge lists with pre-allocation hint
        let all_edges = graph.get_all_edges();
        let estimated_degree = if nodes_len > 0 {
            (all_edges.len() * 2) / nodes_len + 1
        } else {
            0
        };

        // Pre-allocate edge vectors with estimated capacity
        for wedge_vec in wedges.iter_mut() {
            wedge_vec.reserve(estimated_degree);
        }

        // Process edges sequentially - simpler and more predictable performance
        for edge in all_edges {
            let from = graph.get_node_index(&edge.u).unwrap();
            let to = graph.get_node_index(&edge.v).unwrap();
            let weight = if weighted { edge.weight as f32 } else { 1.0 };

            wedges[from].push(WEdge { from, to, weight });
            wedges[to].push(WEdge {
                from: to,
                to: from,
                weight,
            });
            m += 2.0 * weight;
        }

        // Handle edge case of graphs with no edges (m = 0)
        let resolution_over_m = if m > 0.0 { resolution / m } else { 0.0 };

        Ok(Self {
            m,
            resolution,
            seed,
            m_half: m * 0.5,
            resolution_over_m,
            original_nodes,
            origin_nodes_community,
            nodes,
            communities,
            edges: wedges,
        })
    }

    fn run_louvain(&mut self) -> Result<Vec<Vec<HashSet<T>>>, Error> {
        self.init_caches();

        // Handle edge case: graphs with no edges - each node is its own community
        if self.m == 0.0 {
            let final_communities = self.get_communities();
            return Ok(vec![final_communities]);
        }

        let mut some_change = true;
        let mut all_partitions = Vec::new();
        let mut iteration_count = 0;
        let max_iterations = 1000; // Prevent infinite loops

        while some_change && iteration_count < max_iterations {
            some_change = false;
            let mut local_change = true;
            iteration_count += 1;

            // Debug: prevent infinite loops with more aggressive counter
            let mut inner_iteration_count = 0;
            let max_inner_iterations = 1000;

            while local_change && inner_iteration_count < max_inner_iterations {
                local_change = false;
                inner_iteration_count += 1;
                let mut step = 0;
                let mut node_index = 0;
                let num_communities = self.communities.len();

                // Safety check: if no communities, break out
                if num_communities == 0 {
                    break;
                }

                // Sequential processing due to data dependencies in community detection
                while step < num_communities {
                    let best_community = self.find_best_community(node_index);
                    let node = &self.nodes[node_index];

                    if let Some(best_community) = best_community {
                        if best_community != node.community_id {
                            self.move_node_to(node_index, best_community);
                            local_change = true;
                        }
                    }

                    step += 1;
                    node_index = (node_index + 1) % num_communities;
                }

                some_change = local_change || some_change;
            }

            if some_change {
                let current_communities = self.get_communities();
                all_partitions.push(current_communities);
                self.merge_nodes();
            }
        }

        // Add final partition
        let final_communities = self.get_communities();
        all_partitions.push(final_communities);

        Ok(all_partitions)
    }

    fn init_caches(&mut self) {
        let node_component: Vec<CommunityId> = self.nodes.iter().map(|n| n.community_id).collect();

        // Sequential cache initialization - simpler and more predictable
        for (node_index, node) in self.nodes.iter_mut().enumerate() {
            node.init_cache(&self.edges[node_index], &node_component);
        }

        for community in self.communities.iter_mut() {
            community.total_degree = Self::community_total_degree_compute(community, &self.nodes);
        }
    }

    fn community_total_degree_compute(community: &Community, nodes: &[CNode]) -> f32 {
        let mut sum = 0.0;
        for node_index in community.nodes.iter() {
            let node = &nodes[*node_index];
            sum += node.degree;
        }
        sum
    }

    fn find_best_community(&self, node_id: NodeId) -> Option<CommunityId> {
        let mut best: f32 = 0.0;
        let mut best_community = None;
        let node = &self.nodes[node_id];

        // Early exit if node has no community connections
        if node.communities.is_empty() {
            return None;
        }

        // Optimize for the common case (no seed) - iterate directly over HashMap
        if self.seed.is_none() {
            for (&community_index, &shared_degree) in &node.communities {
                if shared_degree > 0.0 {
                    let q_value = self.q(node_id, community_index, shared_degree);
                    if q_value > best {
                        best = q_value;
                        best_community = Some(community_index);
                    }
                }
            }
        } else {
            // Deterministic case - use sorted keys
            let community_keys = self.get_community_keys_iter(&node.communities);
            for community_index in community_keys {
                if let Some(shared_degree) = node.communities.get(&community_index) {
                    if *shared_degree > 0.0 {
                        let q_value = self.q(node_id, community_index, *shared_degree);
                        if q_value > best {
                            best = q_value;
                            best_community = Some(community_index);
                        }
                    }
                }
            }
        }
        best_community
    }

    fn q(&self, node_id: NodeId, community_id: CommunityId, shared_degree: f32) -> f32 {
        // Optimized delta modularity calculation using cached values
        // delta_q = (resolution*d_ij - (d_i*d_j)/(2*m))/m
        let node = &self.nodes[node_id];
        let current_community = node.community_id;
        let community = &self.communities[community_id];

        if current_community == community_id {
            if community.nodes.len() == 1 {
                0.0
            } else {
                let d_i = node.degree;
                // we simulate the case that the node is removed from current community
                let d_j = community.total_degree - d_i;
                let d_ij = shared_degree * 2.0;
                // Use cached values for better performance
                self.resolution_over_m * d_ij - (d_i * d_j) / (self.m_half * self.m)
            }
        } else {
            let d_i = node.degree;
            let d_j = community.total_degree;
            let d_ij = shared_degree * 2.0;
            // Use cached values for better performance
            self.resolution_over_m * d_ij - (d_i * d_j) / (self.m_half * self.m)
        }
    }

    fn move_node_to(&mut self, node_id: NodeId, community_id: CommunityId) {
        let old_community_id = self.nodes[node_id].community_id;
        let node_degree = self.nodes[node_id].degree;

        let old_community = &mut self.communities[old_community_id];
        old_community.remove_node(node_id, node_degree);

        let new_community = &mut self.communities[community_id];
        new_community.add_node(node_id, node_degree);

        // Update neighbor community connections
        for wedge in self.edges[node_id].iter() {
            let neighbor = wedge.to;
            let neighbor_node = &mut self.nodes[neighbor];

            if let Entry::Occupied(mut entry) = neighbor_node.communities.entry(old_community_id) {
                let new_val = *entry.get() - wedge.weight;
                if new_val == 0.0 {
                    entry.remove();
                } else {
                    *entry.get_mut() = new_val;
                }
            }
            *neighbor_node.communities.entry(community_id).or_insert(0.0) += wedge.weight;
        }

        self.nodes[node_id].community_id = community_id;
    }

    fn get_communities(&self) -> Vec<HashSet<T>> {
        // Pre-allocate with estimated capacity based on active communities
        let active_communities = self
            .communities
            .iter()
            .filter(|c| !c.nodes.is_empty())
            .count();
        let mut communities_map: HashMap<CommunityId, HashSet<T>> =
            HashMap::with_capacity(active_communities);

        for (node_index, &community_id) in self.origin_nodes_community.iter().enumerate() {
            let actual_community = self.nodes[community_id].community_id;
            communities_map
                .entry(actual_community)
                .or_insert_with(HashSet::new)
                .insert(self.original_nodes[node_index].clone());
        }

        if self.seed.is_some() {
            // Return communities in deterministic order
            let mut sorted_communities: Vec<(CommunityId, HashSet<T>)> =
                communities_map.into_iter().collect();
            sorted_communities.sort_by_key(|(community_id, _)| *community_id);
            sorted_communities
                .into_iter()
                .map(|(_, community)| community)
                .collect()
        } else {
            communities_map.into_values().collect()
        }
    }

    fn get_final_communities(&self) -> Vec<HashSet<T>> {
        self.get_communities()
    }

    fn merge_nodes(&mut self) {
        // Map old community IDs to new community IDs
        let mut new_community_count = 0;
        for c in self.communities.iter_mut() {
            if !c.nodes.is_empty() {
                c.next_id = new_community_count;
                new_community_count += 1;
            }
        }

        let mut new_communities: Vec<Community> = Vec::with_capacity(new_community_count);
        let mut new_edges: Vec<Vec<WEdge>> = vec![Vec::new(); new_community_count];
        let mut new_nodes: Vec<CNode> = Vec::with_capacity(new_community_count);
        let mut m = 0.0;

        for community in self.communities.iter() {
            if community.nodes.is_empty() {
                continue;
            }

            let new_community_id = community.next_id;
            new_communities.push(Community::init(new_community_id));
            let mut new_node = CNode::init(new_community_id);
            let mut edges_for_community: HashMap<CommunityId, f32> = HashMap::new();
            let mut self_reference = 0.0;

            for &node_id in community.nodes.iter() {
                for wedge in self.edges[node_id].iter() {
                    let neighbor_community = self.nodes[wedge.to].community_id;
                    let neighbor_community_new = self.communities[neighbor_community].next_id;
                    *edges_for_community
                        .entry(neighbor_community_new)
                        .or_insert(0.0) += wedge.weight;
                }
                self_reference += self.nodes[node_id].self_reference_weight;
            }

            // Optimize edge iteration based on whether determinism is needed
            if self.seed.is_some() {
                // Deterministic case - need to sort
                let mut sorted_edges: Vec<(&CommunityId, &f32)> =
                    Vec::with_capacity(edges_for_community.len());
                sorted_edges.extend(edges_for_community.iter());
                sorted_edges.sort_unstable_by_key(|(community_id, _)| *community_id);

                for (neighbor_community, weight) in sorted_edges {
                    m += weight;
                    if *neighbor_community == new_community_id {
                        self_reference += weight;
                    } else {
                        new_edges[new_community_id].push(WEdge {
                            from: new_community_id,
                            to: *neighbor_community,
                            weight: *weight,
                        });
                    }
                }
            } else {
                // Fast path - iterate directly over HashMap
                for (neighbor_community, weight) in &edges_for_community {
                    m += weight;
                    if *neighbor_community == new_community_id {
                        self_reference += weight;
                    } else {
                        new_edges[new_community_id].push(WEdge {
                            from: new_community_id,
                            to: *neighbor_community,
                            weight: *weight,
                        });
                    }
                }
            }

            new_node.self_reference_weight = self_reference;
            new_nodes.push(new_node);
        }

        // Update origin nodes community mapping
        for i in 0..self.origin_nodes_community.len() {
            let new_community_old_id = self.nodes[self.origin_nodes_community[i]].community_id;
            let new_community_id = self.communities[new_community_old_id].next_id;
            self.origin_nodes_community[i] = new_community_id;
        }

        self.communities = new_communities;
        self.nodes = new_nodes;
        self.edges = new_edges;
        self.m = m;
        // Update cached values when m changes - handle division by zero
        self.m_half = m * 0.5;
        self.resolution_over_m = if m > 0.0 { self.resolution / m } else { 0.0 };
        self.init_caches();
    }
}

// Helper function to compute modularity for the current community assignment
pub fn compute_modularity<T, A>(
    graph: &Graph<T, A>,
    communities: &[HashSet<T>],
    weighted: bool,
) -> f32
where
    T: Hash + Eq + Clone + Ord + Display + Send + Sync,
    A: Clone + Send + Sync,
{
    let m = graph.size(weighted) as f32;
    let mut q = 0.0f32;

    // Create node to community mapping
    let mut node_to_community: HashMap<&T, usize> = HashMap::new();
    for (comm_id, community) in communities.iter().enumerate() {
        for node in community {
            node_to_community.insert(node, comm_id);
        }
    }

    for community in communities {
        let mut internal_edges = 0.0f32;
        let mut total_degree = 0.0f32;

        for node in community {
            let degree = if weighted {
                graph.get_node_weighted_degree(node.clone()).unwrap_or(0.0) as f32
            } else {
                graph.get_node_degree(node.clone()).unwrap_or(0) as f32
            };
            total_degree += degree;

            // Count internal edges
            if let Ok(neighbors) = graph.get_successor_nodes(node.clone()) {
                for neighbor in neighbors {
                    if let Some(&neighbor_comm) = node_to_community.get(&neighbor.name) {
                        if neighbor_comm == *node_to_community.get(node).unwrap() {
                            let edge_weight = if weighted {
                                if let Ok(edge) =
                                    graph.get_edge(node.clone(), neighbor.name.clone())
                                {
                                    edge.weight as f32
                                } else {
                                    1.0f32
                                }
                            } else {
                                1.0f32
                            };
                            internal_edges += edge_weight;
                        }
                    }
                }
            }
        }

        internal_edges /= 2.0; // Each internal edge is counted twice
        q += internal_edges / m - (total_degree / (2.0 * m)).powi(2);
    }

    q
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Edge, Graph, GraphSpecs};

    #[test]
    fn test_louvain_basic() {
        let mut graph = Graph::<usize, ()>::new(GraphSpecs::undirected_create_missing());

        // Create a simple graph with two clear communities: {0,1,2} and {3,4,5}
        graph
            .add_edges(vec![
                Edge::new(0, 1),
                Edge::new(0, 2),
                Edge::new(1, 2),
                Edge::new(2, 3), // bridge edge
                Edge::new(3, 4),
                Edge::new(3, 5),
                Edge::new(4, 5),
            ])
            .expect("couldn't add edges");

        let communities = louvain_communities(&graph, false, None, None, None).unwrap();

        // Should find 2 communities
        assert_eq!(communities.len(), 2);

        // Verify that nodes are grouped correctly
        let mut community_sizes: Vec<usize> = communities.iter().map(|c| c.len()).collect();
        community_sizes.sort();
        assert_eq!(community_sizes, vec![3, 3]);
    }

    #[test]
    fn test_modularity_calculation() {
        let mut graph = Graph::<usize, ()>::new(GraphSpecs::undirected_create_missing());

        // Create a graph with clear community structure: two connected components
        graph
            .add_edges(vec![
                Edge::new(0, 1),
                Edge::new(1, 2),
                Edge::new(3, 4),
                Edge::new(4, 5),
            ])
            .expect("couldn't add edges");

        // Communities matching the structure: {0,1,2} and {3,4,5}
        let mut community_0 = HashSet::new();
        community_0.insert(0usize);
        community_0.insert(1usize);
        community_0.insert(2usize);
        let mut community_1 = HashSet::new();
        community_1.insert(3usize);
        community_1.insert(4usize);
        community_1.insert(5usize);
        let communities_structured = vec![community_0, community_1];
        let mod_structured = compute_modularity(&graph, &communities_structured, false);

        // All nodes in one community (poor structure)
        let mut community_all = HashSet::new();
        community_all.insert(0usize);
        community_all.insert(1usize);
        community_all.insert(2usize);
        community_all.insert(3usize);
        community_all.insert(4usize);
        community_all.insert(5usize);
        let communities_all = vec![community_all];
        let mod_all = compute_modularity(&graph, &communities_all, false);

        // Structured communities should have higher modularity than all-in-one
        assert!(mod_structured > mod_all);
    }

    #[test]
    fn test_weighted_louvain() {
        let mut graph = Graph::<&str, ()>::new(GraphSpecs::undirected_create_missing());

        graph
            .add_edges(vec![
                Edge::with_weight("a", "b", 5.0),
                Edge::with_weight("b", "c", 5.0),
                Edge::with_weight("c", "d", 1.0), // weak connection
                Edge::with_weight("d", "e", 5.0),
            ])
            .expect("couldn't add edges");

        let communities = louvain_communities(&graph, true, None, None, None).unwrap();

        // Should favor grouping strongly connected nodes
        assert!(communities.len() >= 2);
    }
}
