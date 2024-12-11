use crate::{ext::vec::VecExt, Error, Graph};
use std::collections::HashSet;
use std::fmt::Display;
use std::hash::Hash;

/**
Returns the weakly connected components of a directed graph.

# Arguments:

* `graph`: a [Graph](../../struct.Graph.html) instance that must be directed.

# Examples:

```
use graphrs::{algorithms::{components}, generators};
let graph = generators::random::fast_gnp_random_graph(250, 0.005, true, Some(1)).unwrap();
let weak_components = components::weakly_connected_components(&graph).unwrap();
assert_eq!(weak_components.len(), 34);
```
*/
pub fn weakly_connected_components<T, A>(graph: &Graph<T, A>) -> Result<Vec<HashSet<T>>, Error>
where
    T: Hash + Eq + Clone + Ord + Display + Send + Sync,
    A: Clone + Send + Sync,
{
    graph.ensure_directed()?;
    let mut seen = HashSet::new();
    let mut components = vec![];
    for v in graph.get_all_node_names() {
        if !seen.contains(v) {
            let bfs = plain_bfs(graph, v).to_hashset();
            seen = seen.union(&bfs).cloned().collect();
            components.push(bfs);
        }
    }
    Ok(components)
}

fn plain_bfs<T, A>(graph: &Graph<T, A>, source: &T) -> Vec<T>
where
    T: Hash + Eq + Clone + Ord + Display + Send + Sync,
    A: Clone + Send + Sync,
{
    let mut connected_nodes: Vec<T> = vec![];
    let gsucc = graph.get_successors_map();
    let gpred = graph.get_predecessors_map();
    let mut seen = HashSet::new();
    let mut nextlevel = HashSet::new();
    let empty_hs: HashSet<T> = HashSet::new();
    nextlevel.insert(source.clone());
    while nextlevel.len() > 0 {
        let thislevel = nextlevel;
        nextlevel = HashSet::new();
        for v in thislevel {
            if !seen.contains(&v) {
                seen.insert(v.clone());
                connected_nodes.push(v.clone());
                nextlevel = nextlevel
                    .union(gsucc.get(&v).unwrap_or(&empty_hs))
                    .cloned()
                    .collect();
                nextlevel = nextlevel
                    .union(gpred.get(&v).unwrap_or(&empty_hs))
                    .cloned()
                    .collect();
                connected_nodes.push(v.clone());
            }
        }
    }
    connected_nodes
}

pub fn bfs_equal_size_partitions<T, A>(graph: &Graph<T, A>, num_partitions: usize) -> Vec<Vec<T>>
where
    T: Hash + Eq + Clone + Ord + Display + Send + Sync,
    A: Clone + Send + Sync,
{
    let number_of_nodes = graph.number_of_nodes();
    let mut partitions = vec![vec![]; num_partitions];
    let mut visited = vec![false; graph.number_of_nodes()];
    let mut visited_count = 0;
    let mut queue = Vec::new();
    let mut partition = 0;
    let partition_max_size = (graph.number_of_nodes() / num_partitions) + 1;
    while visited_count < number_of_nodes {
        let node = (0..graph.number_of_nodes())
            .find(|node| !visited[*node])
            .unwrap();
        queue.push(node);
        while !queue.is_empty() {
            let current = queue.remove(0);
            if !visited[current] {
                visited[current] = true;
                partitions[partition].push(current);
                visited_count += 1;
                if partitions[partition].len() == partition_max_size {
                    break;
                }
                for neighbor in graph.get_successor_nodes_by_index(&current) {
                    queue.push(neighbor.node_index);
                }
            }
        }
        if partitions[partition].len() == partition_max_size {
            queue = Vec::new();
            partition += 1;
        }
    }
    partitions
        .iter()
        .map(|partition| {
            partition
                .iter()
                .map(|node| graph.get_node_by_index(node).unwrap().name.clone())
                .collect()
        })
        .collect()
}
