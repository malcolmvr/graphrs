use crate::{Edge, Error, ErrorKind, Graph, GraphSpecs, Node};
use quick_xml::{
    events::{BytesEnd, BytesStart, BytesText, Event},
    Reader, Writer,
};
use std::collections::HashMap;
use std::fmt::Display;
use std::fs;
use std::fs::File;
use std::hash::Hash;
use std::io::{BufWriter, Write};
use std::str;

/**
Creates a graph according to the contents of a GraphML-formatted file.

The only attributes that are currently supported are a "weight" attribute for edges.

# Arguments

* `file`: the path to a GraphML-formatted file
* `specs`: the [GraphSpecs](../../struct.GraphSpecs.html) to use for the created [Graph](../../struct.Graph.html)

# Examples

```ignore
use graphrs::{readwrite, GraphSpecs};
let graph = readwrite::graphml::read_graphml_file("/some/file.graphml", GraphSpecs::directed());
```
*/
pub fn read_graphml_file(file: &str, specs: GraphSpecs) -> Result<Graph<String, ()>, Error> {
    let string = fs::read_to_string(file).expect("could not open the specified file");
    read_graphml_string(&string, specs)
}

/**
Creates a graph according to the contents of a GraphML-formatted file.

The only attributes that are currently supported are a "weight" attribute for edges.

# Arguments

* `string`: the path to a GraphML-formatted file
* `specs`: the [GraphSpecs](../../struct.GraphSpecs.html) to use for the created [Graph](../../struct.Graph.html)

# Examples

```ignore
use graphrs::{readwrite, GraphSpecs};
let string = "<graphml xmlns=\"http://graphml.graphdrawing.org/xmlns\" xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\" xsi:schemaLocation=\"http://graphml.graphdrawing.org/xmlns http://graphml.graphdrawing.org/xmlns/1.0/graphml.xsd\"><graph edgedefault=\"undirected\"><node id=\"1\"/><node id=\"2\"/><edge source=\"1\" target=\"2\"></edge></graph></graphml>";
let graph = readwrite::graphml::read_graphml_string("/some/file.graphml", GraphSpecs::directed());
```
*/
pub fn read_graphml_string(string: &str, specs: GraphSpecs) -> Result<Graph<String, ()>, Error> {
    let mut reader = Reader::from_str(string);
    let mut buf = Vec::new();
    let mut directed: bool = true;
    let mut nodes: Vec<Node<String, ()>> = vec![];
    let mut edges: Vec<Edge<String, ()>> = vec![];
    let mut last_element_name: String = "".to_string();
    let mut edge_weight_attr_name = "weight".to_string();
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Empty(ref e)) => match e.name().as_ref() {
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
                b"key" => {
                    let attrs = get_attributes_as_hashmap(e);
                    if attrs.contains_key("attr.name")
                        && attrs.get("attr.name").unwrap() == "weight"
                        && attrs.get("for").unwrap() == "edge"
                    {
                        edge_weight_attr_name = attrs.get("id").unwrap().to_string();
                    }
                }
                _ => (),
            },
            Ok(Event::Start(ref e)) => {
                match e.name().as_ref() {
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
                                    directed = false;
                                }
                                _ => {
                                    return Err(get_read_error("the <graph> element's \"edgedefault\" attribute does not have a valid value; it should be one of \"directed\" or \"undirected\""));
                                }
                            },
                        }
                    }
                    b"node" => {
                        last_element_name = "node".to_string();
                        let result = add_node(&mut nodes, e);
                        if let Err(value) = result {
                            return Err(value);
                        }
                    }
                    b"edge" => {
                        last_element_name = "edge".to_string();
                        let result = add_edge(&mut edges, e);
                        if let Err(value) = result {
                            return Err(value);
                        }
                    }
                    b"key" => {
                        let attrs = get_attributes_as_hashmap(e);
                        if attrs.contains_key("attr.name")
                            && attrs.get("attr.name").unwrap() == "weight"
                            && attrs.get("for").unwrap() == "edge"
                        {
                            edge_weight_attr_name = attrs.get("id").unwrap().to_string();
                        }
                    }
                    b"data" => {
                        let attrs = get_attributes_as_hashmap(e);
                        if attrs.contains_key("key") {
                            let key = attrs.get("key").unwrap();
                            if key == &edge_weight_attr_name {
                                let mut buf = Vec::new();
                                match reader.read_event_into(&mut buf) {
                                    Ok(Event::Text(e)) => {
                                        let weight = str::from_utf8(&e).unwrap();
                                        match last_element_name.as_str() {
                                            "edge" => {
                                                let edge = edges.last_mut().unwrap();
                                                edge.weight = weight.parse::<f64>().unwrap();
                                            }
                                            _ => (),
                                        }
                                    }
                                    _ => (),
                                }
                            }
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

The only attributes that are currently written are a "weight" attribute for edges.

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
pub fn write_graphml_file<T, A>(graph: &Graph<T, A>, file: &str) -> Result<(), std::io::Error>
where
    T: Eq + Clone + PartialOrd + Ord + Hash + Send + Sync + Display,
    A: Clone,
{
    let string = write_graphml_string(graph)?;
    let mut file = File::create(file)?;
    file.write_all(string.as_bytes())?;
    Ok(())
}

/**
Writes a `Graph` to a GraphML-formatted file.

The only attributes that are currently written are a "weight" attribute for edges.

# Arguments

* `graph` the `Graph` object to write to file
* `file` the name of the file to write

# Examples

```ignore
use graphrs::{generators, readwrite};
let graph = generators::social::karate_club_graph();
let string = readwrite::graphml::write_graphml_string(&graph);
```
*/
pub fn write_graphml_string<T, A>(graph: &Graph<T, A>) -> Result<String, std::io::Error>
where
    T: Eq + Clone + PartialOrd + Ord + Hash + Send + Sync + Display,
    A: Clone,
{
    let bufwriter = BufWriter::new(Vec::new());
    let mut writer = Writer::new(bufwriter);

    let mut graphml_elem_start = BytesStart::new("graphml");
    graphml_elem_start.push_attribute(("xmlns", "http://graphml.graphdrawing.org/xmlns"));
    graphml_elem_start.push_attribute(("xmlns:xsi", "http://www.w3.org/2001/XMLSchema-instance"));
    graphml_elem_start.push_attribute(("xsi:schemaLocation", "http://graphml.graphdrawing.org/xmlns http://graphml.graphdrawing.org/xmlns/1.0/graphml.xsd"));
    assert!(writer.write_event(Event::Start(graphml_elem_start)).is_ok());

    let mut key_elem = BytesStart::new("key");
    key_elem.push_attribute(("id", "weight"));
    key_elem.push_attribute(("for", "edge"));
    key_elem.push_attribute(("attr.name", "weight"));
    key_elem.push_attribute(("attr.type", "double"));
    assert!(writer.write_event(Event::Empty(key_elem)).is_ok());

    let mut graph_elem_start = BytesStart::new("graph");
    let edge_default = match graph.specs.directed {
        true => "directed",
        false => "undirected",
    };
    graph_elem_start.push_attribute(("edgedefault", edge_default));
    assert!(writer.write_event(Event::Start(graph_elem_start)).is_ok());

    for node in graph.get_all_nodes() {
        let mut node_elem_start = BytesStart::new("node");
        node_elem_start.push_attribute(("id", format!("{}", node.name).as_str()));
        assert!(writer.write_event(Event::Empty(node_elem_start)).is_ok());
    }

    for edge in graph.get_all_edges() {
        let mut edge_elem_start = BytesStart::new("edge");
        edge_elem_start.push_attribute(("source", format!("{}", edge.u).as_str()));
        edge_elem_start.push_attribute(("target", format!("{}", edge.v).as_str()));
        assert!(writer.write_event(Event::Start(edge_elem_start)).is_ok());
        write_edge_weight(&mut writer, &edge);
        let edge_elem_end = BytesEnd::new("edge");
        assert!(writer.write_event(Event::End(edge_elem_end)).is_ok());
    }

    let graph_elem_end = BytesEnd::new("graph");
    assert!(writer.write_event(Event::End(graph_elem_end)).is_ok());

    let graphml_elem_end = BytesEnd::new("graphml");
    assert!(writer.write_event(Event::End(graphml_elem_end)).is_ok());

    let bytes = writer.into_inner().into_inner()?;
    let string = String::from_utf8(bytes).unwrap();
    Ok(string)
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
            let key_vec = attr.key.local_name().as_ref().to_vec();
            let key = String::from_utf8(key_vec).unwrap();
            let value = attr.unescape_value().unwrap().into_owned();
            (key, value)
        })
        .collect()
}

fn get_read_error(message: &str) -> Error {
    Error {
        kind: ErrorKind::ReadError,
        message: message.to_string(),
    }
}

fn write_edge_weight<T, A>(writer: &mut Writer<BufWriter<Vec<u8>>>, edge: &Edge<T, A>)
where
    T: Eq + Clone + PartialOrd + Ord + Hash + Send + Sync + Display,
    A: Clone,
{
    if edge.weight.is_nan() {
        return;
    }
    let mut edge_data_elem_start = BytesStart::new("data");
    edge_data_elem_start.push_attribute(("key", "weight"));
    assert!(writer
        .write_event(Event::Start(edge_data_elem_start))
        .is_ok());
    assert!(writer
        .write_event(Event::Text(BytesText::new(
            format!("{}", edge.weight).as_str()
        )))
        .is_ok());
    let edge_data_elem_end = BytesEnd::new("data");
    assert!(writer.write_event(Event::End(edge_data_elem_end)).is_ok());
}
