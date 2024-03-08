use crate::{ext::vec::VecExt, Error, Graph};
use std::collections::{HashSet};
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
assert_eq!(weak_components.len(), 23);
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
                nextlevel = nextlevel.union(gsucc.get(&v).unwrap_or(&empty_hs)).cloned().collect();
                nextlevel = nextlevel.union(gpred.get(&v).unwrap_or(&empty_hs)).cloned().collect();
                connected_nodes.push(v.clone());
            }
        }
    }
    connected_nodes
}