use graphrs::{algorithms, generators, readwrite};
use rayon::prelude::*;
use statrs::statistics::Statistics;
use std::time::Instant;
use vec_to_array::vec_to_array;

fn main() {
    let graph = generators::random::fast_gnp_random_graph(10000, 0.01, true, Some(1)).unwrap();
    println!("edges: {}", graph.get_all_edges().len());
    match readwrite::graphml::write_graphml(&graph, "random.graphml") {
        Ok(_) => println!("Wrote graph to file"),
        Err(e) => println!("Error writing graph to file: {}", e),
    }
    const NUM_ITERATIONS: usize = 10;

    //let warm_up1 = algorithms::shortest_path::dijkstra::all_pairs(&graph, false, None, false);
    //for _ap in warm_up1 {}
    /*
    let mut times1 = vec![];
    for _i in 0..10 {
        let now1 = Instant::now();
        let _all_pairs1 =
            algorithms::shortest_path::dijkstra::all_pairs(&graph, false, None, false);
        for _ap in _all_pairs1 {}
        let elapsed1 = now1.elapsed().as_secs_f64() * 1000.0;
        times1.push(elapsed1);
    }
    let times1_array = vec_to_array!(times1, f64, 10);
    let stdev1: f64 = times1_array.population_std_dev();
    let mean1 = times1_array.mean();
    println!("Mean: {}", mean1);
    println!("Stdev: {}", stdev1);
    */

    // let warm_up3 = algorithms::shortest_path::dijkstra::all_pairs_iter(&graph, false, None, false);
    // warm_up3.for_each(|(_a, _b)| {});

    let mut times3 = vec![];
    for _i in 0..NUM_ITERATIONS {
        let now3 = Instant::now();
        let _all_pairs3 =
            algorithms::shortest_path::dijkstra::all_pairs_iter(&graph, false, None, false);
        _all_pairs3.for_each(|(_a, _b)| {});
        let elapsed3 = now3.elapsed().as_secs_f64() * 1000.0;
        times3.push(elapsed3);
    }
    let times3_array = vec_to_array!(times3, f64, NUM_ITERATIONS);
    let stdev3: f64 = times3_array.population_std_dev();
    let mean3 = times3_array.mean();
    println!("\nMean Iter: {}", mean3);
    println!("Stdev Iter : {}", stdev3);

    // let warm_up2 =
    //     algorithms::shortest_path::dijkstra::all_pairs_par_iter(&graph, false, None, false);
    // warm_up2.for_each(|(_a, _b)| {});

    let mut times2 = vec![];
    for _i in 0..NUM_ITERATIONS {
        let now2 = Instant::now();
        let _all_pairs2 =
            algorithms::shortest_path::dijkstra::all_pairs_par_iter(&graph, false, None, false);
        _all_pairs2.for_each(|(_a, _b)| {});
        let elapsed2 = now2.elapsed().as_secs_f64() * 1000.0;
        times2.push(elapsed2);
    }
    let times2_array = vec_to_array!(times2, f64, NUM_ITERATIONS);
    let stdev2: f64 = times2_array.population_std_dev();
    let mean2 = times2_array.mean();
    println!("\nMean Par Iter: {}", mean2);
    println!("Stdev Par Iter : {}", stdev2);
}
