#[cfg(test)]
mod tests {

    use graphrs::{generators, readwrite, GraphSpecs};

    #[test]
    fn test_write_then_read_graphml() {
        
        // write
        let graph = generators::social::karate_club_graph();
        let result = readwrite::graphml::write_graphml(&graph, "./tests/files/karate.graphml");
        assert!(result.is_ok());

        // read
        let result = readwrite::graphml::read_graphml("./tests/files/karate.graphml", GraphSpecs::undirected());
        assert!(result.is_ok());
        let graph = result.unwrap();
        let nodes = graph.get_all_nodes();
        assert_eq!(nodes.len(), 34);
        let edges = graph.get_all_edges();
        assert_eq!(edges.len(), 78);
    }

}
