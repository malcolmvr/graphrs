#[cfg(test)]
mod tests {

    use graphrs::Edge;
    use std::cmp::Ordering;

    #[test]
    fn test_edge_cmp() {
        let edge = Edge::<&str, &str, &f64>::new("n1", "n2");
        assert_eq!(
            edge.cmp(&Edge::<&str, &str, &f64>::new("n1", "n2")),
            Ordering::Equal
        );
        assert_eq!(
            edge.cmp(&Edge::<&str, &str, &f64>::new("n2", "n1")),
            Ordering::Less
        );
        assert_eq!(
            edge.cmp(&Edge::<&str, &str, &f64>::new("n0", "n1")),
            Ordering::Greater
        );
        assert_eq!(
            edge.cmp(&Edge::<&str, &str, &f64>::new("n1", "n3")),
            Ordering::Less
        );
        assert_eq!(
            edge.cmp(&Edge::<&str, &str, &f64>::new("n1", "n1")),
            Ordering::Greater
        );
    }
}
