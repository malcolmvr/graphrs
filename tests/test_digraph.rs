#[cfg(test)]
mod tests {

    use std::collections::HashMap;
    use graphx::{DiGraph, Node, Edge};
    

    #[test]
    fn test_digraph_new_from_tuples() {

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

        let graph = DiGraph::new_from_tuples(node_vec, edges_vec);

        assert_eq!(graph.nodes().len(), 3);
        assert_eq!(graph.edges().len(), 3);

    }


    #[test]
    fn test_digraph_new_from_nodes_and_edges() {

        let mut nodes = Vec::new();
        nodes.push(Node::from_name_and_weight("n1", "weight", &1.0));
        nodes.push(Node::from_name_and_weight("n2", "weight", &2.0));
        nodes.push(Node::<&str, &str, &f64>::from_name("n3"));
        
        let mut edges = Vec::new();
        edges.push(Edge { u: "n1", v: "n2" });
        edges.push(Edge { u: "n2", v: "n1" });
        edges.push(Edge { u: "n1", v: "n3" });

        let graph = DiGraph::new_from_nodes_and_edges(nodes, edges);

        assert_eq!(graph.nodes().len(), 3);
        assert_eq!(graph.edges().len(), 3);

    }


    #[test]
    fn test_digraph_add_nodes() {

        let graph = get_basic_graph();

        let mut nodes = Vec::new();
        nodes.push(Node::from_name_and_weight("n10", "weight", &10.0));
        nodes.push(Node::from_name_and_weight("n11", "weight", &11.0));

        let graph = graph.add_nodes(nodes);

        assert_eq!(graph.nodes().len(), 5);

    }


    #[test]
    fn test_digraph_add_nodes_from_tuples() {

        let graph = get_basic_graph();

        let mut node10attr = HashMap::new();
        node10attr.insert("weight", &10.0);

        let mut node11attr = HashMap::new();
        node11attr.insert("weight", &11.0);

        let mut node_vec = Vec::new();
        node_vec.push(("n10", Some(node10attr)));
        node_vec.push(("n11", Some(node11attr)));

        let graph = graph.add_nodes_from_tuples(node_vec);

        assert_eq!(graph.nodes().len(), 5);

    }


    fn get_basic_graph<'a>() -> DiGraph<&'a str, &'a str, &'a f64> {

        let mut nodes = Vec::new();
        nodes.push(Node::from_name_and_weight("n1", "weight", &1.0));
        nodes.push(Node::from_name_and_weight("n2", "weight", &2.0));
        nodes.push(Node::<&str, &str, &f64>::from_name("n3"));
        
        let mut edges = Vec::new();
        edges.push(Edge { u: "n1", v: "n2" });
        edges.push(Edge { u: "n2", v: "n1" });
        edges.push(Edge { u: "n1", v: "n3" });

        DiGraph::<&str, &str, &f64>::new_from_nodes_and_edges(nodes, edges)

    }

}