use graphrs::{algorithms, generators, readwrite};
use rayon::prelude::*;
// use std::iter::Iterator;
use rand::prelude::*;
use std::thread;
use std::time::Duration;
use std::time::Instant;

fn main() {
    // let graph = generators::random::fast_gnp_random_graph(1000, 0.01, true, Some(1)).unwrap();
    // match readwrite::graphml::write_graphml(&graph, "random.graphml") {
    //     Ok(_) => {}
    //     Err(e) => println!("Error writing graph to file: {}", e),
    // }
    let now = Instant::now();
    // let _all_pairs = algorithms::shortest_path::dijkstra::all_pairs(&graph, false, None, false);
    // let _all_pairs =
    //     algorithms::shortest_path::dijkstra::all_pairs_iter(&graph, false, None, false);
    // let mut c = 0;
    // for _ap in _all_pairs {
    //     c = c + 1;
    // }
    sub().for_each(|x| println!("{}", x));
    // for i in sub() {
    //     println!("{}", i)
    // }
    let elapsed = now.elapsed();
    println!("Elapsed time: {:?}", elapsed);
}

fn sub() -> rayon::iter::Map<rayon::range::Iter<i32>, impl Fn(i32) -> i32> {
    (0..100).into_par_iter().map(|i| i * i)
}

// let mut rng = rand::thread_rng();
// let y: f64 = rng.gen::<f64>() * 3000.0_f64;
// thread::sleep(Duration::from_millis(y as u64));
