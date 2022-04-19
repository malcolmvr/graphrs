use super::utility::get_neighbors_of_nodes;
use crate::{
    ext::{hashset::HashSetExt, iterator::IteratorExt},
    Graph,
};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::hash::Hash;

pub struct TrianglesAndDegrees<T> {
    pub node_name: T,
    pub degree: usize,
    pub number_of_triangles: usize,
    pub generalized_degree: HashMap<usize, usize>,
}

/**
Returns a `Vec<TrianglesAndDegrees>` for a given graph and list of nodes.
This double counts triangles so you may want to divide by 2.

# Arguments

* `graph`: a [Graph](../../struct.Graph.html) instance
* `node_names`: Node names that optionally define a subset of the graph to work with
*/
pub fn get_triangles_and_degrees<T, A>(
    graph: &Graph<T, A>,
    node_names: Option<&[T]>,
) -> Vec<TrianglesAndDegrees<T>>
where
    T: Hash + Eq + Clone + Ord + Display + Send + Sync,
    A: Clone + Send + Sync,
{
    let neighbors_map = get_neighbors_of_nodes(node_names, graph);
    neighbors_map
        .clone()
        .into_iter()
        .map(|(v, v_nbrs)| get_triangles_and_degrees_for_node(v, v_nbrs, &neighbors_map))
        .collect()
}

/// Returns a `TrianglesAndDegrees` struct for a given node `v`.
fn get_triangles_and_degrees_for_node<T>(
    n: T,
    n_nbrs: HashSet<T>,
    neighbors_map: &HashMap<T, HashSet<T>>,
) -> TrianglesAndDegrees<T>
where
    T: Hash + Eq + Clone + Ord + Display + Send + Sync,
{
    let nbrs = &n_nbrs.without(&n);
    let generalized_degree: HashMap<usize, usize> = nbrs
        .iter()
        .map(|w| {
            let w_nbrs = neighbors_map.get(w).unwrap().without(w);
            w_nbrs.intersection(nbrs).collect::<HashSet<&T>>().len()
        })
        .sorted()
        .group_by_count()
        .collect();
    let ntriangles: usize = generalized_degree.iter().map(|(k, val)| k * val).sum();
    TrianglesAndDegrees {
        node_name: n,
        degree: nbrs.len(),
        number_of_triangles: ntriangles,
        generalized_degree,
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::{Edge, Graph, GraphSpecs};
    use std::collections::HashMap;

    #[test]
    fn test_get_triangles_and_degrees_1() {
        let edges = vec![
            Edge::with_weight("n0", "n1", 1.1),
            Edge::with_weight("n0", "n2", 1.3),
            Edge::with_weight("n0", "n3", 1.4),
            Edge::with_weight("n3", "n2", 1.5),
        ];
        let graph: Graph<&str, ()> =
            Graph::new_from_nodes_and_edges(vec![], edges, GraphSpecs::undirected_create_missing())
                .unwrap();
        let result = get_triangles_and_degrees(&graph, None);
        assert_eq!(result.len(), 4);
        let hm: HashMap<&str, TrianglesAndDegrees<&str>> =
            result.into_iter().map(|item| (item.node_name, item)).collect();
        let mut tad = hm.get("n0").unwrap();
        assert_eq!(tad.degree, 3);
        assert_eq!(tad.number_of_triangles, 2);
        assert_eq!(
            tad.generalized_degree,
            vec![(1, 2), (0, 1)].into_iter().collect()
        );
        tad = hm.get("n1").unwrap();
        assert_eq!(tad.degree, 1);
        assert_eq!(tad.number_of_triangles, 0);
        assert_eq!(tad.generalized_degree, vec![(0, 1)].into_iter().collect());
        tad = hm.get("n2").unwrap();
        assert_eq!(tad.degree, 2);
        assert_eq!(tad.number_of_triangles, 2);
        assert_eq!(tad.generalized_degree, vec![(1, 2)].into_iter().collect());
        tad = hm.get("n3").unwrap();
        assert_eq!(tad.degree, 2);
        assert_eq!(tad.number_of_triangles, 2);
        assert_eq!(tad.generalized_degree, vec![(1, 2)].into_iter().collect());
    }
}
