use crate::algorithms::shortest_path::ShortestPathInfo;
use crate::{Error, ErrorKind, Graph, Node};
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::fmt::Display;
use std::hash::Hash;
use std::mem;
use std::os::unix::fs::lchown;
use std::sync::Arc;

/**
As a graph is explored by a shortest-path algorithm the nodes at the
"fringe" of the explored part are maintained. This struct holds information
about a fringe node.
*/
struct FringeNode {
    pub node_index: usize,
    pub count: i32,
    pub distance: f64,
}

impl Ord for FringeNode {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.distance < other.distance {
            Ordering::Less
        } else if self.distance > other.distance {
            Ordering::Greater
        } else {
            let count_ordering = self.count.cmp(&other.count);
            match count_ordering {
                Ordering::Equal => self.node_index.cmp(&other.node_index),
                _ => count_ordering,
            }
        }
    }
}

impl PartialOrd for FringeNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for FringeNode {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
            && self.count == other.count
            && self.node_index == other.node_index
    }
}

impl Eq for FringeNode {}

static CONTRADICTORY_PATHS_ERROR_MESSAGE: &str =
    "Contradictary paths found, do some edges have negative weights?";

/**
Uses Dijkstra's algorithm to find shortest weighted paths between all pairs
of nodes.

# Arguments

* `graph`: a [Graph](../../../struct.Graph.html) instance where all edges have a weight.
* `weighted`: determines if shortest paths are determined with edge weight, or not
* `cutoff`: Length (sum of edge weights) at which the search is stopped.
If cutoff is provided, only return paths with summed weight <= cutoff.
* `first_only`: If `true` returns the first shortest path found for each source and target,
if `false` returns all shortest paths found between sources and targets.

# Returns

A `HashMap` of `HashMaps`. The keys to the first one are the starting nodes
and the keys to the second are the target nodes. The values of the second one are `Vec`s
of the shortest paths between the starting and target nodes.

# Examples

```
use graphrs::{Edge, Graph, GraphSpecs, Node};
use graphrs::{algorithms::{shortest_path::{dijkstra}}};

let mut graph = Graph::<&str, ()>::new(GraphSpecs::directed_create_missing());
graph.add_edges(vec![
    Edge::with_weight("n1", "n2", 1.0),
    Edge::with_weight("n2", "n1", 2.0),
    Edge::with_weight("n1", "n3", 3.0),
    Edge::with_weight("n2", "n3", 1.1),
]);

let all_pairs = dijkstra::all_pairs(&graph, true, None, None, false, true);
assert_eq!(all_pairs.unwrap().get("n1").unwrap().get("n3").unwrap().distance, 2.1);
```

# References

1. E. W. Dijkstra. A note on two problems in connection with graphs. Numer. Math., 1:269–271, 1959.

*/
pub fn all_pairs<T, A>(
    graph: &Graph<T, A>,
    weighted: bool,
    target: Option<T>,
    cutoff: Option<f64>,
    first_only: bool,
    with_paths: bool,
) -> Result<HashMap<T, HashMap<T, ShortestPathInfo<T>>>, Error>
where
    T: Hash + Eq + Clone + Ord + Display + Send + Sync,
    A: Clone + Send + Sync,
{
    if weighted {
        graph.ensure_weighted()?;
    }

    let x = graph
        .get_all_nodes()
        .into_iter()
        .map(|node| {
            let ss = single_source(
                graph,
                weighted,
                node.name.clone(),
                target.clone(),
                cutoff,
                first_only,
                with_paths,
            );
            (node.name.clone(), ss)
        })
        .collect::<Vec<(T, Result<HashMap<T, ShortestPathInfo<T>>, Error>)>>();
    let y = x
        .iter()
        .filter(|t| t.1.is_err())
        .collect::<Vec<&(T, Result<HashMap<T, ShortestPathInfo<T>>, Error>)>>();
    if !y.is_empty() {
        match &y[0].1 {
            Err(e) => {
                return Err(e.clone());
            }
            Ok(_) => {}
        }
    }
    Ok(x.into_iter().map(|t| (t.0, t.1.unwrap())).collect())
}

