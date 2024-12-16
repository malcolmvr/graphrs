use super::Graph;
use crate::{Error, ErrorKind};
use nalgebra::DMatrix;
use ndarray::Array2;
use sprs::{CsMat, TriMat};
use std::fmt::Display;
use std::hash::Hash;

impl<T, A> Graph<T, A>
where
    T: Eq + Clone + PartialOrd + Ord + Hash + Send + Sync + Display,
    A: Clone,
{
    /**
    Gets the (adjacency matrix)[https://en.wikipedia.org/wiki/Adjacency_matrix] of the graph
    as a 1D vector (Vec<f64>).

    This is an optional feature for crate. Enable in Cargo.toml with:
    `graphrs = { version = "x.y.z", features = ["adjacency_matrix"] }`

    # Examples

    ```
    use graphrs::generators;
    let graph = generators::social::karate_club_graph();
    let matrix = graph.get_adjacency_matrix_vec(true).unwrap();
    ```
    */
    pub fn get_adjacency_matrix_vec(&self, weighted: bool) -> Result<Vec<f64>, Error>
    where
        T: Hash + Eq + Clone + Ord + Display + Send + Sync,
        A: Clone,
    {
        if self.specs.multi_edges {
            return Err(Error {
                kind: ErrorKind::WrongMethod,
                message: "The `get_adjaceny_matrix_vec_vec` method cannot be used on multi-edge graphs. \
                Consider using the `to_single_edges` method to convert the graph to a single-edge graph."
                    .to_string(),
            });
        }
        let vec_vec = self.get_adjacency_matrix_vec_vec(weighted)?;
        Ok(vec_vec.into_iter().flatten().collect())
    }

    /**
    Gets the (adjacency matrix)[https://en.wikipedia.org/wiki/Adjacency_matrix] of the graph
    as a 2D vector (Vec<Vec<f64>>).

    This is an optional feature for crate. Enable in Cargo.toml with:
    `graphrs = { version = "x.y.z", features = ["adjacency_matrix"] }`

    # Examples

    ```
    use graphrs::generators;
    let graph = generators::social::karate_club_graph();
    let matrix = graph.get_adjacency_matrix_vec_vec(true).unwrap();
    ```
    */
    pub fn get_adjacency_matrix_vec_vec(&self, weighted: bool) -> Result<Vec<Vec<f64>>, Error>
    where
        T: Hash + Eq + Clone + Ord + Display + Send + Sync,
        A: Clone,
    {
        if self.specs.multi_edges {
            return Err(Error {
                kind: ErrorKind::WrongMethod,
                message: "The `get_adjaceny_matrix_vec_vec` method cannot be used on multi-edge graphs. \
                Consider using the `to_single_edges` method to convert the graph to a single-edge graph."
                    .to_string(),
            });
        }
        let num_nodes = self.number_of_nodes();
        let mut matrix = vec![vec![0.0; num_nodes]; num_nodes];
        for u in 0..num_nodes {
            for adj in self.get_successor_nodes_by_index(&u) {
                let v = adj.node_index;
                let weight = match weighted {
                    true => adj.weight,
                    false => 1.0,
                };
                matrix[u][v] = weight;
                if !self.specs.directed {
                    matrix[v][u] = weight;
                }
            }
        }
        Ok(matrix)
    }

    /**
    Gets the (adjacency matrix)[https://en.wikipedia.org/wiki/Adjacency_matrix] of the graph
    as an `nalgebra` `DMatrix<f64>`.

    This is an optional feature for crate. Enable in Cargo.toml with:
    `graphrs = { version = "x.y.z", features = ["adjacency_matrix"] }`

    # Examples

    ```
    use graphrs::generators;
    let graph = generators::social::karate_club_graph();
    let matrix = graph.get_adjacency_matrix_nalgebra(true).unwrap();
    ```
    */
    pub fn get_adjacency_matrix_nalgebra(&self, weighted: bool) -> Result<DMatrix<f64>, Error>
    where
        T: Hash + Eq + Clone + Ord + Display + Send + Sync,
        A: Clone,
    {
        if self.specs.multi_edges {
            return Err(Error {
                kind: ErrorKind::WrongMethod,
                message: "The `get_adjaceny_matrix_nalgebra` method cannot be used on multi-edge graphs. \
                Consider using the `to_single_edges` method to convert the graph to a single-edge graph."
                    .to_string(),
            });
        }
        let num_nodes = self.number_of_nodes();
        let mut matrix = DMatrix::from_element(num_nodes, num_nodes, 0.0);
        for u in 0..num_nodes {
            for adj in self.get_successor_nodes_by_index(&u) {
                let v = adj.node_index;
                let weight = match weighted {
                    true => adj.weight,
                    false => 1.0,
                };
                matrix[(u, v)] = weight;
                if !self.specs.directed {
                    matrix[(v, u)] = weight;
                }
            }
        }
        Ok(matrix)
    }

    /**
    Gets the (adjacency matrix)[https://en.wikipedia.org/wiki/Adjacency_matrix] of the graph
    as an `ndarray` `Array2<f64>`.

    This is an optional feature for crate. Enable in Cargo.toml with:
    `graphrs = { version = "x.y.z", features = ["adjacency_matrix"] }`

    # Examples

    ```
    use graphrs::generators;
    let graph = generators::social::karate_club_graph();
    let matrix = graph.get_adjacency_matrix_ndarray(true).unwrap();
    ```
    */
    pub fn get_adjacency_matrix_ndarray(&self, weighted: bool) -> Result<Array2<f64>, Error>
    where
        T: Hash + Eq + Clone + Ord + Display + Send + Sync,
        A: Clone,
    {
        if self.specs.multi_edges {
            return Err(Error {
                kind: ErrorKind::WrongMethod,
                message: "The `get_adjaceny_matrix_nalgebra` method cannot be used on multi-edge graphs. \
                Consider using the `to_single_edges` method to convert the graph to a single-edge graph."
                    .to_string(),
            });
        }
        let num_nodes = self.number_of_nodes();
        let mut matrix = Array2::<f64>::zeros((num_nodes, num_nodes));
        for u in 0..num_nodes {
            for adj in self.get_successor_nodes_by_index(&u) {
                let v = adj.node_index;
                let weight = match weighted {
                    true => adj.weight,
                    false => 1.0,
                };
                matrix[(u, v)] = weight;
                if !self.specs.directed {
                    matrix[(v, u)] = weight;
                }
            }
        }
        Ok(matrix)
    }

    /**
    Gets the (adjacency matrix)[https://en.wikipedia.org/wiki/Adjacency_matrix] of the graph.

    For large graphs, the adjacency matrix can be very large so this method returns a sparse matrix
    using the "sprs" crate.false

    ```
    use graphrs::generators;
    let graph = generators::social::karate_club_graph();
    let matrix = graph.get_adjacency_matrix_sprs(true).unwrap();
    ```
    */
    pub fn get_adjacency_matrix_sprs(&self, weighted: bool) -> Result<CsMat<f64>, Error>
    where
        T: Hash + Eq + Clone + Ord + Display + Send + Sync,
        A: Clone,
    {
        if self.specs.multi_edges {
            return Err(Error {
                kind: ErrorKind::WrongMethod,
                message:
                    "The `get_sparse_adjaceny_matrix` method cannot be used on multi-edge graphs."
                        .to_string(),
            });
        }
        let num_nodes = self.number_of_nodes();
        let num_edges = self.get_all_edges().len();
        let mut row_inds: Vec<usize> = vec![0; num_edges];
        let mut col_inds: Vec<usize> = vec![0; num_edges];
        let mut data = vec![0.0; num_edges];
        let mut count = 0;
        for (u, hm) in self.edges_map.iter() {
            for (v, edges) in hm.iter() {
                row_inds[count] = *u;
                col_inds[count] = *v;
                data[count] = match weighted {
                    true => edges[0].weight,
                    false => 1.0,
                };
                count += 1;
            }
        }
        let tri_matrix = TriMat::from_triplets((num_nodes, num_nodes), row_inds, col_inds, data);
        let matrix = tri_matrix.to_csr::<usize>();
        Ok(matrix)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::{Edge, GraphSpecs};

    #[test]
    fn test_get_matrix_vec_weighted() {
        let graph = get_graph_1(true);
        let matrix = graph.get_adjacency_matrix_vec(true).unwrap();
        assert_eq!(matrix[0], 0.0);
        assert_eq!(matrix[1], 1.0);
        assert_eq!(matrix[2], 0.0);
        assert_eq!(matrix[3], 2.0);
        assert_eq!(matrix[4], 9.0);
        assert_eq!(matrix[5], 0.0);
        assert_eq!(matrix[6], 0.0);
        assert_eq!(matrix[7], 5.0);
        assert_eq!(matrix[8], 0.0);
        assert_eq!(matrix[9], 0.0);
        assert_eq!(matrix[10], 0.0);
        assert_eq!(matrix[11], 0.0);
        assert_eq!(matrix[12], 0.0);
        assert_eq!(matrix[13], 0.0);
        assert_eq!(matrix[14], 1.0);
        assert_eq!(matrix[15], 0.0);
        assert_eq!(matrix[16], 0.0);
        assert_eq!(matrix[17], 3.0);
        assert_eq!(matrix[18], 0.0);
        assert_eq!(matrix[19], 0.0);
        assert_eq!(matrix[20], 0.0);
        assert_eq!(matrix[21], 0.0);
        assert_eq!(matrix[22], 0.0);
        assert_eq!(matrix[23], 0.0);
        assert_eq!(matrix[24], 0.0);
    }

    #[test]
    fn test_get_matrix_vec_unweighted() {
        let graph = get_graph_1(true);
        let matrix = graph.get_adjacency_matrix_vec(false).unwrap();
        assert_eq!(matrix[0], 0.0);
        assert_eq!(matrix[1], 1.0);
        assert_eq!(matrix[2], 0.0);
        assert_eq!(matrix[3], 1.0);
        assert_eq!(matrix[4], 1.0);
        assert_eq!(matrix[5], 0.0);
        assert_eq!(matrix[6], 0.0);
        assert_eq!(matrix[7], 1.0);
        assert_eq!(matrix[8], 0.0);
        assert_eq!(matrix[9], 0.0);
        assert_eq!(matrix[10], 0.0);
        assert_eq!(matrix[11], 0.0);
        assert_eq!(matrix[12], 0.0);
        assert_eq!(matrix[13], 0.0);
        assert_eq!(matrix[14], 1.0);
        assert_eq!(matrix[15], 0.0);
        assert_eq!(matrix[16], 0.0);
        assert_eq!(matrix[17], 1.0);
        assert_eq!(matrix[18], 0.0);
        assert_eq!(matrix[19], 0.0);
        assert_eq!(matrix[20], 0.0);
        assert_eq!(matrix[21], 0.0);
        assert_eq!(matrix[22], 0.0);
        assert_eq!(matrix[23], 0.0);
        assert_eq!(matrix[24], 0.0);
    }

    #[test]
    fn test_get_matrix_vec_vec_weighted() {
        let graph = get_graph_1(true);
        let matrix = graph.get_adjacency_matrix_vec_vec(true).unwrap();
        assert_eq!(matrix[0][0], 0.0);
        assert_eq!(matrix[0][1], 1.0);
        assert_eq!(matrix[0][2], 0.0);
        assert_eq!(matrix[0][3], 2.0);
        assert_eq!(matrix[0][4], 9.0);
        assert_eq!(matrix[1][0], 0.0);
        assert_eq!(matrix[1][1], 0.0);
        assert_eq!(matrix[1][2], 5.0);
        assert_eq!(matrix[1][3], 0.0);
        assert_eq!(matrix[1][4], 0.0);
        assert_eq!(matrix[2][0], 0.0);
        assert_eq!(matrix[2][1], 0.0);
        assert_eq!(matrix[2][2], 0.0);
        assert_eq!(matrix[2][3], 0.0);
        assert_eq!(matrix[2][4], 1.0);
        assert_eq!(matrix[3][0], 0.0);
        assert_eq!(matrix[3][1], 0.0);
        assert_eq!(matrix[3][2], 3.0);
        assert_eq!(matrix[3][3], 0.0);
        assert_eq!(matrix[3][4], 0.0);
        assert_eq!(matrix[4][0], 0.0);
        assert_eq!(matrix[4][1], 0.0);
        assert_eq!(matrix[4][2], 0.0);
        assert_eq!(matrix[4][3], 0.0);
        assert_eq!(matrix[4][4], 0.0);
    }

    #[test]
    fn test_get_matrix_vec_vec_unweighted() {
        let graph = get_graph_1(true);
        let matrix = graph.get_adjacency_matrix_vec_vec(false).unwrap();
        assert_eq!(matrix[0][0], 0.0);
        assert_eq!(matrix[0][1], 1.0);
        assert_eq!(matrix[0][2], 0.0);
        assert_eq!(matrix[0][3], 1.0);
        assert_eq!(matrix[0][4], 1.0);
        assert_eq!(matrix[1][0], 0.0);
        assert_eq!(matrix[1][1], 0.0);
        assert_eq!(matrix[1][2], 1.0);
        assert_eq!(matrix[1][3], 0.0);
        assert_eq!(matrix[1][4], 0.0);
        assert_eq!(matrix[2][0], 0.0);
        assert_eq!(matrix[2][1], 0.0);
        assert_eq!(matrix[2][2], 0.0);
        assert_eq!(matrix[2][3], 0.0);
        assert_eq!(matrix[2][4], 1.0);
        assert_eq!(matrix[3][0], 0.0);
        assert_eq!(matrix[3][1], 0.0);
        assert_eq!(matrix[3][2], 1.0);
        assert_eq!(matrix[3][3], 0.0);
        assert_eq!(matrix[3][4], 0.0);
        assert_eq!(matrix[4][0], 0.0);
        assert_eq!(matrix[4][1], 0.0);
        assert_eq!(matrix[4][2], 0.0);
        assert_eq!(matrix[4][3], 0.0);
        assert_eq!(matrix[4][4], 0.0);
    }

    #[test]
    fn test_get_matrix_nalgebra_weighted() {
        let graph = get_graph_1(true);
        let matrix = graph.get_adjacency_matrix_nalgebra(true).unwrap();
        assert_eq!(matrix[(0, 0)], 0.0);
        assert_eq!(matrix[(0, 1)], 1.0);
        assert_eq!(matrix[(0, 2)], 0.0);
        assert_eq!(matrix[(0, 3)], 2.0);
        assert_eq!(matrix[(0, 4)], 9.0);
        assert_eq!(matrix[(1, 0)], 0.0);
        assert_eq!(matrix[(1, 1)], 0.0);
        assert_eq!(matrix[(1, 2)], 5.0);
        assert_eq!(matrix[(1, 3)], 0.0);
        assert_eq!(matrix[(1, 4)], 0.0);
        assert_eq!(matrix[(2, 0)], 0.0);
        assert_eq!(matrix[(2, 1)], 0.0);
        assert_eq!(matrix[(2, 2)], 0.0);
        assert_eq!(matrix[(2, 3)], 0.0);
        assert_eq!(matrix[(2, 4)], 1.0);
        assert_eq!(matrix[(3, 0)], 0.0);
        assert_eq!(matrix[(3, 1)], 0.0);
        assert_eq!(matrix[(3, 2)], 3.0);
        assert_eq!(matrix[(3, 3)], 0.0);
        assert_eq!(matrix[(3, 4)], 0.0);
        assert_eq!(matrix[(4, 0)], 0.0);
        assert_eq!(matrix[(4, 1)], 0.0);
        assert_eq!(matrix[(4, 2)], 0.0);
        assert_eq!(matrix[(4, 3)], 0.0);
        assert_eq!(matrix[(4, 4)], 0.0);
    }

    #[test]
    fn test_get_matrix_nalgebra_unweighted() {
        let graph = get_graph_1(true);
        let matrix = graph.get_adjacency_matrix_nalgebra(false).unwrap();
        assert_eq!(matrix[(0, 0)], 0.0);
        assert_eq!(matrix[(0, 1)], 1.0);
        assert_eq!(matrix[(0, 2)], 0.0);
        assert_eq!(matrix[(0, 3)], 1.0);
        assert_eq!(matrix[(0, 4)], 1.0);
        assert_eq!(matrix[(1, 0)], 0.0);
        assert_eq!(matrix[(1, 1)], 0.0);
        assert_eq!(matrix[(1, 2)], 1.0);
        assert_eq!(matrix[(1, 3)], 0.0);
        assert_eq!(matrix[(1, 4)], 0.0);
        assert_eq!(matrix[(2, 0)], 0.0);
        assert_eq!(matrix[(2, 1)], 0.0);
        assert_eq!(matrix[(2, 2)], 0.0);
        assert_eq!(matrix[(2, 3)], 0.0);
        assert_eq!(matrix[(2, 4)], 1.0);
        assert_eq!(matrix[(3, 0)], 0.0);
        assert_eq!(matrix[(3, 1)], 0.0);
        assert_eq!(matrix[(3, 2)], 1.0);
        assert_eq!(matrix[(3, 3)], 0.0);
        assert_eq!(matrix[(3, 4)], 0.0);
        assert_eq!(matrix[(4, 0)], 0.0);
        assert_eq!(matrix[(4, 1)], 0.0);
        assert_eq!(matrix[(4, 2)], 0.0);
        assert_eq!(matrix[(4, 3)], 0.0);
        assert_eq!(matrix[(4, 4)], 0.0);
    }

    #[test]
    fn test_get_matrix_ndarray_weighted() {
        let graph = get_graph_1(true);
        let matrix = graph.get_adjacency_matrix_ndarray(true).unwrap();
        assert_eq!(matrix[[0, 0]], 0.0);
        assert_eq!(matrix[[0, 1]], 1.0);
        assert_eq!(matrix[[0, 2]], 0.0);
        assert_eq!(matrix[[0, 3]], 2.0);
        assert_eq!(matrix[[0, 4]], 9.0);
        assert_eq!(matrix[[1, 0]], 0.0);
        assert_eq!(matrix[[1, 1]], 0.0);
        assert_eq!(matrix[[1, 2]], 5.0);
        assert_eq!(matrix[[1, 3]], 0.0);
        assert_eq!(matrix[[1, 4]], 0.0);
        assert_eq!(matrix[[2, 0]], 0.0);
        assert_eq!(matrix[[2, 1]], 0.0);
        assert_eq!(matrix[[2, 2]], 0.0);
        assert_eq!(matrix[[2, 3]], 0.0);
        assert_eq!(matrix[[2, 4]], 1.0);
        assert_eq!(matrix[[3, 0]], 0.0);
        assert_eq!(matrix[[3, 1]], 0.0);
        assert_eq!(matrix[[3, 2]], 3.0);
        assert_eq!(matrix[[3, 3]], 0.0);
        assert_eq!(matrix[[3, 4]], 0.0);
        assert_eq!(matrix[[4, 0]], 0.0);
        assert_eq!(matrix[[4, 1]], 0.0);
        assert_eq!(matrix[[4, 2]], 0.0);
        assert_eq!(matrix[[4, 3]], 0.0);
        assert_eq!(matrix[[4, 4]], 0.0);
    }

    #[test]
    fn test_get_matrix_ndarray_unweighted() {
        let graph = get_graph_1(true);
        let matrix = graph.get_adjacency_matrix_ndarray(false).unwrap();
        assert_eq!(matrix[[0, 0]], 0.0);
        assert_eq!(matrix[[0, 1]], 1.0);
        assert_eq!(matrix[[0, 2]], 0.0);
        assert_eq!(matrix[[0, 3]], 1.0);
        assert_eq!(matrix[[0, 4]], 1.0);
        assert_eq!(matrix[[1, 0]], 0.0);
        assert_eq!(matrix[[1, 1]], 0.0);
        assert_eq!(matrix[[1, 2]], 1.0);
        assert_eq!(matrix[[1, 3]], 0.0);
        assert_eq!(matrix[[1, 4]], 0.0);
        assert_eq!(matrix[[2, 0]], 0.0);
        assert_eq!(matrix[[2, 1]], 0.0);
        assert_eq!(matrix[[2, 2]], 0.0);
        assert_eq!(matrix[[2, 3]], 0.0);
        assert_eq!(matrix[[2, 4]], 1.0);
        assert_eq!(matrix[[3, 0]], 0.0);
        assert_eq!(matrix[[3, 1]], 0.0);
        assert_eq!(matrix[[3, 2]], 1.0);
        assert_eq!(matrix[[3, 3]], 0.0);
        assert_eq!(matrix[[3, 4]], 0.0);
        assert_eq!(matrix[[4, 0]], 0.0);
        assert_eq!(matrix[[4, 1]], 0.0);
        assert_eq!(matrix[[4, 2]], 0.0);
        assert_eq!(matrix[[4, 3]], 0.0);
        assert_eq!(matrix[[4, 4]], 0.0);
    }

    #[test]
    fn test_get_matrix_sparse_weighted() {
        let graph = get_graph_1(true);
        let matrix = graph.get_adjacency_matrix_sprs(true).unwrap();
        assert!(matrix.get(0, 0).is_none());
        assert_eq!(matrix.get(0, 1).unwrap(), &1.0);
        assert!(matrix.get(0, 2).is_none());
        assert_eq!(matrix.get(0, 3).unwrap(), &2.0);
        assert_eq!(matrix.get(0, 4).unwrap(), &9.0);
        assert!(matrix.get(1, 0).is_none());
        assert!(matrix.get(1, 1).is_none());
        assert_eq!(matrix.get(1, 2).unwrap(), &5.0);
        assert!(matrix.get(1, 3).is_none());
        assert!(matrix.get(1, 4).is_none());
        assert!(matrix.get(2, 0).is_none());
        assert!(matrix.get(2, 1).is_none());
        assert!(matrix.get(2, 2).is_none());
        assert!(matrix.get(2, 3).is_none());
        assert_eq!(matrix.get(2, 4).unwrap(), &1.0);
        assert!(matrix.get(3, 0).is_none());
        assert!(matrix.get(3, 1).is_none());
        assert_eq!(matrix.get(3, 2).unwrap(), &3.0);
        assert!(matrix.get(3, 3).is_none());
        assert!(matrix.get(3, 4).is_none());
        assert!(matrix.get(4, 0).is_none());
        assert!(matrix.get(4, 1).is_none());
        assert!(matrix.get(4, 2).is_none());
        assert!(matrix.get(4, 3).is_none());
        assert!(matrix.get(4, 4).is_none());
    }

    #[test]
    fn test_get_matrix_sparse_unweighted() {
        let graph = get_graph_1(true);
        let matrix = graph.get_adjacency_matrix_sprs(false).unwrap();
        assert!(matrix.get(0, 0).is_none());
        assert_eq!(matrix.get(0, 1).unwrap(), &1.0);
        assert!(matrix.get(0, 2).is_none());
        assert_eq!(matrix.get(0, 3).unwrap(), &1.0);
        assert_eq!(matrix.get(0, 4).unwrap(), &1.0);
        assert!(matrix.get(1, 0).is_none());
        assert!(matrix.get(1, 1).is_none());
        assert_eq!(matrix.get(1, 2).unwrap(), &1.0);
        assert!(matrix.get(1, 3).is_none());
        assert!(matrix.get(1, 4).is_none());
        assert!(matrix.get(2, 0).is_none());
        assert!(matrix.get(2, 1).is_none());
        assert!(matrix.get(2, 2).is_none());
        assert!(matrix.get(2, 3).is_none());
        assert_eq!(matrix.get(2, 4).unwrap(), &1.0);
        assert!(matrix.get(3, 0).is_none());
        assert!(matrix.get(3, 1).is_none());
        assert_eq!(matrix.get(3, 2).unwrap(), &1.0);
        assert!(matrix.get(3, 3).is_none());
        assert!(matrix.get(3, 4).is_none());
        assert!(matrix.get(4, 0).is_none());
        assert!(matrix.get(4, 1).is_none());
        assert!(matrix.get(4, 2).is_none());
        assert!(matrix.get(4, 3).is_none());
        assert!(matrix.get(4, 4).is_none());
    }

    fn get_graph_1<'a>(directed: bool) -> Graph<&'a str, ()> {
        let edges = vec![
            Edge::with_weight("n1", "n2", 1.0),
            Edge::with_weight("n2", "n3", 5.0),
            Edge::with_weight("n1", "n4", 2.0),
            Edge::with_weight("n4", "n3", 3.0),
            Edge::with_weight("n1", "n5", 9.0),
            Edge::with_weight("n3", "n5", 1.0),
        ];
        let graph: Graph<&str, ()> = Graph::new_from_nodes_and_edges(
            vec![],
            edges,
            GraphSpecs {
                directed,
                ..GraphSpecs::directed_create_missing()
            },
        )
        .unwrap();
        graph
    }
}
