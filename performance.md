# graphrs Performance

`graphrs` is designed to use parallel processing to achieve faster execution times.
The graphrs functions that take advantage of parallel processing are:

- `algorithms::shortest_path::dijkstra::all_pairs`
- `algorithms::shortest_path::dijkstra::multi_source`
- `algorithms::shortest_path::dijkstra::get_all_shortest_paths_involving`
- `algorithms::centrality::betweenness::betweenness_centrality`
- `algorithms::centrality::betweenness::closeness_centrality`

We'll compare the betweenness and closeness centrality algorithms to their `NetworkX`,
`graph-tool` and `igraph` equivalents.

## Versions

The versions of the libraries used for the performance testing:

| Library                | Version |
| ---------------------- | ------- |
| rustc                  | 1.82.0  |
| graphrs                | 1.0.0   |
| rayon                  | 1.10.0  |
| Python                 | 3.12.3  |
| NetworkX               | 3.4.2   |
| graph-tool             | 2.80    |
| igraph (python-igraph) | 0.11.8  |

## Graph creation

Random graphs were created with `fast_gnp_random_graph`, then written to a graphml file. For example:

```
let graph = generators::random::fast_gnp_random_graph(100, 0.01, true, Some(1)).unwrap();
readwrite::graphml::write_graphml(&graph, "random.graphml");
```

## graphrs execution and timing

The graphrs `dijkstra::betweenness_centrality` and `dijkstra::closeness_centrality` methods were executed
on the generated graphs and timed. For example:

```
let start = Instant::now();
algorithms::centrality::betweenness::betweenness_centrality(&graph, true, true);
let elapsed = now.elapsed().as_secs_f64();
```

Each method was executed ten times so that the mean and the standard deviation could be computed. The testing was done on an AWS m5d.4xlarge server, which has 16 vCPUs.

## NetworkX execution and timing

The graphml created above file is then read and NetworkX's `all_pairs_dijkstra` method is executed and timed. The code used:

```
from time import perf_counter
import networkx as nx

graph = nx.read_graphml("random.graphml")

start = perf_counter()
nx.betweenness_centrality(graph, weight="weight")
elapsed = end - start
```

## graph-tool execution and timing

```
from time import perf_counter

graph = gt.load_graph("random.graphml", fmt="graphml")

start = perf_counter()
gt.betweenness(graph, weight=graph.ep.weight)
elapsed = end - start
```

## igraph execution and timing

```
from time import perf_counter

graph = ig.read("random.graphml")

start = perf_counter()
graph.betweenness(weights=graph.es["weight"])
elapsed = end - start
```

## Results

The results are in milliseconds.

### Betweenness Centrality

| Number of Nodes / Edges | NetworkX | graph-tool | igraph  | graphrs |
| ----------------------- | -------- | ---------- | ------- | ------- |
| 10 / 50                 | 0.723    | 1.389      | 0.048   | 0.035   |
| 100 / 506               | 44.92    | 1.61       | 2.00    | 1.51    |
| 250 / 3,209             | 522.05   | 3.88       | 17.37   | 5.10    |
| 500 / 12,605            | 3,432.6  | 12.5       | 109.2   | 19.9    |
| 2,000 / 199,702         | 294,397  | 478        | 8,280   | 670     |
| 5,000 / 499,205         | n/a      | 3,410      | 61,919  | 4,474   |
| 10,000 / 10,157         | n/a      | 17,029     | 285,400 | 21,3770 |

### Closeness Centrality

| Number of Nodes / Edges | NetworkX | graph-tool | igraph  | graphrs |
| ----------------------- | -------- | ---------- | ------- | ------- |
| 10 / 50                 | 1.015    | 5.818      | 0.041   | 0.070   |
| 100 / 506               | 25.54    | 1.63       | 2.14    | 1.58    |
| 250 / 3,209             | 320.0    | 14.9       | 25.4    | 5.74    |
| 500 / 12,605            | 2,396.5  | 11.0       | 176.4   | 24.0    |
| 2,000 / 199,702         | 192,494  | 517        | 12,156  | 760     |
| 5,000 / 499,205         | n/a      | 3,797      | 99,425  | 3,876   |
| 10,000 / 10,157         |          | 17,484     | 511,349 | 15,027  |

### Conclusion

`graphrs` meets or exceeds igraph's performance on small graphs and gets close to graph-tool's
performance on large graphs. There's still some room for improvement!