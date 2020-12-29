#[cfg(test)]
mod tests {

    use graphx::{DiGraph,NodeAttributes};
    
    #[test]
    fn test_add_node() {
        let mut g = DiGraph::new();
        let g = g.add_node("n1", None);
        let attr = g.get_node_attributes("n1");
        assert_eq!(attr.attributes.len(), 0);
        let mut attr = NodeAttributes::new();
        attr.attributes.insert("a", &1.0);
        let g = g.add_node("n1", Some(attr));
        let attr = g.get_node_attributes("n1");
        assert_eq!(attr.attributes.len(), 1);
        let mut attr = NodeAttributes::new();
        attr.attributes.insert("b", &2.0);
        let g = g.add_node("n1", Some(attr));
        let attr = g.get_node_attributes("n1");
        assert_eq!(attr.attributes.len(), 2);
    }
    
}