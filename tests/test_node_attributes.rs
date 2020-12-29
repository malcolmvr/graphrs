#[cfg(test)]
mod tests {

    use graphx::NodeAttributes;

    #[test]
    fn test_merge_attributes() {
        let mut attr1 = NodeAttributes::new();
        attr1.attributes.insert("a", &1.0);
        attr1.attributes.insert("b", &2.0);
        let mut attr2 = NodeAttributes::new();
        attr2.attributes.insert("a", &1.5);
        attr2.attributes.insert("c", &3.0);
        let result = graphx::NodeAttributes::merge_attributes(&attr1, &attr2);
        assert_eq!(result.attributes.len(), 3);
        assert_eq!(result.attributes["a"], &1.5);
        assert_eq!(result.attributes["b"], &2.0);
        assert_eq!(result.attributes["c"], &3.0);
    }
    
}