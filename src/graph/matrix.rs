use super::Graph;
use crate::{Error, ErrorKind};
use std::fmt::Display;
use std::hash::Hash;

impl<T, A> Graph<T, A>
where
    T: Eq + Clone + PartialOrd + Ord + Hash + Send + Sync + Display,
    A: Clone,
{
    /**
    Gets the (adjacency matrix)[https://en.wikipedia.org/wiki/Adjacency_matrix] of the graph.

    For large graphs, the adjacency matrix can be very large so the `get_sparse_adjacency_matrix`
    function should be used instead.

    # Examples

    ```
    use graphrs::generators;
    let graph = generators::social::karate_club_graph();
    let matrix = graph.get_adjacency_matrix().unwrap();
    ```
    */
    pub fn get_adjacency_matrix(&self) -> Result<Vec<Vec<f64>>, Error>
    where
        T: Hash + Eq + Clone + Ord + Display + Send + Sync,
        A: Clone,
    {
        if self.specs.multi_edges {
            return Err(Error {
                kind: ErrorKind::WrongMethod,
                message: "The `get_adjaceny_matrix` method cannot be used on multi-edge graphs. \
                Consider using the `to_single_edges` method to convert the graph to a single-edge graph."
                    .to_string(),
            });
        }
        let num_nodes = self.number_of_nodes();
        let mut matrix = vec![vec![0.0; num_nodes]; num_nodes];
        for u in 0..num_nodes {
            for adj in self.get_successor_nodes_by_index(&u) {
                let v = adj.node_index;
                matrix[u][v] = adj.weight;
                if !self.specs.directed {
                    matrix[v][u] = adj.weight;
                }
            }
        }
        Ok(matrix)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::{Edge, GraphSpecs};

    #[test]
    fn test_get_matrix_1() {
        let graph = get_graph_1(true);
        let matrix = graph.get_adjacency_matrix().unwrap();
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