pub(crate) fn all_pairs_iter<'a, T, A>(
    graph: &'a Graph<T, A>,
    weighted: bool,
    target: Option<T>,
    cutoff: Option<f64>,
    first_only: bool,
    with_paths: bool,
) -> impl Iterator<Item = (usize, Vec<(usize, ShortestPathInfo<usize>)>)> + 'a
where
    T: Hash + Eq + Clone + Ord + Display + Send + Sync,
    A: Clone + Send + Sync,
{
    let target_index = match target.clone() {
        Some(t) => Some(graph.get_node_index(&t).unwrap()),
        None => None,
    };
    let x = (0..graph.number_of_nodes())
        .collect::<Vec<_>>()
        .into_iter()
        .map(move |node_index| {
            let ss_index = match can_use_no_options(target.clone(), cutoff, first_only) {
                true => dijkstra_no_options(graph, weighted, node_index),
                false => dijkstra(
                    graph,
                    weighted,
                    node_index,
                    target_index,
                    cutoff,
                    first_only,
                    with_paths,
                ),
            }
            .unwrap();
            (node_index, ss_index)
        });
    x
}

pub(crate) fn all_pairs_par_iter<'a, T, A>(
    graph: &'a Graph<T, A>,
    weighted: bool,
    target: Option<T>,
    cutoff: Option<f64>,
    first_only: bool,
    with_paths: bool,
) -> rayon::iter::Map<
    rayon::vec::IntoIter<usize>,
    impl Fn(usize) -> (usize, Vec<(usize, ShortestPathInfo<usize>)>) + 'a,
>
where
    T: Hash + Eq + Clone + Ord + Display + Send + Sync + 'a,
    A: Clone + Send + Sync + 'a,
{
    let target_index = match target.clone() {
        Some(t) => Some(graph.get_node_index(&t).unwrap()),
        None => None,
    };
    let x = (0..graph.number_of_nodes())
        .collect::<Vec<_>>()
        .into_par_iter()
        .map(move |node_index| {
            let ss_index = match can_use_no_options(target.clone(), cutoff, first_only) {
                true => dijkstra_no_options(graph, weighted, node_index),
                false => dijkstra(
                    graph,
                    weighted,
                    node_index,
                    target_index,
                    cutoff,
                    first_only,
                    with_paths,
                ),
            }
            .unwrap();
            (node_index, ss_index)
        });
    x
}

fn convert_vec_shortest_path_usize_to_t<T, A>(
    graph: &Graph<T, A>,
    vec: Vec<(usize, ShortestPathInfo<usize>)>,
) -> Vec<(T, ShortestPathInfo<T>)>
where
    T: Hash + Eq + Clone + Ord + Display + Send + Sync,
    A: Clone + Send + Sync,
{
    vec.into_iter()
        .map(|(k, v)| {
            (
                graph.get_node_by_index(&k).unwrap().name.clone(),
                convert_shortest_path_info_index_to_t(graph, v),
            )
        })
        .collect()
}
/**
Uses Dijkstra's algorithm to find shortest weighted paths from a single source node.
Unlike most implementations this returns all shortest paths of equal length rather
than just the first one found.

# Arguments

* `graph`: a [Graph](../../../struct.Graph.html) instance where all edges have a weight.
* `weighted`: determines if shortest paths are determined with edge weight, or not
* `source`: The starting node.
* `target`: The ending node. If `None` then the shortest paths between `source` and
all other nodes will be found.
* `cutoff`: Length (sum of edge weights) at which the search is stopped.
If cutoff is provided, only return paths with summed weight <= cutoff.
* `first_only`: If `true` returns the first shortest path found for each target, if
`false` returns all shortest paths found between source and targets.

# Examples

```
use graphrs::{Edge, Graph, GraphSpecs, Node};
use graphrs::{algorithms::{shortest_path::{dijkstra}}};

let mut graph = Graph::<&str, ()>::new(GraphSpecs::directed_create_missing());
graph.add_edges(vec![
    Edge::with_weight("n1", "n2", 1.0),
    Edge::with_weight("n2", "n1", 2.0),
    Edge::with_weight("n1", "n3", 3.0),
    Edge::with_weight("n2", "n3", 1.1),
]);

let shortest_paths = dijkstra::single_source(&graph, true, "n1", Some("n3"), None, false, true);
assert_eq!(shortest_paths.unwrap().get("n3").unwrap().distance, 2.1);
```

# References

1. E. W. Dijkstra. A note on two problems in connection with graphs. Numer. Math., 1:269–271, 1959.
*/
pub fn single_source<T, A>(
    graph: &Graph<T, A>,
    weighted: bool,
    source: T,
    target: Option<T>,
    cutoff: Option<f64>,
    first_only: bool,
    with_paths: bool,
) -> Result<HashMap<T, ShortestPathInfo<T>>, Error>
where
    T: Hash + Eq + Clone + Ord + Display + Send + Sync,
    A: Clone + Send + Sync,
{
    let source_index = graph.get_node_index(&source)?;
    let target_index = match target.clone() {
        Some(t) => Some(graph.get_node_index(&t)?),
        None => None,
    };
    let result = match can_use_no_options(target, cutoff, first_only) {
        true => dijkstra_no_options(graph, weighted, source_index),
        false => dijkstra(
            graph,
            weighted,
            source_index,
            target_index,
            cutoff,
            first_only,
            with_paths,
        ),
    }?;
    Ok(convert_shortest_path_info_index_map_to_t_map(graph, result))
}

