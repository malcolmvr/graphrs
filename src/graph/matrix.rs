use super::Graph;
use crate::{Error, ErrorKind};
use sprs::TriMat;
use std::fmt::Display;
use std::hash::Hash;

impl<T, A> Graph<T, A>
where
    T: Eq + Clone + PartialOrd + Ord + Hash + Send + Sync + Display,
    A: Clone,
{
    /**
    TODO: Add documentation
    */
    pub fn generate_sparse_adjacency_matrix(&mut self) -> Result<(), Error>
    where
        T: Hash + Eq + Clone + Ord + Display + Send + Sync,
        A: Clone,
    {
        if self.specs.multi_edges {
            return Err(Error {
                kind: ErrorKind::WrongMethod,
                message: "The `generate_sparse_adjaceny_matrix` method cannot be used on multi-edge graphs."
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
                data[count] = edges[0].weight;
                count += 1;
            }
        }
        let tri_matrix = TriMat::from_triplets((num_nodes, num_nodes), row_inds, col_inds, data);
        let matrix = tri_matrix.to_csr::<usize>();
        self.adjacency_matrix = Some(matrix);
        Ok(())
    }
}
