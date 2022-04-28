use crate::{ext::vec::VecExt, Error, Graph};
use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::hash::Hash;

/**
Returns the components of a graph.
A connected component is a maximal connected subgraph of an undirected graph.

# Arguments:

* `graph`: a [Graph](../../struct.Graph.html) instance that must be undirected.

# Examples:

```
use graphrs::{Edge, Graph, GraphSpecs};
use graphrs::{algorithms::components};
assert!(false);
```
*/
pub fn strongly_connected_components<T, A>(graph: &Graph<T, A>) -> Result<Vec<HashSet<T>>, Error>
where
    T: Hash + Eq + Clone + Ord + Display + Send + Sync,
    A: Clone + Send + Sync,
{
    graph.ensure_directed()?;
    let mut preorder = HashMap::new();
    let mut lowlink = HashMap::new();
    let mut scc_found = HashSet::new();
    let mut scc_queue = vec![];
    let mut i = 0; // preorder counter
    let neighbors = graph.get_successors_map();
    for source in graph.get_all_node_names() {
        if scc_found.contains(source) {
            continue;
        }
        let queue = vec![source];
        while !queue.is_empty() {
            let v = queue.last().unwrap();
            if !preorder.contains_key(v) {
                i = i + 1;
                preorder.insert(v, i);
            }
            let done = true;
            for w in neighbors.get(v).unwrap() {
                if !preorder.contains_key(&w) {
                    queue.push(w);
                    done = false;
                    break;
                }
            }
            if !done {
                continue;
            }
            lowlink.insert(v, preorder.get(v).unwrap());
            for w in neighbors.get(v).unwrap() {}
        }
    }
    Ok(vec![])
}
