#[cfg(test)]
mod tests {

    use std::collections::HashMap;
    use graphx::{DiGraph};
    
    #[test]
    fn test_digraph_new() {

        let mut node1attr = HashMap::new();
        node1attr.insert("weight", &1.0);

        let mut node2attr = HashMap::new();
        node2attr.insert("weight", &2.0);

        let mut node_vec = Vec::new();
        node_vec.push(("n1", Some(node1attr)));
        node_vec.push(("n2", Some(node2attr)));
        node_vec.push(("n3", None));

        let mut edges_vec = Vec::new();
        edges_vec.push(("n1", "n2"));
        edges_vec.push(("n2", "n1"));
        edges_vec.push(("n1", "n3"));

        let graph = DiGraph::new(node_vec, edges_vec);

        assert_eq!(graph.nodes().len(), 3);
        assert_eq!(graph.edges().len(), 3);

    }
    
}