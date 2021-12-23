use crate::{Error, ErrorKind, Graph, Node};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::fmt::Display;
use std::hash::Hash;

/**
Information about the unweighted shortest path between two nodes.
*/
pub struct ShortestPathUnweighted<T> {
    /// The distance between two nodes.
    pub distance: i32,
    /// The path between two nodes. The first item is the starting node
    /// and the last item is the target node.
    pub path: Vec<T>,
}

/**
As a graph is explored by a shortest-path algorithm the nodes at the
"fringe" of the explored part are maintained. This struct holds information
about a fringe node.
*/
struct FringeNode<T> {
    pub node_name: T,
    pub count: i32,
    pub distance: i32,
}

impl<T: Eq + Ord> Ord for FringeNode<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.distance.cmp(&other.distance) {
            Ordering::Equal => match self.count.cmp(&other.count) {
                Ordering::Equal => self.node_name.cmp(&other.node_name),
                ordering => ordering,
            },
            ordering => ordering,
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
            && self.count == self.count
            && self.node_name == self.node_name
    }
}

impl<T: Eq> Eq for FringeNode<T> {}

/**
Uses Dijkstra's algorithm to find shortest unweighted paths between all pairs
of nodes.

# Arguments

* `graph`: a `Graph` instance where all edges have a weight.
* `cutoff`: Length at which the search is stopped.
If cutoff is provided, only return paths with length <= cutoff.

# Returns

A `HashMap` of `HashMaps`. The keys to the first one are the starting nodes
and the keys to the second are the target nodes.

# Examples

```
use graphrs::{algorithms::{shortest_path::{unweighted}}, generators};
let graph = generators::social::karate_club_graph();
let all_pairs = unweighted::all_pairs_dijkstra(&graph, None);
assert_eq!(all_pairs.unwrap().get(&0).unwrap().get(&20).unwrap().distance, 3);
```
*/
pub fn all_pairs_dijkstra<T, A>(
    graph: &Graph<T, A>,
    cutoff: Option<i32>,
) -> Result<HashMap<T, HashMap<T, ShortestPathUnweighted<T>>>, Error>
where
    T: Hash + Eq + Copy + Ord + Display,
    A: Copy,
{
    Ok(graph
        .get_all_nodes()
        .iter()
        .map(|n| {
            (
                n.name,
                single_source_dijkstra(graph, n.name, None, cutoff).unwrap(),
            )
        })
        .collect())
}

/**
Uses Dijkstra's algorithm to find shortest unweighted paths from a single source node.

# Arguments

* `graph`: a `Graph` instance where all edges have a weight.
* `source`: The starting node.
* `target`: The ending node. If `None` then the shortest paths between `source` and
all other nodes will be found.
* `cutoff`: Length (sum of edge weights) at which the search is stopped.
If cutoff is provided, only return paths with summed weight <= cutoff.

# Examples

```
use graphrs::{algorithms::{shortest_path::{unweighted}}, generators};
let graph = generators::social::karate_club_graph();
let shortest_paths = unweighted::single_source_dijkstra(&graph, 0, Some(20), None);
assert_eq!(shortest_paths.unwrap().get(&20).unwrap().distance, 3);
```
*/
pub fn single_source_dijkstra<T, A>(
    graph: &Graph<T, A>,
    source: T,
    target: Option<T>,
    cutoff: Option<i32>,
) -> Result<HashMap<T, ShortestPathUnweighted<T>>, Error>
where
    T: Hash + Eq + Copy + Ord + Display,
    A: Copy,
{
    multi_source_dijkstra(graph, vec![source], target, cutoff)
}

