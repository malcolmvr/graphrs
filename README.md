# graphrs

`graphrs` is a Rust package for the creation, manipulation and analysis of [graphs](https://en.wikipedia.org/wiki/Graph_(discrete_mathematics)).

It allows graphs to be created with support for:
* directed and undirected edges
* multiple edges between two nodes
* self-loops

A `Graph` has two generic arguments:
* `T`: Specifies the type to use for node names.
* `A`: Specifies the type to use for node and edge attributes. Attributes are *optional*
extra data that are associated with a node or an edge. For example, if nodes represent
people and `T` is an `i32` of their employee ID then the node attributes might store
their first and last names.

## Major structs

* `Graph`
* `Node`
* `Edge`

## Examples

### Create a weighted, directed graph

```
use graphrs::{Edge, Graph, GraphSpecs, Node};

let nodes = vec![
    Node::from_name("n1"),
    Node::from_name("n2"),
    Node::from_name("n3"),
];

let edges = vec![
    Edge::with_weight("n1", "n2", 1.0),
    Edge::with_weight("n2", "n1", 2.0),
    Edge::with_weight("n1", "n3", 3.0),
    Edge::with_weight("n2", "n3", 3.0),
];

let specs = GraphSpecs::directed();

let graph = Graph::<&str, ()>::new_from_nodes_and_edges(
    nodes,
    edges,
    specs
);
```

### Create an undirected graph from just edges

```
use graphrs::{Edge, Graph, GraphSpecs};

let mut graph: Graph<&str, ()> = Graph::new(GraphSpecs::undirected_create_missing());
let result = graph.add_edges(vec![
    Edge::new("n1", "n2"),
    Edge::new("n2", "n3"),
]);
```

### Create an empty graph with all possible specifications

```
use graphrs::{Graph, GraphSpecs, EdgeDedupeStrategy, MissingNodeStrategy, SelfLoopsFalseStrategy};

let graph = Graph::<&str, ()>::new(
    GraphSpecs {
        directed: true, 
        edge_dedupe_strategy: EdgeDedupeStrategy::Error,
        missing_node_strategy: MissingNodeStrategy::Error,
        multi_edges: false,
        self_loops: false,
        self_loops_false_strategy: SelfLoopsFalseStrategy::Error,
    }
);
```

### Generate a "complete" graph

```
use graphrs::{generators};
let graph = generators::classic::complete_graph(5, true);
```

### Find the shortest path between two nodes

```
use graphrs::{Edge, Graph, GraphSpecs, Node};
use graphrs::{algorithms::{shortest_path::{dijkstra}}};

let mut graph = Graph::<&str, ()>::new(GraphSpecs::directed_create_missing());
graph.add_edges(vec![
    Edge::with_weight("n1", "n2", 1.0),
    Edge::with_weight("n2", "n1", 2.0),
    Edge::with_weight("n1", "n3", 3.0),
    Edge::with_weight("n2", "n3", 1.1),
]);

let shortest_paths = dijkstra::single_source(&graph, true, "n1", Some("n3"), None, false);
assert_eq!(shortest_paths.unwrap().get("n3").unwrap().distance, 2.1);
```

### Compute the betweenness centrality for all nodes

```
use graphrs::{algorithms::{centrality::{betweenness}}, generators};
let graph = generators::social::karate_club_graph();
let centralities = betweenness::betweenness_centrality(&graph, false, true);
```

## Credits

Some of the structure of the API and some of the algorithms were inspired by NetworkX.

## License

MIT

