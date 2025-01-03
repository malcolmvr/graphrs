use crate::Graph;
use rand::prelude::*;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fmt::Display;
use std::hash::Hash;

/// Returns all the node indexes in `graph`, shuffled randomly.
pub(crate) fn get_shuffled_node_indexes<T, A>(graph: &Graph<T, A>, seed: Option<u64>) -> Vec<usize>
where
    T: Hash + Eq + Clone + Ord + Display + Send + Sync,
    A: Clone + Send + Sync,
{
    let mut rng = get_rng(seed);
    let mut indexes: Vec<usize> = (0..graph.number_of_nodes()).collect();
    indexes.shuffle(&mut rng);
    indexes
}

/// Returns a random number generator (RNG), optionally seeded.
fn get_rng(seed: Option<u64>) -> StdRng {
    match seed {
        None => {
            let mut trng = thread_rng();
            StdRng::seed_from_u64(trng.next_u64())
        }
        Some(s) => StdRng::seed_from_u64(s),
    }
}
