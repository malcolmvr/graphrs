/**
Functions to generate graphs.
**/
pub mod generators {
    use crate::{Edge, Graph, GraphSpecs, MissingNodeStrategy};
    use itertools::Itertools;
    
    /**
    Generates a "complete" graph: one where every node is connected to every other node.

    ```
    use graphrs::{generators};
    let graph = generators::complete_graph(5, true);
    ```
    **/
    pub fn complete_graph<'a>(num_nodes: i32, directed: bool) -> Graph<i32, &'a str, i32> {
        let x = match directed {
            false => (0..num_nodes).combinations(2).collect::<Vec<Vec<i32>>>(),
            true => (0..num_nodes).permutations(2).collect::<Vec<Vec<i32>>>(),
        };
        let nodes = vec![];
        let edges = x
            .into_iter()
            .map(|x| Edge::new(x[0], x[1]))
            .collect::<Vec<Edge<i32, &str, i32>>>();
        let specs = match directed {
            false => GraphSpecs {
                missing_node_strategy: MissingNodeStrategy::Create,
                ..GraphSpecs::undirected()
            },
            true => GraphSpecs {
                missing_node_strategy: MissingNodeStrategy::Create,
                ..GraphSpecs::directed()
            },
        };
        Graph::new_from_nodes_and_edges(nodes, edges, specs).unwrap()
    }

}