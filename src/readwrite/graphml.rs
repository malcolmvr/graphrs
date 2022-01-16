use crate::{Edge, Error, ErrorKind, Graph, GraphSpecs, Node};
use quick_xml::{
    events::{BytesEnd, BytesStart, Event},
    Reader, Writer,
};
use std::collections::HashMap;
use std::fmt::Display;
use std::fs::File;
use std::hash::Hash;
use std::str;

/**
Creates a graph according to the contents of a GraphML-formatted file.

# Arguments

* `file`: the path to a GraphML-formatted file
* `specs`: the [GraphSpecs](../../struct.GraphSpecs.html) to use for the created [Graph](../../struct.Graph.html)

# Examples

```ignore
use graphrs::{readwrite, GraphSpecs};
let graph = readwrite::graphml::read_graphml("/some/file.graphml", GraphSpecs::directed());
```
*/
pub fn read_graphml(file: &str, specs: GraphSpecs) -> Result<Graph<String, ()>, Error> {
    let mut reader = Reader::from_file(file).expect("could not open the specified file");
    let mut buf = Vec::new();
    let mut directed: bool = true;
    let mut nodes: Vec<Node<String, ()>> = vec![];
    let mut edges: Vec<Edge<String, ()>> = vec![];
    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Empty(ref e)) => match e.name() {
                b"node" => {
                    let result = add_node(&mut nodes, e);
                    if let Err(value) = result {
                        return Err(value);
                    }
                }
                b"edge" => {
                    let result = add_edge(&mut edges, e);
                    if let Err(value) = result {
                        return Err(value);
                    }
                }
                _ => (),
            },
            Ok(Event::Start(ref e)) => {
                match e.name() {
                    b"graph" => {
                        let attrs = get_attributes_as_hashmap(e);
                        match attrs.get("edgedefault") {
                            None => {
                                return Err(get_read_error("the <graph> element does not have an \"edgedefault\" attribute"));
                            }
                            Some(value) => match value.as_str() {
                                "directed" => {
                                    directed = true;
                                }
                                "undirected" => {
                                    directed = true;
                                }
                                _ => {
                                    return Err(get_read_error("the <graph> element's \"edgedefault\" attribute does not have a valid value; it should be one of \"directed\" or \"undirected\""));
                                }
                            },
                        }
                    }
                    b"node" => {
                        let result = add_node(&mut nodes, e);
                        if let Err(value) = result {
                            return Err(value);
                        }
                    }
                    b"edge" => {
                        let result = add_edge(&mut edges, e);
                        if let Err(value) = result {
                            return Err(value);
                        }
                    }
                    _ => (),
                }
            }
            Ok(Event::Eof) => break, // exits the loop when reaching end of file
            Err(e) => {
                return Err(get_read_error(format!("{}", e).as_str()));
            }
            _ => (), // There are several other `Event`s we do not consider here
        }
    }
    let new_specs = GraphSpecs { directed, ..specs };
    Graph::new_from_nodes_and_edges(nodes, edges, new_specs)
}

/**
Writes a `Graph` to a GraphML-formatted file.

# Arguments

* `graph` the `Graph` object to write to file
* `file` the name of the file to write

# Examples

```ignore
use graphrs::{generators, readwrite};
let graph = generators::social::karate_club_graph();
readwrite::graphml::write_graphml(&graph, "/some/file.graphml");
```
*/
pub fn write_graphml<T, A>(graph: &Graph<T, A>, file: &str) -> Result<(), std::io::Error>
where
    T: Eq + Clone + PartialOrd + Ord + Hash + Send + Sync + Display,
    A: Clone,
{
    let f = File::create(file);
    if let Err(e) = f {
        return Err(e);
    }
    let mut writer = Writer::new(f.unwrap());

    let mut graphml_elem_start = BytesStart::owned(b"graphml".to_vec(), "graphml".len());
    graphml_elem_start.push_attribute(("xmlns", "http://graphml.graphdrawing.org/xmlns"));
    graphml_elem_start.push_attribute(("xmlns:xsi", "http://www.w3.org/2001/XMLSchema-instance"));
    graphml_elem_start.push_attribute(("xsi:schemaLocation", "http://graphml.graphdrawing.org/xmlns http://graphml.graphdrawing.org/xmlns/1.0/graphml.xsd"));
    assert!(writer.write_event(Event::Start(graphml_elem_start)).is_ok());

    let mut graph_elem_start = BytesStart::owned(b"graph".to_vec(), "graph".len());
    let edge_default = match graph.specs.directed {
        true => "directed",
        false => "undirected",
    };
    graph_elem_start.push_attribute(("edgedefault", edge_default));
    assert!(writer.write_event(Event::Start(graph_elem_start)).is_ok());

    for node in graph.get_all_nodes() {
        let mut node_elem_start = BytesStart::owned(b"node".to_vec(), "node".len());
        node_elem_start.push_attribute(("id", format!("{}", node.name).as_str()));
        assert!(writer.write_event(Event::Empty(node_elem_start)).is_ok());
    }

    for edge in graph.get_all_edges() {
        let mut edge_elem_start = BytesStart::owned(b"edge".to_vec(), "edge".len());
        edge_elem_start.push_attribute(("source", format!("{}", edge.u).as_str()));
        edge_elem_start.push_attribute(("target", format!("{}", edge.v).as_str()));
        assert!(writer.write_event(Event::Empty(edge_elem_start)).is_ok());
    }

    let graph_elem_end = BytesEnd::owned(b"graph".to_vec());
    assert!(writer.write_event(Event::End(graph_elem_end)).is_ok());

    let graphml_elem_end = BytesEnd::owned(b"graphml".to_vec());
    assert!(writer.write_event(Event::End(graphml_elem_end)).is_ok());

    Ok(())
}

fn add_edge(edges: &mut Vec<Edge<String, ()>>, e: &BytesStart) -> Result<(), Error> {
    let attrs = get_attributes_as_hashmap(e);
    if !attrs.contains_key("source") {
        return Err(get_read_error(
            "an <edge> element does not have a \"source\" attribute",
        ));
    }
    if !attrs.contains_key("target") {
        return Err(get_read_error(
            "an <edge> element does not have a \"target\" attribute",
        ));
    }
    let source = attrs.get("source").unwrap().to_string();
    let target = attrs.get("target").unwrap().to_string();
    edges.push(Edge::new(source, target));
    Ok(())
}

fn add_node(nodes: &mut Vec<Node<String, ()>>, e: &BytesStart) -> Result<(), Error> {
    let attrs = get_attributes_as_hashmap(e);
    match attrs.get("id") {
        None => Err(get_read_error(
            "a <node> element does not have an \"id\" attribute",
        )),
        Some(value) => {
            nodes.push(Node::from_name(value.to_string()));
            Ok(())
        }
    }
}

fn get_attributes_as_hashmap(event: &BytesStart) -> HashMap<String, String> {
    event
        .attributes()
        .map(|a| {
            let attr = a.unwrap();
            (
                str::from_utf8(attr.key).unwrap().to_string(),
                str::from_utf8(attr.value.as_ref()).unwrap().to_string(),
            )
        })
        .collect()
}

fn get_read_error(message: &str) -> Error {
    Error {
        kind: ErrorKind::ReadError,
        message: message.to_string(),
    }
}
