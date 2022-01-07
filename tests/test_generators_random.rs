#[cfg(test)]
mod tests {

    use graphrs::generators;

    #[test]
    fn test_fast_gnp_random_graph() {
        let graph = generators::random::fast_gnp_random_graph(10, 0.3, true, None).unwrap();
        let all_nodes = graph.get_all_nodes();
        assert_eq!(all_nodes.len(), 10);
    }
}
