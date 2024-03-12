# graphrs Performance

## graphrs vs NetworkX

We're going to generate a random graph with `fast_gnp_random_graph`, write the graph to a graphml file.

```
let graph = generators::random::fast_gnp_random_graph(100, 0.01, true, Some(1)).unwrap();
readwrite::graphml::write_graphml(&graph, "random.graphml");
```

Next, we'll execute and time the graphrs `dijkstra::all_pairs` method.

```
let start = Instant::now();
let _all_pairs = algorithms::shortest_path::dijkstra::all_pairs(&graph, false, None, false);
let elapsed = now..elapsed().as_secs_f64();
```

The graphml file is then read with Python and NetworkX's `all_pairs_dijkstra` method is executed and timed.

```
import time
import networkx as nx

graph = nx.read_graphml("/home/malcolm/Source/personal/graphrs/random.graphml")
print(graph.number_of_nodes())

start = time.perf_counter()
all_pairs = nx.all_pairs_dijkstra(graph)
for ap in all_pairs:
    pass
end = time.perf_counter()
print((end-start))
```

Finally, we'll execute and time the graphrs `dijkstra::all_pairs_par_iter` method. This method runs the Dijkstra algorithm in parallel, across multiple threads - however many the computer has. NetworkX does not have an algorithm to parallelize getting all Dijkstra pairs

We then increase the number of nodes and repeat the measurements. We'll do all this 10 times so that we can calculate mean and stdev.

The results:

| Number of Nodes | Python (ms) | graphrs (ms) | graphrs par_iter (ms) |
| --------------- | ----------- | ------------ | --------------------- |
| 100             | 0.887       |              |
| 500             | 298         | 56           |
| 1,000           | 560         | 380          |
| 5,000           | 343,561     | 47,938       |
| 10,000          |             |              |

graphrs - 10k non-parallel: 2,361,054
Python on same: 578,511 !
