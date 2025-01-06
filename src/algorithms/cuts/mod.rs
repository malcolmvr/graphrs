use crate::{
    algorithms::boundary::{edge_boundary, edge_boundary_by_indexes},
    Error, Graph,
};
use std::fmt::Debug;
use std::fmt::Display;
use std::hash::Hash;

/**
Returns the size of the cut between two bunches of nodes.

A *cut* is a partition of the nodes of a graph into two sets. The
*cut size* is the sum of the weights of the edges "between" the two
sets of nodes.

# Arguments

* `graph`: the `Graph` the nodes are in
* `nbunch1`: the first set of nodes
* `nbunch2`: the second set of nodes
* `weighted`: whether to consider edge weights

```
use graphrs::{algorithms::cuts::cut_size, generators, Graph};

let graph = generators::social::karate_club_graph();
let size = cut_size(&graph, &[0, 1, 2, 3], &[4, 5, 6, 7], true).unwrap();
assert_eq!(size, 22.0);
```

*/
pub fn cut_size<T, A>(
    graph: &Graph<T, A>,
    nbunch1: &[T],
    nbunch2: &[T],
    weighted: bool,
) -> Result<f64, Error>
where
    T: Hash + Eq + Clone + Ord + Debug + Display + Send + Sync,
    A: Clone + Send + Sync,
{
    let mut edges = edge_boundary(graph, nbunch1, Some(nbunch2))?;
    edges = match graph.specs.directed {
        false => edges,
        true => {
            edges.extend(edge_boundary(graph, nbunch2, Some(nbunch1))?);
            edges
        }
    };
    Ok(edges
        .into_iter()
        .map(|e| match weighted {
            true => e.weight,
            false => 1.0,
        })
        .sum())
}

pub(crate) fn cut_size_by_indexes<T, A>(
    graph: &Graph<T, A>,
    nbunch1: &[usize],
    nbunch2: &[usize],
    weighted: bool,
) -> f64
where
    T: Hash + Eq + Clone + Ord + Debug + Display + Send + Sync,
    A: Clone + Send + Sync,
{
    let mut edges = edge_boundary_by_indexes(graph, nbunch1, nbunch2);
    edges = match graph.specs.directed {
        false => edges,
        true => {
            edges.extend(edge_boundary_by_indexes(graph, nbunch2, nbunch1));
            edges
        }
    };
    edges
        .into_iter()
        .map(|e| match weighted {
            true => e.2,
            false => 1.0,
        })
        .sum()
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::{generators, Edge, GraphSpecs};

    #[test]
    fn test_cut_size_1() {
        let edges = vec![
            Edge::new("n1", "n2"),
            Edge::new("n1", "n3"),
            Edge::new("n2", "n1"),
            Edge::new("n2", "n3"),
        ];
        let specs = GraphSpecs::directed_create_missing();
        let graph: Graph<&str, ()> = Graph::new_from_nodes_and_edges(vec![], edges, specs).unwrap();
        let result = cut_size(&graph, &["n1"], &["n2"], false).unwrap();
        assert_eq!(result, 2.0);
    }

    #[test]
    fn test_cut_size_2() {
        let edges = vec![
            Edge::with_weight("n1", "n2", 1.1),
            Edge::with_weight("n1", "n3", 2.3),
            Edge::with_weight("n2", "n1", 3.5),
            Edge::with_weight("n2", "n3", 4.7),
        ];
        let specs = GraphSpecs::directed_create_missing();
        let graph: Graph<&str, ()> = Graph::new_from_nodes_and_edges(vec![], edges, specs).unwrap();
        let result = cut_size(&graph, &["n1"], &["n2"], true).unwrap();
        assert_eq!(result, 4.6);
    }

    #[test]
    fn test_cut_size_3() {
        let edges = vec![
            Edge::new("n1", "n3"),
            Edge::new("n2", "n1"),
            Edge::new("n2", "n3"),
        ];
        let specs = GraphSpecs::undirected_create_missing();
        let graph: Graph<&str, ()> = Graph::new_from_nodes_and_edges(vec![], edges, specs).unwrap();
        let result = cut_size(&graph, &["n1"], &["n2"], false).unwrap();
        assert_eq!(result, 1.0);
    }

    #[test]
    fn test_cut_size_4() {
        let edges = vec![
            Edge::with_weight("n1", "n3", 2.3),
            Edge::with_weight("n2", "n1", 3.5),
            Edge::with_weight("n2", "n3", 4.7),
        ];
        let specs = GraphSpecs::undirected_create_missing();
        let graph: Graph<&str, ()> = Graph::new_from_nodes_and_edges(vec![], edges, specs).unwrap();
        let result = cut_size(&graph, &["n1"], &["n2"], true).unwrap();
        assert_eq!(result, 3.5);
    }

    #[test]
    fn test_cut_size_5() {
        let graph = generators::social::karate_club_graph();
        let result = cut_size(&graph, &[0, 1, 2, 3], &[4, 5, 6, 7], true).unwrap();
        assert_eq!(result, 22.0);
    }
}