/**
Uses Dijkstra's algorithm to find shortest weighted paths from multiple source nodes.
Unlike most implementations this returns all shortest paths of equal length rather
than just the first one found.

# Arguments

* `graph`: a [Graph](../../../struct.Graph.html) instance where all edges have a weight.
* `weighted`: determines if shortest paths are determined with edge weight, or not
* `sources`: The starting nodes. The shortest path will be found that can start
for any of the `sources` and ends at the `target`.
* `target`: The ending node. If `None` then the shortest paths between `sources` and
all other nodes will be found.
* `cutoff`: Length (sum of edge weights) at which the search is stopped.
If cutoff is provided, only return paths with summed weight <= cutoff.
* `first_only`: If `true` returns the first shortest path found for each target, if
`false` returns all shortest paths found between sources and targets.

# Examples

```
use graphrs::{Edge, Graph, GraphSpecs, Node};
use graphrs::{algorithms::{shortest_path::{dijkstra}}};

let mut graph = Graph::<&str, ()>::new(GraphSpecs::directed_create_missing());
graph.add_edges(vec![
    Edge::with_weight("n1", "n2", 1.0),
    Edge::with_weight("n2", "n1", 2.0),
    Edge::with_weight("n1", "n3", 3.0),
    Edge::with_weight("n2", "n3", 1.1),
]);

let shortest_paths = dijkstra::multi_source(&graph, true, vec!["n1", "n2"], Some("n3"), None, false, true);
assert_eq!(shortest_paths.unwrap().get("n2").unwrap().get("n3").unwrap().distance, 1.1);
```

# References

1. E. W. Dijkstra. A note on two problems in connection with graphs. Numer. Math., 1:269–271, 1959.
*/
pub fn multi_source<T, A>(
    graph: &Graph<T, A>,
    weighted: bool,
    sources: Vec<T>,
    target: Option<T>,
    cutoff: Option<f64>,
    first_only: bool,
    with_paths: bool,
) -> Result<HashMap<T, HashMap<T, ShortestPathInfo<T>>>, Error>
where
    T: Hash + Eq + Clone + Ord + Display + Send + Sync,
    A: Clone + Send + Sync,
{
    let target_index = match target.clone() {
        Some(t) => Some(graph.get_node_index(&t)?),
        None => None,
    };
    let shortest_path_infos: Result<HashMap<T, HashMap<T, ShortestPathInfo<T>>>, Error> = sources
        .into_iter()
        .map(|s| {
            let source = graph.get_node_index(&s)?;
            let result = match can_use_no_options(target.clone(), cutoff, first_only) {
                true => dijkstra_no_options(graph, weighted, source),
                false => dijkstra(
                    graph,
                    weighted,
                    source,
                    target_index,
                    cutoff,
                    first_only,
                    with_paths,
                ),
            };
            result.map(|res| {
                let result_t = convert_shortest_path_info_index_map_to_t_map(graph, res);
                (s, result_t)
            })
        })
        .collect();
    match shortest_path_infos {
        Err(e) => Err(e),
        Ok(spis) => Ok(spis),
    }
}

