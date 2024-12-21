use crate::{algorithms, Error, Graph};
use std::fmt::{Debug, Display};
use std::hash::Hash;

/**
Compute the average node betweenness of a graph.

The average vertex betweenness of a graph is the summation of node betweenness
for every node in the graph. The smaller the average vertex betweenness, the more
robust the graph. We can view this as the load of the graph being better
distributed and less dependent on a few nodes.

# Arguments
* `graph` - A reference to a `Graph` object.
* 'weighted' - A boolean flag to indicate if the edge weights should be used.

# Returns
* The average node betweenness of the graph.

# Examples

```
use graphrs::{algorithms::{resiliency::{average_node_betweenness}}, generators};
let graph = generators::social::karate_club_graph();
let avg: f64 = average_node_betweenness::average_node_betweenness(&graph, true).unwrap();
```
*/
pub fn average_node_betweenness<T, A>(graph: &Graph<T, A>, weighted: bool) -> Result<f64, Error>
where
    T: Hash + Eq + Clone + Ord + Debug + Display + Send + Sync,
    A: Clone + Send + Sync,
{
    let centralities =
        algorithms::centrality::betweenness::betweenness_centrality(graph, weighted, false)?;
    Ok(centralities.iter().map(|(_, v)| v).sum::<f64>() / graph.number_of_nodes() as f64)
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::generators;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn test_average_node_betweenness_1() {
        let graph = generators::social::karate_club_graph();
        let sg = average_node_betweenness(&graph, true).unwrap();
        assert_approx_eq!(sg, 26.19362745098039);
    }

    #[test]
    fn test_average_node_betweenness_2() {
        let graph = generators::social::karate_club_graph();
        let sg = average_node_betweenness(&graph, false).unwrap();
        assert_approx_eq!(sg, 23.23529411764705);
    }
}
