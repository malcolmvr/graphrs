# graphrs

`graphrs` is a Rust package for the creation, manipulation and analysis of [graphs](<https://en.wikipedia.org/wiki/Graph_(discrete_mathematics)>).

It allows graphs to be created with support for:

- directed and undirected edges
- multiple edges between two nodes
- self-loops

A `Graph` has two generic arguments:

- `T`: Specifies the type to use for node names.
- `A`: Specifies the type to use for node and edge attributes. Attributes are _optional_
  extra data that are associated with a node or an edge. For example, if nodes represent
  people and `T` is an `i32` of their employee ID then the node attributes might store
  their first and last names.

## Documentation

The doc.rs documentation [is here](<https://doc.rs/graphrs>).

## Major structs

- `Graph`
- `Node`
- `Edge`

## Modules

- `algorithms::centrality`
- `algorithms::centrality`
- `algorithms::cluster`
- `algorithms::community`
- `algorithms::components`
- `algorithms::shortest_path`
- `generators`
- `readwrite`

## Examples

### Create a weighted, directed graph

```rust
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

```rust
use graphrs::{Edge, Graph, GraphSpecs};

let mut graph: Graph<&str, ()> = Graph::new(GraphSpecs::undirected_create_missing());
let result = graph.add_edges(vec![
    Edge::new("n1", "n2"),
    Edge::new("n2", "n3"),
]);
```

### Create an empty graph with all possible specifications

```rust
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

### Generate graphs

```rust
use graphrs::{generators};
let graph_complete = generators::classic::complete_graph(5, true);
let graph_random = generators::random::fast_gnp_random_graph(250, 0.25, true, None);
```

### Find the shortest path between two nodes

```rust
use graphrs::{Edge, Graph, GraphSpecs, Node};
use graphrs::{algorithms::{shortest_path::{dijkstra}}};

let mut graph = Graph::<&str, ()>::new(GraphSpecs::directed_create_missing());
graph.add_edges(vec![
    Edge::with_weight("n1", "n2", 1.0),
    Edge::with_weight("n2", "n1", 2.0),
    Edge::with_weight("n1", "n3", 3.0),
    Edge::with_weight("n2", "n3", 1.1),
]);

let shortest_paths = dijkstra::single_source(&graph, true, "n1", Some("n3"), None, false, true);
assert_eq!(shortest_paths.unwrap().get("n3").unwrap().distance, 2.1);
```

### Compute the betweenness, closeness and eigenvector centrality for all nodes

```rust
use graphrs::{algorithms::centrality, generators};
let graph = generators::social::karate_club_graph();
let centralities = centrality::betweenness::betweenness_centrality(&graph, false, true);
let closeness = centrality::closeness::closeness_centrality(&graph, false, true);
let centralities = centrality::eigenvector::eigenvector_centrality(&graph, false, None, None);
```

### Detect communities within a graph

```rust
use graphrs::{algorithms::{community}, generators};
let graph = generators::social::karate_club_graph();
let partitions = community::louvain::louvain_partitions(&graph, false, None, None, Some(1));
```

### Read and write graphml files

```rust,ignore
use graphrs::{readwrite, GraphSpecs};
let graph = readwrite::graphml::read_graphml_file("/some/file.graphml", GraphSpecs::directed());
readwrite::graphml::write_graphml(&graph, "/some/other/file.graphml");
```

## Performance

A comparison of the performance of `graphrs` against `NetworkX`, `igraph` and `graph-tool` can be found [here](performance.md).

## Credits

Some of the structure of the API and some of the algorithms were inspired by NetworkX.

## License

MIT
