use std::collections::HashSet;
use std::time::Instant;

fn main() {
    let i32vec = (0..1000).collect::<Vec<i32>>();

    let mut counter: i32 = 0;
    let count = 1000000000;

    let now = Instant::now();
    for i in (0..count) {
        for j in i32vec.iter() {
            counter = counter + 1;
        }
    }
    println!("Elapsed time: {:?}", now.elapsed());

    counter = 0;
    let hs = i32vec.iter().collect::<HashSet<&i32>>();
    let now = Instant::now();
    for i in (0..count) {
        for j in hs.iter() {
            counter = counter + 1;
        }
    }
    println!("Elapsed time: {:?}", now.elapsed());
}
