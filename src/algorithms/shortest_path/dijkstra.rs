use crate::algorithms::shortest_path::ShortestPathInfo;
use crate::{Error, ErrorKind, Graph, Node};
use nohash::IntMap;
use rayon::prelude::*;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::sync::Arc;

/**
As a graph is explored by a shortest-path algorithm the nodes at the
"fringe" of the explored part are maintained. This struct holds information
about a fringe node.
*/
struct FringeNode<T> {
    pub node_index: T,
    pub count: i32,
    pub distance: f64,
}

impl<T: Eq + Ord> Ord for FringeNode<T> {
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

impl<T: Eq + Ord> PartialOrd for FringeNode<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Eq> PartialEq for FringeNode<T> {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
            && self.count == other.count
            && self.node_index == other.node_index
    }
}

impl<T: Eq> Eq for FringeNode<T> {}

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

let all_pairs = dijkstra::all_pairs(&graph, true, None, false);
assert_eq!(all_pairs.unwrap().get("n1").unwrap().get("n3").unwrap().distance, 2.1);
```

# References

1. E. W. Dijkstra. A note on two problems in connection with graphs. Numer. Math., 1:269–271, 1959.

*/
pub fn all_pairs<T, A>(
    graph: &Graph<T, A>,
    weighted: bool,
    cutoff: Option<f64>,
    first_only: bool,
) -> Result<HashMap<T, HashMap<T, ShortestPathInfo<T>>>, Error>
where
    T: Hash + Eq + Clone + Ord + Debug + Display + Send + Sync,
    A: Clone + Send + Sync,
{
    let x = graph
        .get_all_nodes()
        .into_iter()
        .map(|node| {
            let ss = single_source(graph, weighted, node.name.clone(), None, cutoff, first_only);
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

// TODO: fix the multisource dijkstra - it contains a bug that gives incorrect results
pub fn all_pairs_ms<T, A>(
    graph: &Graph<T, A>,
    weighted: bool,
    cutoff: Option<f64>,
    first_only: bool,
) -> Result<HashMap<T, HashMap<T, ShortestPathInfo<T>>>, Error>
where
    T: Hash + Eq + Clone + Ord + Debug + Display + Send + Sync,
    A: Clone + Send + Sync,
{
    let source_indexes = (0..graph.number_of_nodes()).collect::<Vec<_>>();
    let shortest_paths =
        dijkstra_multisource_by_index(graph, weighted, source_indexes, None, cutoff, first_only)?;
    Ok(shortest_paths
        .into_iter()
        .map(|(k, v)| {
            let node_name = graph.get_node_by_index(&k).unwrap().name.clone();
            let mut inner_map = HashMap::new();
            inner_map.insert(
                node_name.clone(),
                convert_shortest_path_info_index_to_t(graph, v),
            );
            (node_name, inner_map)
        })
        .collect())
}

pub fn all_pairs_iter<'a, T, A>(
    graph: &'a Graph<T, A>,
    weighted: bool,
    cutoff: Option<f64>,
    first_only: bool,
) -> impl Iterator<Item = (T, HashMap<T, ShortestPathInfo<T>>)> + 'a
where
    T: Hash + Eq + Clone + Ord + Debug + Display + Send + Sync,
    A: Clone + Send + Sync,
{
    let x = graph
        .get_all_nodes()
        .into_iter()
        .map(move |node: &Arc<Node<T, A>>| {
            let ss = single_source(graph, weighted, node.name.clone(), None, cutoff, first_only);
            (node.name.clone(), ss.unwrap())
        });
    x
}

pub fn all_pairs_par_iter<'a, T, A>(
    graph: &'a Graph<T, A>,
    weighted: bool,
    cutoff: Option<f64>,
    first_only: bool,
) -> rayon::iter::Map<
    rayon::vec::IntoIter<usize>,
    impl Fn(usize) -> (T, HashMap<T, ShortestPathInfo<T>>) + 'a,
>
where
    T: Hash + Eq + Clone + Ord + Display + Send + Sync + 'a,
    A: Clone + Send + Sync + 'a,
{
    let x = (0..graph.number_of_nodes())
        .collect::<Vec<_>>()
        .into_par_iter()
        .map(move |node_index: usize| {
            let ss_index =
                single_source_by_index(&graph, weighted, node_index, None, cutoff, first_only)
                    .unwrap();
            let node = graph.get_node_by_index(&node_index).unwrap();
            let ss = convert_shortest_path_info_index_map_to_t_map(&graph, ss_index);
            (node.name.clone(), ss)
        });
    x
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

let shortest_paths = dijkstra::single_source(&graph, true, "n1", Some("n3"), None, false);
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
) -> Result<HashMap<T, ShortestPathInfo<T>>, Error>
where
    T: Hash + Eq + Clone + Ord + Debug + Display + Send + Sync,
    A: Clone + Send + Sync,
{
    multi_source(graph, weighted, vec![source], target, cutoff, first_only)
}

fn single_source_by_index<T, A>(
    graph: &Graph<T, A>,
    weighted: bool,
    source: usize,
    target: Option<usize>,
    cutoff: Option<f64>,
    first_only: bool,
) -> Result<IntMap<usize, ShortestPathInfo<usize>>, Error>
where
    T: Hash + Eq + Clone + Ord + Display + Send + Sync,
    A: Clone,
{
    multi_source_by_index(graph, weighted, vec![source], target, cutoff, first_only)
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

let shortest_paths = dijkstra::multi_source(&graph, true, vec!["n1", "n2"], Some("n3"), None, false);
assert_eq!(shortest_paths.unwrap().get("n3").unwrap().distance, 1.1);
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
) -> Result<HashMap<T, ShortestPathInfo<T>>, Error>
where
    T: Hash + Eq + Clone + Ord + Display + Send + Sync,
    A: Clone + Send + Sync,
{
    let sources_indexes = sources
        .iter()
        .map(|s| graph.get_node_index(s))
        .collect::<Vec<_>>();
    let target_index = match target.clone() {
        Some(t) => Some(graph.get_node_index(&t)),
        None => None,
    };
    let shortest_path_infos = dijkstra_multisource_by_index(
        graph,
        weighted,
        sources_indexes,
        target_index,
        cutoff,
        first_only,
    );
    match shortest_path_infos {
        Err(e) => Err(e),
        Ok(spis) => Ok(spis
            .into_iter()
            .filter(|(k, _v)| target.is_none() || *k == target_index.unwrap())
            .map(|(k, v)| {
                (
                    graph.get_node_by_index(&k).unwrap().name.clone(),
                    convert_shortest_path_info_index_to_t(&graph, v),
                )
            })
            .collect::<HashMap<T, ShortestPathInfo<T>>>()),
    }
}

pub fn multi_source_by_index<T, A>(
    graph: &Graph<T, A>,
    weighted: bool,
    sources: Vec<usize>,
    target: Option<usize>,
    cutoff: Option<f64>,
    first_only: bool,
) -> Result<IntMap<usize, ShortestPathInfo<usize>>, Error>
where
    T: Hash + Eq + Clone + Ord + Display + Send + Sync,
    A: Clone,
{
    dijkstra_multisource_by_index(graph, weighted, sources, target, cutoff, first_only)
}

/**
Uses Dijkstra's algorithm to find shortest weighted paths.
This is a private function that does all the work of finding the
shortest paths. All the public functions in this module call this one.
*/
fn dijkstra_multisource_by_index<T, A>(
    graph: &Graph<T, A>,
    weighted: bool,
    sources: Vec<usize>,
    target: Option<usize>,
    cutoff: Option<f64>,
    first_only: bool,
) -> Result<IntMap<usize, ShortestPathInfo<usize>>, Error>
where
    T: Hash + Eq + Clone + Ord + Display + Send + Sync,
    A: Clone,
{
    if weighted {
        graph.ensure_weighted()?;
    }

    let get_cost = |u, v| match weighted {
        true => match graph.specs.multi_edges {
            false => get_cost_single(graph, u, v),
            true => get_cost_multi(graph, u, v),
        },
        false => 1.0,
    };

    let mut paths: IntMap<usize, Vec<Vec<usize>>> = sources
        .iter()
        .map(|s| (s.clone(), vec![vec![s.clone()]]))
        .collect();
    let mut dist = IntMap::<usize, f64>::default();
    let mut seen = IntMap::<usize, f64>::default();
    let mut fringe = BinaryHeap::new();
    let mut count = 0;

    for source in sources {
        seen.insert(source, 0.0);
        fringe.push(FringeNode {
            node_index: source,
            count: 0,
            distance: -0.0,
        });
    }

    while let Some(fringe_item) = fringe.pop() {
        let d = -fringe_item.distance;
        let v = fringe_item.node_index;
        // println!("    v: {}", v);
        if dist.contains_key(&v) {
            continue;
        }
        dist.insert(v, d);
        if target.as_ref() == Some(&v) {
            break;
        }
        for u in graph.get_successors_or_neighbors_by_index(&v) {
            // println!("        u: {}", u);
            let cost = get_cost(v, u);
            // println!("            cost: {}", cost);
            let vu_dist = dist.get(&v).unwrap() + cost;
            // println!("            vu_dist: {}", vu_dist);
            if cutoff.map_or(false, |c| vu_dist > c) {
                // println!("            cutoff continue");
                continue;
            }
            if let Some(&u_dist) = dist.get(&u) {
                // println!("            u_dist = dist.get(u)");
                if vu_dist < u_dist {
                    return Err(get_contractory_paths_error());
                }
            } else if !seen.contains_key(&u) || vu_dist < *seen.get(&u).unwrap() {
                // println!("            !seen.contains_key(u)");
                seen.insert(u, vu_dist);
                push_fringe_node(&mut count, &mut fringe, u, vu_dist);
                let mut new_paths_v = paths.get(&v).cloned().unwrap_or_default();
                new_paths_v.iter_mut().for_each(|pv| pv.push(u));
                paths.insert(u, new_paths_v);
            } else if !first_only && vu_dist == *seen.get(&u).unwrap() {
                // println!("            !first_only");
                push_fringe_node(&mut count, &mut fringe, u, vu_dist);
                add_u_to_v_paths_and_append_v_paths_to_u_paths(u, v, &mut paths);
            }
        }
    }

    Ok(get_shortest_path_infos::<T, A>(dist, paths))
}

/*
fn dijkstra<T, A>(
    graph: &Graph<T, A>,
    src: usize,
    pq: &mut BinaryHeap<FringeNode<usize>>,
    S: &mut [i32],
    P: &mut [i32],
    P_start: &mut [i32],
    P_len: &mut [i32],
    sigma: &mut [f64],
    D: &mut [f64],
    seen: &mut [f64],
) -> usize
where
    T: Hash + Eq + Clone + Ord + Display + Send + Sync,
    A: Clone,
{
    let V = graph.V;

    pq.count = 0;

    for u in 0..V {
        pq.id[u] = -1;
        S[u] = -1;
        sigma[u] = 0.0;
        D[u] = f64::INFINITY;
        seen[u] = f64::INFINITY;
        P_len[u] = 0;
    }
    sigma[src] = 1.0;
    seen[src] = 0.0;
    pq.push(src, src, 0.0);
    let mut countS = 0;

    while pq.count > 0 {
        let top = pq.pop().unwrap();
        let pred = top.pred;
        let v = top.v;
        let dist = top.dist;

        if D[v] != f64::INFINITY {
            continue;
        }

        sigma[v] += sigma[pred];
        S[countS] = v as i32;
        countS += 1;
        D[v] = dist;

        let mut cur = graph.array[v].next.as_ref();
        while let Some(node) = cur {
            let w = node.dest;
            let weight = node.weight;
            let vw_dist = dist + weight;

            if D[w] == f64::INFINITY && (seen[w] == f64::INFINITY || vw_dist < seen[w]) {
                seen[w] = vw_dist;
                pq.push(v, w, vw_dist);
                sigma[w] = 0.0;
                P[P_start[w] as usize] = v as i32;
                P_len[w] = 1;
            } else if vw_dist == seen[w] {
                sigma[w] += sigma[v];
                P[(P_start[w] + P_len[w]) as usize] = v as i32;
                P_len[w] += 1;
            }
            cur = node.next.as_ref();
        }
    }
    countS
}
 */

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
fn push_fringe_node<T>(count: &mut i32, fringe: &mut BinaryHeap<FringeNode<T>>, u: T, vu_dist: f64)
where
    T: Hash + Eq + Clone + Ord + Display + Send + Sync,
{
    *count += 1;
    fringe.push(FringeNode {
        node_index: u,
        count: *count,
        distance: -vu_dist,
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
    paths: &mut IntMap<usize, Vec<Vec<usize>>>,
) {
    // add u to all paths[v], then *append* them to paths[u]
    let v_paths: Vec<Vec<usize>> = paths
        .get(&v)
        .unwrap()
        .iter()
        .map(|p| {
            let mut x = p.clone();
            x.push(u.clone());
            x
        })
        .collect();
    let u_paths = paths.get_mut(&u).unwrap();
    for v_path in v_paths {
        u_paths.push(v_path);
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
    T: Hash + Eq + Clone + Ord + Debug + Display + Send + Sync,
    A: Clone + Send + Sync,
{
    let result = all_pairs(graph, weighted, None, false);
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
Returns the "cost" of a (`u`, `v`) edges when the `graph` is a multigraph.

Finds lowest weight of the (u, v) edges.
*/
fn get_cost_multi<T, A>(graph: &Graph<T, A>, u: usize, v: usize) -> f64
where
    T: Hash + Eq + Clone + Ord + Display + Send + Sync,
    A: Clone,
{
    let edges = graph.get_edges_by_indexes(u, v).unwrap();
    let weights = edges.into_iter().map(|e| e.weight);
    weights.into_iter().reduce(f64::min).unwrap()
}

/**
Returns the weight of the (u, v) edge in a `graph` that is not a multigraph.
*/
fn get_cost_single<T, A>(graph: &Graph<T, A>, u: usize, v: usize) -> f64
where
    T: Hash + Eq + Clone + Ord + Display + Send + Sync,
    A: Clone,
{
    let edge = graph.get_edge_by_indexes(u, v).unwrap();
    edge.weight
}

/**
Zips the `distances` and the `paths` together into a `HashMap` where
the keys are the names of the target nodes and the values are
`ShortestPathInfo` objects.
*/
fn get_shortest_path_infos<T, A>(
    distances: IntMap<usize, f64>,
    paths: IntMap<usize, Vec<Vec<usize>>>,
) -> IntMap<usize, ShortestPathInfo<usize>>
where
    T: Hash + Eq + Clone + Ord + Display + Send + Sync,
    A: Clone,
{
    distances
        .into_iter()
        .map(|(k, v)| {
            (
                k.clone(),
                ShortestPathInfo {
                    distance: v,
                    paths: paths.get(&k).unwrap().clone(),
                },
            )
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
    spi_map: IntMap<usize, ShortestPathInfo<usize>>,
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
