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
        node_vec.push(("n1", node1attr));
        node_vec.push(("n2", node2attr));

        let mut edges_vec = Vec::new();
        edges_vec.push(("n1", "n2"));
        edges_vec.push(("n2", "n1"));

        let graph = DiGraph::new(node_vec, edges_vec);

        assert_eq!(graph.nodes.len(), 2);
        assert_eq!(graph.edges.len(), 2);

    }
    
}