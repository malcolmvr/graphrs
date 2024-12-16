extern crate lapack_sys as lapack;

use crate::{Error, ErrorKind, Graph};
use lapack::dsyev_;
use std::fmt::Display;
use std::hash::Hash;
use std::os::raw::c_char;

/**
Compute the spectral gap of a graph.

The spectral gap is the difference between the largest and second largest eigenvalues of the
adjacency matrix of a graph. The larger the spectral gap, the more robust the graph.

# Arguments
* `graph` - A reference to a `Graph` object.
* 'weighted' - A boolean flag to indicate if the edge weights should be used.

# Raises
* If the Lapack library fails to compute the eigenvalues of the Laplacian matrix.

# Returns
* The spectral gap of the graph.

# Examples

```
use graphrs::{algorithms::{resiliency::{spectral_gap}}, generators};
let graph = generators::social::karate_club_graph();
let sg = spectral_gap::spectral_gap(&graph, true).unwrap();
```
*/
pub fn spectral_gap<T, A>(graph: &Graph<T, A>, weighted: bool) -> Result<f64, Error>
where
    T: Hash + Eq + Clone + Ord + Display + Send + Sync,
    A: Clone,
{
    let num_nodes = graph.number_of_nodes() as i32;
    let mut a = graph.get_adjacency_matrix_vec(weighted)?;
    let jobz = b'N' as c_char;
    let uplo = b'U' as c_char;
    let mut w = vec![0.0; num_nodes as usize];
    let mut info = 0;

    unsafe {
        let mut lwork = -1;
        let mut work = vec![0.0; 1];
        // the first call to dsyev_ is to get the optimal work size (lwork)
        // https://stackoverflow.com/questions/46618391/what-is-the-use-of-the-work-parameters-in-lapack-routines
        dsyev_(
            &jobz,
            &uplo,
            &num_nodes,
            a.as_mut_ptr(),
            &num_nodes,
            w.as_mut_ptr(),
            work.as_mut_ptr(),
            &lwork,
            &mut info,
        );
        lwork = work[0] as i32;
        work = vec![0.0; lwork as usize];
        // the second call to dsyev_ is to compute the eigenvalues
        dsyev_(
            &jobz,
            &uplo,
            &num_nodes,
            a.as_mut_ptr(),
            &num_nodes,
            w.as_mut_ptr(),
            work.as_mut_ptr(),
            &lwork,
            &mut info,
        );
    }

    if info != 0 {
        return Err(Error {
            kind: ErrorKind::LaPackError,
            message: "lapack failed to compute the eigenvalues.".to_string(),
        });
    }
    w.sort_by(|a, b| b.partial_cmp(a).unwrap());

    Ok(w[0] - w[1])
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::generators;
    use assert_approx_eq::assert_approx_eq;
    use rayon::prelude::{IntoParallelIterator, ParallelIterator};

    #[test]
    fn test_spectral_gap_1() {
        let graph = generators::social::karate_club_graph();
        let sg = spectral_gap(&graph, true).unwrap();
        assert_approx_eq!(sg, 4.727593829875396);
    }

    #[test]
    fn test_spectral_gap_parallel() {
        // some reports that LAPACK functions might not be thread safe
        // https://stackoverflow.com/questions/18216314/shouldnt-lapacks-dsyevr-function-for-eigenvalues-and-eigenvectors-be-thread-s
        // let test that!
        let graph = generators::social::karate_club_graph();
        let v = vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
        ];
        v.into_par_iter().for_each(|_i| {
            let sg = spectral_gap(&graph, true).unwrap();
            assert_approx_eq!(sg, 4.727593829875396);
        });
    }
}
