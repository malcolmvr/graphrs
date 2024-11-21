use nohash::BuildNoHashHasher;
use std::time::Instant;
use std::{collections::HashSet, hash::Hash};

#[derive(PartialEq, Eq)]
struct EdgeIndex {
    u: usize,
    v: usize,
}

impl Hash for EdgeIndex {
    fn hash<H: std::hash::Hasher>(&self, hasher: &mut H) {
        let single_value: u64 = self.u as u64 + (self.v as u64) << 32;
        hasher.write_u64(single_value);
    }
}

impl nohash::IsEnabled for EdgeIndex {}

fn main() {
    let count = 10000000;

    let now = Instant::now();
    let mut hs: HashSet<(usize, usize)> = HashSet::new();
    hs.insert((0, 1));
    for i in (0..count) {
        let x = hs.contains(&(0, 1));
    }
    println!("Elapsed time: {:?}", now.elapsed());

    let now = Instant::now();
    let mut hs: HashSet<EdgeIndex, BuildNoHashHasher<EdgeIndex>> =
        HashSet::with_hasher(BuildNoHashHasher::default());
    for i in (0..count) {
        let x = hs.contains(&EdgeIndex { u: 0, v: 1 });
    }
    println!("Elapsed time: {:?}", now.elapsed());

    // hs.insert(EdgeIndex { u: 0, v: 1 });
    // hs.insert(EdgeIndex { u: 0, v: 1 });
    // hs.insert(EdgeIndex { u: 1, v: 0 });
    // println!("{:?}", hs.len());
}
