#[cfg(test)]
mod tests {

    use graphrs::{generators, readwrite, Edge, Graph, GraphSpecs};

    #[test]
    fn test_write_then_read_graphml_file_1() {
        let file = "./tests/karate.graphml";

        // write
        let graph = generators::social::karate_club_graph();
        let result = readwrite::graphml::write_graphml_file(&graph, file);
        assert!(result.is_ok());

        // read
        let result = readwrite::graphml::read_graphml_file(file, GraphSpecs::undirected());
        assert!(result.is_ok());
        let graph = result.unwrap();
        let nodes = graph.get_all_nodes();
        assert_eq!(nodes.len(), 34);
        let edges = graph.get_all_edges();
        assert_eq!(edges.len(), 78);

        // assert!(std::fs::remove_file(file).is_ok());
    }

    #[test]
    fn test_write_then_read_graphml_file_2() {
        let edges = vec![
            Edge::with_weight("n1", "n2", 1.1),
            Edge::with_weight("n2", "n3", 5.0),
            Edge::with_weight("n1", "n4", 2.0),
            Edge::with_weight("n4", "n3", 3.0),
            Edge::with_weight("n1", "n5", 9.0),
            Edge::with_weight("n3", "n5", 1.0),
        ];
        let graph: Graph<&str, ()> = Graph::new_from_nodes_and_edges(
            vec![],
            edges,
            GraphSpecs {
                directed: true,
                ..GraphSpecs::directed_create_missing()
            },
        )
        .unwrap();

        // write
        let file = "./tests/graph2.graphml";
        let result = readwrite::graphml::write_graphml_file(&graph, file);
        assert!(result.is_ok());

        // read
        let result = readwrite::graphml::read_graphml_file(file, GraphSpecs::directed());
        assert!(result.is_ok());
        let graph = result.unwrap();
        let nodes = graph.get_all_nodes();
        assert_eq!(nodes.len(), 5);
        let edges = graph.get_all_edges();
        assert_eq!(edges.len(), 6);
        assert_eq!(
            graph
                .get_edge("n1".to_string(), "n2".to_string())
                .unwrap()
                .weight,
            1.1
        );
        assert_eq!(
            graph
                .get_edge("n1".to_string(), "n5".to_string())
                .unwrap()
                .weight,
            9.0
        );

        assert!(std::fs::remove_file(file).is_ok());
    }

