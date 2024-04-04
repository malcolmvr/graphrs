# graphrs Performance

## graphrs vs NetworkX - Dijsktra all pairs

### Versions

The versions of the libraries used for the performance testing:

| Library  | Version |
| -------- | ------- |
| rustc    | 1.76.0  |
| graphrs  | 0.9.0   |
| rayon    | 1.9.0   |
| Python   | 3.11.9  |
| NetworkX | 3.2.1   |

### Graph creation

Random graphs were created with `fast_gnp_random_graph`, then written to a graphml file. For example:

```
let graph = generators::random::fast_gnp_random_graph(100, 0.01, true, Some(1)).unwrap();
readwrite::graphml::write_graphml(&graph, "random.graphml");
```

### graphrs execution and timing

The graphrs `dijkstra::all_pairs_iter` and ``dijkstra::all_pairs_par_iter` methods were executed on the generated graphs and timed. For example:

```
let start = Instant::now();
let _all_pairs = algorithms::shortest_path::dijkstra::all_pairs_iter(&graph, false, None, false);
_all_pairs.for_each(|(_a, _b)| {});
let elapsed = now.elapsed().as_secs_f64();
```

Each method was executed ten times so that the mean and the standard deviation could be computed. The testing was done on an AWS m5d.4xlarge server, which has 16 vCPUs.

### NetworkX execution and timing

The graphml file is then read with NetworkX's `all_pairs_dijkstra` method, executed and timed. The code used:

```
import time
import networkx as nx

graph = nx.read_graphml("../graphrs/random.graphml")

start = time.perf_counter()
all_pairs = nx.all_pairs_dijkstra(graph)
for ap in all_pairs:
    pass
end = time.perf_counter()
elapsed = end - start
```

### Results

The results (in milliseconds):

| Number of Nodes / Edges | NetworkX mean | NetworkX stdev | graphrs iter mean | graphrs iter stdev | graphrs par_iter mean | graphrs par_iter stdev |
| ----------------------- | ------------- | -------------- | ----------------- | ------------------ | --------------------- | ---------------------- |
| 100 / 109               | 1.24          | 0.01           | 0.54              | 0.01               | 0.25                  | 0.36                   |
| 500 / 2,583             | 677           | 41             | 383               | 0.17               | 43.5                  | 0.90                   |
| 1,000 / 5,063           | 2,825         | 34             | 1,575             | 7.8                | 173                   | 1.9                    |
| 5,000 / 25,181          | 81,585        | 862            | 41,440            | 282                | 5,318                 | 76                     |
| 10,000 / 10,157         | 2,743         | 10.0           | 1,302             | 1.5                | 140                   | 3.0                    |

NetworkX's `all_pairs_dijkstra` (Python) does fairly well against graphrs' `dijkstra::all_pairs_iter` (Rust) - it only takes roughly twice as much time to finish. The graphrs `dijkstra::all_pairs_par_iter`, which parallelizes the work across available CPUs is where graphrs can provide a significant reduction in computation time.
