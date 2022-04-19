use super::utility::get_adjacent_nodes_without;
use crate::Graph;
use std::fmt::Display;
use std::hash::Hash;

pub struct DirectedTrianglesAndDegree<T> {
    pub node_name: T,
    pub total_degree: usize,
    pub reciprocal_degree: usize,
    pub directed_triangles: usize,
}

/**
Returns a `Vec<DirectedTrianglesAndDegree>` for a given graph and list of nodes.
Unlike `get_triangles_and_degrees` this function does not count triangles twice.

# Arguments

* `graph`: a [Graph](../../struct.Graph.html) instance
* `node_names`: Node names that optionally define a subset of the graph to work with
*/
pub fn get_directed_triangles_and_degrees<T, A>(
    graph: &Graph<T, A>,
    node_names: Option<&[T]>,
) -> Vec<DirectedTrianglesAndDegree<T>>
where
    T: Hash + Eq + Clone + Ord + Display + Send + Sync,
    A: Clone + Send + Sync,
{
    let ns: Vec<T> = match node_names {
        None => graph.get_all_nodes().into_iter().map(|n| n.name.clone()).collect(),
        Some(names) => names.to_vec(),
    };
    ns.into_iter()
        .map(|i| {
            let ipreds = get_adjacent_nodes_without(graph, &i, true);
            let isuccs = get_adjacent_nodes_without(graph, &i, false);
            let directed_triangles = &ipreds
                .iter()
                .chain(&isuccs)
                .map(|j| {
                    let jpreds = get_adjacent_nodes_without(graph, j, true);
                    let jsuccs = get_adjacent_nodes_without(graph, j, false);
                    ipreds
                        .clone()
                        .intersection(&jpreds)
                        .chain(ipreds.intersection(&jsuccs))
                        .chain(isuccs.intersection(&jpreds))
                        .chain(isuccs.intersection(&jsuccs))
                        .count()
                })
                .sum::<usize>();
            let total_degree = ipreds.len() + isuccs.len();
            let reciprocal_degree = ipreds.intersection(&isuccs).count();
            DirectedTrianglesAndDegree {
                node_name: i,
                total_degree,
                reciprocal_degree,
                directed_triangles: *directed_triangles,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::{Edge, Graph, GraphSpecs};
    use std::collections::HashMap;

    #[test]
    fn test_get_directed_triangles_and_degrees_1() {
        let edges = vec![
            Edge::with_weight("n0", "n1", 1.1),
            Edge::with_weight("n0", "n2", 1.3),
            Edge::with_weight("n0", "n3", 1.4),
            Edge::with_weight("n3", "n2", 1.5),
        ];
        let graph: Graph<&str, ()> =
            Graph::new_from_nodes_and_edges(vec![], edges, GraphSpecs::directed_create_missing())
                .unwrap();
        let result = get_directed_triangles_and_degrees(&graph, None);
        assert_eq!(result.len(), 4);
        let hm: HashMap<&str, DirectedTrianglesAndDegree<&str>> =
            result.into_iter().map(|item| (item.node_name, item)).collect();
        let mut dtad = hm.get("n0").unwrap();
        assert_eq!(dtad.total_degree, 3);
        assert_eq!(dtad.reciprocal_degree, 0);
        assert_eq!(dtad.directed_triangles, 2);
        dtad = hm.get("n1").unwrap();
        assert_eq!(dtad.total_degree, 1);
        assert_eq!(dtad.reciprocal_degree, 0);
        assert_eq!(dtad.directed_triangles, 0);
        dtad = hm.get("n2").unwrap();
        assert_eq!(dtad.total_degree, 2);
        assert_eq!(dtad.reciprocal_degree, 0);
        assert_eq!(dtad.directed_triangles, 2);
        dtad = hm.get("n3").unwrap();
        assert_eq!(dtad.total_degree, 2);
        assert_eq!(dtad.reciprocal_degree, 0);
        assert_eq!(dtad.directed_triangles, 2);
    }
}