/**
Uses Dijkstra's algorithm to find shortest weighted paths from multiple source nodes.

# Arguments

* `graph`: a `Graph` instance where all edges have a weight.
* `sources`: The starting nodes. The shortest path will be found that can start
for any of the `sources` and ends at the `target`.
* `target`: The ending node. If `None` then the shortest paths between `sources` and
all other nodes will be found.
* `cutoff`: Length (sum of edge weights) at which the search is stopped.
If cutoff is provided, only return paths with summed weight <= cutoff.

# Examples

```
use graphrs::{algorithms::{shortest_path::{unweighted}}, generators};
let graph = generators::social::karate_club_graph();
let shortest_paths = unweighted::multi_source_dijkstra(&graph, vec![0,2], Some(20), None);
assert_eq!(shortest_paths.unwrap().get(&20).unwrap().distance, 2);
```
*/
pub fn multi_source_dijkstra<T, A>(
    graph: &Graph<T, A>,
    sources: Vec<T>,
    target: Option<T>,
    cutoff: Option<i32>,
) -> Result<HashMap<T, ShortestPathUnweighted<T>>, Error>
where
    T: Hash + Eq + Copy + Ord + Display,
    A: Copy,
{
    let result = dijkstra_multisource(graph, sources, target, cutoff);
    match result {
        Err(e) => Err(e),
        Ok((distances, paths)) => Ok(distances
            .into_iter()
            .map(|(k, v)| {
                (
                    k,
                    ShortestPathUnweighted {
                        distance: v,
                        path: paths.get(&k).unwrap().clone(),
                    },
                )
            })
            .collect::<HashMap<T, ShortestPathUnweighted<T>>>()),
    }
}

/**
Uses Dijkstra's algorithm to find shortest unweighted paths.
This is a private function that does all the work of finding the
shortest paths. All the public functions in this module call this one.
*/
fn dijkstra_multisource<T, A>(
    graph: &Graph<T, A>,
    sources: Vec<T>,
    target: Option<T>,
    cutoff: Option<i32>,
) -> Result<(HashMap<T, i32>, HashMap<T, Vec<T>>), Error>
where
    T: Hash + Eq + Copy + Ord + Display,
    A: Copy,
{
    let mut paths: HashMap<T, Vec<T>> = sources
        .iter()
        .map(|s| (s.clone(), vec![s.clone()]))
        .collect();
    let mut dist = HashMap::<T, i32>::new();
    let mut seen = HashMap::<T, i32>::new();
    let mut fringe = BinaryHeap::new();
    let mut count = 0;

    for source in sources {
        seen.insert(source, 0);
        fringe.push(FringeNode {
            node_name: source,
            count: 0,
            distance: -0,
        });
    }

    while fringe.len() > 0 {
        let fringe_item = fringe.pop().unwrap();
        let d = -fringe_item.distance;
        let v = fringe_item.node_name;
        if dist.contains_key(&v) {
            continue;
        }
        dist.insert(v, d);
        if target.is_some() && v == target.unwrap().clone() {
            break;
        }
        let sorn = get_successors_or_neighbors(graph, v);
        for node in sorn {
            let u = node.name;
            let vu_dist = dist.get(&v).unwrap() + 1;
            if cutoff.is_some() && vu_dist > cutoff.unwrap() {
                continue;
            }
            if dist.contains_key(&u) {
                let u_dist = dist.get(&u).unwrap().clone();
                if vu_dist < u_dist {
                    return Err(Error {
                        kind: ErrorKind::ContradictoryPaths,
                        message: "Contradictary paths found, do some edges have negative weights?"
                            .to_string(),
                    });
                }
            } else if !seen.contains_key(&u) || vu_dist < seen.get(&u).unwrap().clone() {
                seen.insert(u, vu_dist);
                count += 1;
                fringe.push(FringeNode {
                    node_name: u,
                    count: count,
                    distance: -vu_dist,
                });
                let mut paths_v = paths.entry(v).or_default().clone();
                paths_v.push(u);
                paths.insert(u, paths_v);
            }
        }
    }

    Ok((dist, paths))
}

/**
Returns successors of a node if the `graph` is directed.
Returns neighbors of a node if the `graph` is undirected.
*/
fn get_successors_or_neighbors<T, A>(graph: &Graph<T, A>, node_name: T) -> Vec<&Node<T, A>>
where
    T: Hash + Eq + Copy + Ord + Display,
    A: Copy,
{
    match graph.specs.directed {
        true => graph.get_successor_nodes(node_name).unwrap(),
        false => graph.get_neighbor_nodes(node_name).unwrap(),
    }
}
