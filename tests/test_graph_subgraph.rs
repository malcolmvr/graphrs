#[cfg(test)]
mod tests {

    use graphrs::generators;

    #[test]
    fn test_get_subgraph() {
        let graph = generators::social::karate_club_graph();
        let subgraph = graph.get_subgraph(&[4, 5, 6, 10, 16]);
        let all_nodes = subgraph.get_all_nodes();
        assert_eq!(all_nodes.len(), 5);
        let mut node_names = all_nodes.into_iter().map(|n| n.name).collect::<Vec<i32>>();
        node_names.sort_unstable();
        assert_eq!(node_names, vec![4, 5, 6, 10, 16]);
        assert_eq!(subgraph.get_all_edges().len(), 6);
        assert!(subgraph.get_edge(5, 6).is_ok());
        assert!(subgraph.get_edge(5, 16).is_ok());
        assert!(subgraph.get_edge(6, 16).is_ok());
        assert!(subgraph.get_edge(5, 10).is_ok());
        assert!(subgraph.get_edge(4, 10).is_ok());
        assert!(subgraph.get_edge(4, 6).is_ok());
    }
}
