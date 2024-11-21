use std::collections::{HashMap, HashSet};
use std::time::Instant;

struct X {
    x: i32,
    y: i32,
}

fn main() {
    let x1 = X { x: 1, y: 2 };
    // let x2 = X { x: 3, y: 4 };
    // let x3 = X { x: 5, y: 6 };

    let mut hm = HashMap::<usize, X>::new();
    hm.insert(13, x1);

    let count = 100000;
    let now = Instant::now();
    for i in (0..count) {
        let x = hm.get(13).unwrap();
    }
    println!("Elapsed time: {:?}", now.elapsed());

    let array_of_x = (0..1).map(|_| X { x: 1, y: 2 }).collect::<Vec<X>>();
    let now = Instant::now();
    for i in (0..count) {
        let x = array_of_x.get(0).unwrap();
    }
    println!("Elapsed time: {:?}", now.elapsed());
}