    #[test]
    fn test_write_graphml_string_1() {
        // karate graph as a string
        let string = "<graphml xmlns=\"http://graphml.graphdrawing.org/xmlns\" xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\" xsi:schemaLocation=\"http://graphml.graphdrawing.org/xmlns http://graphml.graphdrawing.org/xmlns/1.0/graphml.xsd\"><graph edgedefault=\"undirected\"><node id=\"18\"/><node id=\"3\"/><node id=\"25\"/><node id=\"5\"/><node id=\"20\"/><node id=\"10\"/><node id=\"31\"/><node id=\"0\"/><node id=\"15\"/><node id=\"9\"/><node id=\"23\"/><node id=\"27\"/><node id=\"1\"/><node id=\"7\"/><node id=\"4\"/><node id=\"19\"/><node id=\"30\"/><node id=\"6\"/><node id=\"29\"/><node id=\"16\"/><node id=\"33\"/><node id=\"32\"/><node id=\"8\"/><node id=\"13\"/><node id=\"28\"/><node id=\"22\"/><node id=\"2\"/><node id=\"12\"/><node id=\"11\"/><node id=\"26\"/><node id=\"17\"/><node id=\"24\"/><node id=\"14\"/><node id=\"21\"/><edge source=\"0\" target=\"12\"></edge><edge source=\"0\" target=\"6\"></edge><edge source=\"2\" target=\"8\"></edge><edge source=\"31\" target=\"32\"></edge><edge source=\"1\" target=\"21\"></edge><edge source=\"5\" target=\"16\"></edge><edge source=\"0\" target=\"17\"></edge><edge source=\"4\" target=\"6\"></edge><edge source=\"8\" target=\"33\"></edge><edge source=\"0\" target=\"13\"></edge><edge source=\"22\" target=\"32\"></edge><edge source=\"6\" target=\"16\"></edge><edge source=\"2\" target=\"28\"></edge><edge source=\"18\" target=\"33\"></edge><edge source=\"0\" target=\"3\"></edge><edge source=\"30\" target=\"32\"></edge><edge source=\"9\" target=\"33\"></edge><edge source=\"30\" target=\"33\"></edge><edge source=\"2\" target=\"13\"></edge><edge source=\"24\" target=\"31\"></edge><edge source=\"0\" target=\"21\"></edge><edge source=\"0\" target=\"5\"></edge><edge source=\"1\" target=\"17\"></edge><edge source=\"3\" target=\"13\"></edge><edge source=\"5\" target=\"10\"></edge><edge source=\"15\" target=\"33\"></edge><edge source=\"24\" target=\"27\"></edge><edge source=\"20\" target=\"32\"></edge><edge source=\"26\" target=\"33\"></edge><edge source=\"24\" target=\"25\"></edge><edge source=\"23\" target=\"29\"></edge><edge source=\"0\" target=\"10\"></edge><edge source=\"13\" target=\"33\"></edge><edge source=\"26\" target=\"29\"></edge><edge source=\"29\" target=\"32\"></edge><edge source=\"25\" target=\"31\"></edge><edge source=\"2\" target=\"7\"></edge><edge source=\"0\" target=\"11\"></edge><edge source=\"1\" target=\"19\"></edge><edge source=\"18\" target=\"32\"></edge><edge source=\"0\" target=\"19\"></edge><edge source=\"8\" target=\"30\"></edge><edge source=\"1\" target=\"3\"></edge><edge source=\"23\" target=\"27\"></edge><edge source=\"23\" target=\"33\"></edge><edge source=\"3\" target=\"7\"></edge><edge source=\"4\" target=\"10\"></edge><edge source=\"29\" target=\"33\"></edge><edge source=\"23\" target=\"25\"></edge><edge source=\"2\" target=\"32\"></edge><edge source=\"2\" target=\"3\"></edge><edge source=\"0\" target=\"8\"></edge><edge source=\"14\" target=\"33\"></edge><edge source=\"23\" target=\"32\"></edge><edge source=\"19\" target=\"33\"></edge><edge source=\"1\" target=\"7\"></edge><edge source=\"22\" target=\"33\"></edge><edge source=\"8\" target=\"32\"></edge><edge source=\"5\" target=\"6\"></edge><edge source=\"0\" target=\"4\"></edge><edge source=\"20\" target=\"33\"></edge><edge source=\"31\" target=\"33\"></edge><edge source=\"0\" target=\"31\"></edge><edge source=\"32\" target=\"33\"></edge><edge source=\"1\" target=\"30\"></edge><edge source=\"28\" target=\"33\"></edge><edge source=\"1\" target=\"13\"></edge><edge source=\"14\" target=\"32\"></edge><edge source=\"0\" target=\"2\"></edge><edge source=\"3\" target=\"12\"></edge><edge source=\"15\" target=\"32\"></edge><edge source=\"28\" target=\"31\"></edge><edge source=\"2\" target=\"27\"></edge><edge source=\"1\" target=\"2\"></edge><edge source=\"0\" target=\"7\"></edge><edge source=\"2\" target=\"9\"></edge><edge source=\"27\" target=\"33\"></edge><edge source=\"0\" target=\"1\"></edge></graph></graphml>";

        // read
        let result = readwrite::graphml::read_graphml_string(string, GraphSpecs::undirected());
        assert!(result.is_ok());
        let graph = result.unwrap();
        let nodes = graph.get_all_nodes();
        assert_eq!(nodes.len(), 34);
        let edges = graph.get_all_edges();
        assert_eq!(edges.len(), 78);
    }

    #[test]
    fn test_write_graphml_string_2() {
        // karate graph as a string
        let string = "<graphml xmlns=\"http://graphml.graphdrawing.org/xmlns\" xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\" xsi:schemaLocation=\"http://graphml.graphdrawing.org/xmlns http://graphml.graphdrawing.org/xmlns/1.0/graphml.xsd\">
            <key id=\"k1\" for=\"edge\" attr.name=\"weight\" attr.type=\"double\" />
            <graph edgedefault=\"undirected\">
                <node id=\"n1\"/>
                <node id=\"n2\"/>
                <edge source=\"n1\" target=\"n2\">
                    <data key=\"k1\">1.1</data>
                </edge>
            </graph>
        </graphml>";

        // read
        let result = readwrite::graphml::read_graphml_string(string, GraphSpecs::undirected());
        assert!(result.is_ok());
        let graph = result.unwrap();
        let nodes = graph.get_all_nodes();
        assert_eq!(nodes.len(), 2);
        let edges = graph.get_all_edges();
        assert_eq!(edges.len(), 1);
        assert_eq!(
            graph
                .get_edge("n1".to_string(), "n2".to_string())
                .unwrap()
                .weight,
            1.1
        );
    }
}
