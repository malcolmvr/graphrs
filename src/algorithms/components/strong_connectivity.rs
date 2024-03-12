use crate::{ext::vec::VecExt, Error, Graph};
use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::hash::Hash;

/**
Returns the strongly connected components of a directed graph.

# Arguments:

* `graph`: a [Graph](../../struct.Graph.html) instance that must be directed.

# Examples:

```
use graphrs::{algorithms::{components}, generators};
let graph = generators::random::fast_gnp_random_graph(250, 0.02, true, Some(1)).unwrap();
let strong_components = components::strongly_connected_components(&graph).unwrap();
assert_eq!(strong_components.len(), 5);
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
    let mut scc_found: HashSet<T> = HashSet::new();
    let mut scc_queue: Vec<&T> = vec![];
    let mut i = 0; // preorder counter
    let neighbors: HashMap<&T, HashSet<T>> = graph
        .get_successors_map()
        .into_iter()
        .map(|(n, hs)| (n, hs.clone()))
        .collect();
    let mut components: Vec<HashSet<T>> = Vec::new();
    let empty_hs: HashSet<T> = HashSet::new();
    for source in graph.get_all_node_names() {
        if scc_found.contains(&source) {
            continue;
        }
        let mut queue = vec![source];
        while !queue.is_empty() {
            // let last = queue.last();
            let v = *queue.last().unwrap();
            if !preorder.contains_key(&v) {
                i = i + 1;
                preorder.insert(v, i);
            }
            let mut done = true;
            for w in neighbors.get(v).unwrap_or(&empty_hs) {
                if !preorder.contains_key(&w) {
                    queue.push(w);
                    done = false;
                    break;
                }
            }
            // neighbors.remove(v);
            // neighbors.insert(v, HashSet::new());
            if !done {
                continue;
            }
            lowlink.insert(v, preorder.get(v).unwrap().clone());
            for w in neighbors.get(v).unwrap_or(&empty_hs) {
                if !scc_found.contains(&w) {
                    let new_ll = match preorder.get(&w).unwrap() > preorder.get(v).unwrap() {
                        true => vec![lowlink.get(v).unwrap(), lowlink.get(&w).unwrap()]
                            .into_iter()
                            .min()
                            .unwrap(),
                        false => vec![lowlink.get(v).unwrap(), preorder.get(&w).unwrap()]
                            .into_iter()
                            .min()
                            .unwrap(),
                    }
                    .clone();
                    lowlink.insert(v, new_ll);
                }
            }
            queue.pop();
            if lowlink.get(v).unwrap() == preorder.get(v).unwrap() {
                let mut scc: HashSet<T> = vec![v.clone()].to_hashset();
                while !scc_queue.is_empty()
                    && !scc_queue.is_empty()
                    && preorder.get(*scc_queue.last().unwrap()) > preorder.get(v)
                {
                    let k = scc_queue.pop().unwrap();
                    scc.insert(k.clone());
                }
                scc_found = scc_found.union(&scc).cloned().collect();
                components.push(scc)
            } else {
                scc_queue.push(v);
            }
        }
    }
    Ok(components)
}
