use super::Graph;
use std::fmt::Display;
use std::hash::Hash;

impl<T, A> Graph<T, A>
where
    T: Eq + Clone + PartialOrd + Ord + Hash + Send + Sync + Display,
    A: Clone,
{
    /**
    Returns the density of a graph.

    # Examples

    ```
    use graphrs::generators;
    let graph = generators::social::karate_club_graph();
    assert_eq!(graph.get_density(), 0.13903743315508021);
    ```
    */
    pub fn get_density(&self) -> f64 {
        if self.edges.is_empty() {
            return 0.0;
        }
        let m = self.edges.len() as f64;
        let n = self.nodes.len() as f64;
        match self.specs.directed {
            false => (2.0 * m) / (n * (n - 1.0)),
            true => m / (n * (n - 1.0)),
        }
    }
}