fn dijkstra<T, A>(
    graph: &Graph<T, A>,
    weighted: bool,
    source: usize,
    target: Option<usize>,
    cutoff: Option<f64>,
    first_only: bool,
    with_paths: bool,
) -> Result<Vec<(usize, ShortestPathInfo<usize>)>, Error>
where
    T: Hash + Eq + Clone + Ord + Display + Send + Sync,
    A: Clone,
{
    // println!("source: {:?}", source);
    let mut paths: Vec<Vec<Vec<usize>>> = vec![];
    if with_paths {
        paths = vec![vec![]; graph.number_of_nodes()];
        paths[source] = vec![vec![source]];
    }
    let mut dist = vec![f64::MAX; graph.number_of_nodes()];
    let mut seen = vec![f64::MAX; graph.number_of_nodes()];
    let mut fringe = BinaryHeap::<FringeNode>::new();
    let mut count = 0;

    seen[source] = 0.0;
    fringe.push(FringeNode {
        node_index: source,
        count: 0,
        distance: -0.0,
    });

    while let Some(fringe_item) = fringe.pop() {
        let d = -fringe_item.distance;
        let v = fringe_item.node_index;
        // println!("    v: {}", v);
        if dist[v] != f64::MAX {
            continue;
        }
        dist[v] = d;
        if target.as_ref() == Some(&v) {
            break;
        }
        for adj in graph.get_successor_nodes_by_index(&v) {
            let u = adj.node_index;
            // println!("        u: {}", u);
            let cost = match weighted {
                true => adj.weight,
                false => 1.0,
            };
            // println!("            cost: {}", cost);
            let vu_dist = dist[v] + cost;
            // println!("            vu_dist: {}", vu_dist);
            if cutoff.map_or(false, |c| vu_dist > c) {
                continue;
            }
            if dist[u] != f64::MAX {
                let u_dist = dist[u];
                if vu_dist < u_dist {
                    return Err(get_contractory_paths_error());
                }
            } else if vu_dist < seen[u] {
                // println!("            !seen.contains_key(u)");
                seen[u] = vu_dist;
                push_fringe_node(&mut count, &mut fringe, u, vu_dist);
                if with_paths {
                    let mut new_paths_v = paths[v].clone();
                    new_paths_v.iter_mut().for_each(|pv| pv.push(u));
                    paths[u] = new_paths_v;
                }
            } else if !first_only && vu_dist == seen[u] {
                push_fringe_node(&mut count, &mut fringe, u, vu_dist);
                if with_paths {
                    add_u_to_v_paths_and_append_v_paths_to_u_paths(u, v, &mut paths);
                }
            }
        }
    }

    Ok(get_shortest_path_infos::<T, A>(
        dist, &mut paths, with_paths,
    ))
}

fn dijkstra_no_options<T, A>(
    graph: &Graph<T, A>,
    weighted: bool,
    source: usize,
) -> Result<Vec<(usize, ShortestPathInfo<usize>)>, Error>
where
    T: Hash + Eq + Clone + Ord + Display + Send + Sync,
    A: Clone,
{
    // println!("source: {:?}", source);
    let mut paths: Vec<Vec<Vec<usize>>> = vec![vec![]; graph.number_of_nodes()];
    paths[source] = vec![vec![source]];

    let mut dist = vec![f64::MAX; graph.number_of_nodes()];
    let mut seen = vec![f64::MAX; graph.number_of_nodes()];
    let mut fringe = BinaryHeap::<FringeNode>::new();
    let mut count = 0;

    seen[source] = 0.0;
    fringe.push(FringeNode {
        node_index: source,
        count: 0,
        distance: -0.0,
    });

    while let Some(fringe_item) = fringe.pop() {
        let d = -fringe_item.distance;
        let v = fringe_item.node_index;
        // println!("    v: {}", v);
        if dist[v] != f64::MAX {
            continue;
        }
        dist[v] = d;
        for adj in graph.get_successor_nodes_by_index(&v) {
            let u = adj.node_index;
            // println!("        u: {}", u);
            let cost = match weighted {
                true => adj.weight,
                false => 1.0,
            };
            // println!("            cost: {}", cost);
            let vu_dist = dist[v] + cost;
            // println!("            vu_dist: {}", vu_dist);
            if vu_dist < seen[u] {
                // println!("            vu_dist < seen[u]");
                seen[u] = vu_dist;
                push_fringe_node(&mut count, &mut fringe, u, vu_dist);
                let mut new_paths_v = paths[v].clone();
                new_paths_v.iter_mut().for_each(|pv| pv.push(u));
                paths[u] = new_paths_v;
            } else if vu_dist == seen[u] {
                // println!("            vu_dist == seen[u]");
                push_fringe_node(&mut count, &mut fringe, u, vu_dist);
                add_u_to_v_paths_and_append_v_paths_to_u_paths(u, v, &mut paths);
            }
        }
    }

    Ok(get_shortest_path_infos::<T, A>(dist, &mut paths, true))
}

/// Returns the `Error` object for a contradictory-paths error.
#[inline]
fn get_contractory_paths_error() -> Error {
    Error {
        kind: ErrorKind::ContradictoryPaths,
        message: CONTRADICTORY_PATHS_ERROR_MESSAGE.to_string(),
    }
}

/**
Pushes a `FringeNode` into the `fringe` `BinaryHeap`.
Increments `count`.
*/
#[inline]
fn push_fringe_node(count: &mut i32, fringe: &mut BinaryHeap<FringeNode>, u: usize, vu_dist: f64) {
    *count += 1;
    fringe.push(FringeNode {
        node_index: u,
        count: *count,
        distance: -vu_dist, // negative because BinaryHeap is a max heap
    });
}

