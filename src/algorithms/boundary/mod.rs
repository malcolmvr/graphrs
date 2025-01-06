use crate::{Edge, Error, ErrorKind, Graph};
use nohash::IntSet;
use std::collections::HashSet;
use std::fmt::Debug;
use std::fmt::Display;
use std::hash::Hash;
use std::sync::Arc;

/**
Returns the edge boundary for a bunch of nodes.

The *edge boundary* of a set *S* with respect to a set *T* is the
set of edges (*u*, *v*) such that *u* is in *S* and *v* is in *T*.
If *T* is not specified, it is assumed to be the set of all nodes
not in *S*.

# Arguments

* `graph`: the `Graph` the nodes are in
* `nbunch1`: the first set of nodes
* `nbunch2`: the second set of nodes

# Examples

```
use graphrs::{algorithms::boundary::edge_boundary, generators, Graph};

let graph = generators::social::karate_club_graph();
let edges = edge_boundary(&graph, &[0, 1, 2, 3], Some(&[4, 5, 6, 7])).unwrap();
assert_eq!(edges.len(), 7);
```

*/
pub fn edge_boundary<'a, T, A>(
    graph: &'a Graph<T, A>,
    nbunch1: &[T],
    nbunch2: Option<&[T]>,
) -> Result<Vec<&'a Arc<Edge<T, A>>>, Error>
where
    T: Hash + Eq + Clone + Ord + Debug + Display + Send + Sync,
    A: Clone + Send + Sync,
{
    if !graph.has_nodes(nbunch1) {
        return Err(Error {
            kind: ErrorKind::NodeNotFound,
            message: "One or more of `nbunch1` were not found in the graph.".to_string(),
        });
    }
    if nbunch2.is_some() && !graph.has_nodes(nbunch2.unwrap()) {
        return Err(Error {
            kind: ErrorKind::NodeNotFound,
            message: "One or more of `nbunch2` were not found in the graph.".to_string(),
        });
    }

    let out_edges = match graph.specs.directed {
        true => graph.get_out_edges_for_nodes(nbunch1),
        false => graph.get_edges_for_nodes(nbunch1),
    }
    .unwrap();
    let nset1 = nbunch1.iter().cloned().collect::<HashSet<T>>();
    let nset2 = match nbunch2 {
        Some(nbunch2) => nbunch2.iter().cloned().collect::<HashSet<T>>(),
        None => graph
            .get_all_node_names()
            .into_iter()
            .filter(|n| !nset1.contains(n))
            .cloned()
            .collect::<HashSet<T>>(),
    };
    return Ok(out_edges
        .into_iter()
        .filter(|e| {
            (nset1.contains(&e.u) && nset2.contains(&e.v))
                || (nset2.contains(&e.u) && nset1.contains(&e.v))
        })
        .collect());
}

pub(crate) fn edge_boundary_by_indexes<'a, T, A>(
    graph: &'a Graph<T, A>,
    nbunch1: &[usize],
    nbunch2: &[usize],
) -> Vec<(usize, usize, f64)>
where
    T: Hash + Eq + Clone + Ord + Debug + Display + Send + Sync,
    A: Clone + Send + Sync,
{
    let out_edges = graph.get_out_edges_for_node_indexes(nbunch1);
    let nset1 = nbunch1.iter().cloned().collect::<IntSet<usize>>();
    let nset2 = nbunch2.iter().cloned().collect::<IntSet<usize>>();
    out_edges
        .into_iter()
        .filter(|(u, v, _weight)| {
            (nset1.contains(&u) && nset2.contains(&v)) || (nset2.contains(&u) && nset1.contains(&v))
        })
        .collect()
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::{generators, GraphSpecs};
    use assert_unordered::assert_eq_unordered;

    #[test]
    fn test_edge_boundary_1() {
        let edges = vec![
            Edge::new("n1", "n2"),
            Edge::new("n1", "n3"),
            Edge::new("n2", "n1"),
            Edge::new("n2", "n3"),
        ];
        let specs = GraphSpecs::directed_create_missing();
        let graph: Graph<&str, ()> = Graph::new_from_nodes_and_edges(vec![], edges, specs).unwrap();
        let result = edge_boundary(&graph, &["n1"], None).unwrap();
        assert_eq!(result.len(), 2);
        let essence = result.iter().map(|e| (e.u, e.v)).collect::<Vec<_>>();
        assert_eq_unordered!(essence, vec![("n1", "n2"), ("n1", "n3")]);
    }

    #[test]
    fn test_edge_boundary_2() {
        let edges = vec![
            Edge::new("n1", "n2"),
            Edge::new("n1", "n3"),
            Edge::new("n2", "n1"),
            Edge::new("n2", "n3"),
        ];
        let specs = GraphSpecs::directed_create_missing();
        let graph: Graph<&str, ()> = Graph::new_from_nodes_and_edges(vec![], edges, specs).unwrap();
        let result = edge_boundary(&graph, &["n1"], Some(&["n2", "n3"])).unwrap();
        let essence = result.iter().map(|e| (e.u, e.v)).collect::<Vec<_>>();
        assert_eq_unordered!(essence, vec![("n1", "n2"), ("n1", "n3")]);
    }

    #[test]
    fn test_edge_boundary_3() {
        let edges = vec![
            Edge::new("n1", "n3"),
            Edge::new("n2", "n1"),
            Edge::new("n2", "n3"),
        ];
        let specs = GraphSpecs::directed_create_missing();
        let graph: Graph<&str, ()> = Graph::new_from_nodes_and_edges(vec![], edges, specs).unwrap();
        let result = edge_boundary(&graph, &["n1"], Some(&["n2", "n3"])).unwrap();
        let essence = result.iter().map(|e| (e.u, e.v)).collect::<Vec<_>>();
        assert_eq_unordered!(essence, vec![("n1", "n3")]);
    }

    #[test]
    fn test_edge_boundary_4() {
        let edges = vec![
            Edge::new("n1", "n3"),
            Edge::new("n2", "n1"),
            Edge::new("n2", "n3"),
        ];
        let specs = GraphSpecs::undirected_create_missing();
        let graph: Graph<&str, ()> = Graph::new_from_nodes_and_edges(vec![], edges, specs).unwrap();
        let result = edge_boundary(&graph, &["n1"], Some(&["n2", "n3"])).unwrap();
        let essence = result.iter().map(|e| (e.u, e.v)).collect::<Vec<_>>();
        assert_eq_unordered!(essence, vec![("n1", "n2"), ("n1", "n3")]);
    }

    #[test]
    fn test_edge_boundary_5() {
        let graph = generators::social::karate_club_graph();
        let result = edge_boundary(&graph, &[0, 1, 2, 3], Some(&[4, 5, 6, 7])).unwrap();
        let essence = result.iter().map(|e| (e.u, e.v)).collect::<Vec<_>>();
        assert_eq_unordered!(
            essence,
            vec![(0, 4), (0, 5), (0, 6), (0, 7), (1, 7), (2, 7), (3, 7)]
        );
    }
}
