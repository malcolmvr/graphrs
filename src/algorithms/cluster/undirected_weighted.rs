use super::utility::{get_neighbors_of_nodes, get_normalized_edge_weight};
use crate::{ext::hashset::HashSetExt, Graph};
use std::collections::HashSet;
use std::fmt::Display;
use std::hash::Hash;

pub struct DegreesAndWeightedTriangles<T> {
    pub node_name: T,
    pub degree: usize,
    pub weighted_triangles: f64,
}

/**
Returns a `Vec<DegreesAndWeightedTriangles>` for a given graph and list of nodes.
Unlike `get_triangles_and_degrees` this function does not count triangles twice.

# Arguments

* `graph`: a [Graph](../../struct.Graph.html) instance
* `node_names`: Node names that optionally define a subset of the graph to work with
*/
pub fn get_weighted_triangles_and_degrees<T, A>(
    graph: &Graph<T, A>,
    node_names: Option<&[T]>,
) -> Vec<DegreesAndWeightedTriangles<T>>
where
    T: Hash + Eq + Clone + Ord + Display + Send + Sync,
    A: Clone + Send + Sync,
{
    let edges = graph.get_all_edges();
    let max_weight = match edges.is_empty() {
        true => 1.0,
        false => edges.iter().map(|e| e.weight).reduce(f64::max).unwrap(),
    };
    let nodes_nbrs = get_neighbors_of_nodes(node_names, graph);
    nodes_nbrs
        .into_iter()
        .map(|(n, n_nbrs)| {
            get_weighted_triangles_and_degrees_for_node(n, n_nbrs, graph, max_weight)
        })
        .collect()
}

fn get_weighted_triangles_and_degrees_for_node<T, A>(
    n: T,
    n_nbrs: HashSet<T>,
    graph: &Graph<T, A>,
    max_weight: f64,
) -> DegreesAndWeightedTriangles<T>
where
    T: Hash + Eq + Clone + Ord + Display + Send + Sync,
    A: Clone + Send + Sync,
{
    let mut seen = HashSet::<T>::new();
    let nbrs: HashSet<T> = n_nbrs.without(&n);
    let wt = |u: &T, v: &T| get_normalized_edge_weight(u, v, &max_weight, graph);
    let weighted_triangles: f64 = nbrs
        .iter()
        .map(|u| {
            seen.insert(u.clone());
            let unbrs = graph
                .get_neighbor_nodes(u.clone())
                .unwrap()
                .into_iter()
                .map(|n| n.name.clone())
                .collect::<HashSet<T>>()
                .difference(&seen)
                .cloned()
                .collect();
            let wnu = wt(&n, u);
            nbrs.intersection(&unbrs)
                .into_iter()
                .map(|k| f64::cbrt(wnu * wt(u, k) * wt(k, &n)))
                .sum::<f64>()
        })
        .sum::<f64>()
        * 2.0;
    DegreesAndWeightedTriangles {
        node_name: n,
        degree: nbrs.len(),
        weighted_triangles,
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::{Edge, Graph, GraphSpecs};
    use std::collections::HashMap;

    #[test]
    fn test_get_weighted_triangles_and_degrees_1() {
        let edges = vec![
            Edge::with_weight("n0", "n1", 1.1),
            Edge::with_weight("n0", "n2", 1.3),
            Edge::with_weight("n0", "n3", 1.4),
            Edge::with_weight("n3", "n2", 1.5),
        ];
        let graph: Graph<&str, ()> =
            Graph::new_from_nodes_and_edges(vec![], edges, GraphSpecs::undirected_create_missing())
                .unwrap();
        let result = get_weighted_triangles_and_degrees(&graph, None);
        assert_eq!(result.len(), 4);
        let hm: HashMap<&str, DegreesAndWeightedTriangles<&str>> = result
            .into_iter()
            .map(|item| (item.node_name, item))
            .collect();
        let mut dawt = hm.get("n0").unwrap();
        assert_eq!(dawt.degree, 3);
        assert_eq!(dawt.weighted_triangles, 1.86348664915158);
        dawt = hm.get("n1").unwrap();
        assert_eq!(dawt.degree, 1);
        assert_eq!(dawt.weighted_triangles, 0.0);
        dawt = hm.get("n2").unwrap();
        assert_eq!(dawt.degree, 2);
        assert_eq!(dawt.weighted_triangles, 1.86348664915158);
        dawt = hm.get("n3").unwrap();
        assert_eq!(dawt.degree, 2);
        assert_eq!(dawt.weighted_triangles, 1.86348664915158);
    }
}