/**
Adds `u` to the paths that lead to `v`, then appends all the paths that
lead to `v` to the paths that lead to `u`.
*/
#[inline]
fn add_u_to_v_paths_and_append_v_paths_to_u_paths(
    u: usize,
    v: usize,
    paths: &mut Vec<Vec<Vec<usize>>>,
) {
    // add u to all paths[v], then *append* them to paths[u]
    let v_paths: Vec<Vec<usize>> = paths[v]
        .iter()
        .map(|p| {
            let mut x = p.clone();
            x.push(u.clone());
            x
        })
        .collect();
    for v_path in v_paths {
        paths[u].push(v_path);
    }
}

/**
Gets all shortest paths that pass through a given node.

Does not return paths where the node is the source or the target;
the node must be between two other nodes.

# Arguments

* `graph`: a [Graph](../../../struct.Graph.html) instance
* `node_name`: the name of the intermediate node
* `weighted`: determines if shortest paths are determined with edge weight, or not

# Examples

```
use graphrs::{Edge, Graph, GraphSpecs, Node};
use graphrs::{algorithms::{shortest_path::{dijkstra}}};

let mut graph = Graph::<&str, ()>::new(GraphSpecs::directed_create_missing());
graph.add_edges(vec![
    Edge::new("n1", "n2"),
    Edge::new("n2", "n3"),
    Edge::new("n1", "n4"),
    Edge::new("n4", "n3"),
    Edge::new("n2", "n5"),
]);
let result = dijkstra::get_all_shortest_paths_involving(&graph, "n2", false);
assert_eq!(result.len(), 2);
```
*/
pub fn get_all_shortest_paths_involving<T, A>(
    graph: &Graph<T, A>,
    node_name: T,
    weighted: bool,
) -> Vec<ShortestPathInfo<T>>
where
    T: Hash + Eq + Clone + Ord + Display + Send + Sync,
    A: Clone + Send + Sync,
{
    let result = all_pairs(graph, weighted, None, None, false, true);
    match result {
        Err(_) => vec![],
        Ok(pairs) => pairs
            .into_iter()
            .flat_map(|x| x.1.into_iter().map(|y| y.1))
            .filter(|x| x.contains_path_through_node(node_name.clone()))
            .collect(),
    }
}

/**
TODO: update doc
Zips the `distances` and the `paths` together into an `IntMap` where
the keys are the names of the target nodes and the values are
`ShortestPathInfo` objects.
*/
fn get_shortest_path_infos<T, A>(
    distances: Vec<f64>,
    paths: &mut Vec<Vec<Vec<usize>>>,
    with_paths: bool,
) -> Vec<(usize, ShortestPathInfo<usize>)>
where
    T: Hash + Eq + Clone + Ord + Display + Send + Sync,
    A: Clone,
{
    distances
        .into_iter()
        .enumerate()
        .filter(|(_k, v)| *v != f64::MAX)
        .map(|(k, v)| {
            let paths = match with_paths {
                true => mem::take(&mut paths[k]),
                false => vec![],
            };
            (k, ShortestPathInfo { distance: v, paths })
        })
        .collect()
}

fn convert_shortest_path_info_index_to_t<T, A>(
    graph: &Graph<T, A>,
    spi: ShortestPathInfo<usize>,
) -> ShortestPathInfo<T>
where
    T: Hash + Eq + Clone + Ord + Display + Send + Sync,
    A: Clone + Send + Sync,
{
    ShortestPathInfo::<T> {
        distance: spi.distance,
        paths: spi
            .paths
            .iter()
            .map(|p| {
                p.iter()
                    .map(|i| graph.get_node_by_index(i).unwrap().name.clone())
                    .collect()
            })
            .collect(),
    }
}

fn convert_shortest_path_info_index_map_to_t_map<T, A>(
    graph: &Graph<T, A>,
    spi_map: Vec<(usize, ShortestPathInfo<usize>)>,
) -> HashMap<T, ShortestPathInfo<T>>
where
    T: Hash + Eq + Clone + Ord + Display + Send + Sync,
    A: Clone + Send + Sync,
{
    spi_map
        .into_iter()
        .map(|(k, v)| {
            (
                graph.get_node_by_index(&k).unwrap().name.clone(),
                convert_shortest_path_info_index_to_t(graph, v),
            )
        })
        .collect()
}

fn can_use_no_options<T>(target: Option<T>, cutoff: Option<f64>, first_only: bool) -> bool {
    target.is_none() && cutoff.is_none() && first_only == false
}
