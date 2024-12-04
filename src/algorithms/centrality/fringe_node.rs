use std::cmp::Ordering;
use std::collections::BinaryHeap;

pub struct FringeNode {
    pub distance: f64,
    pub pred: usize,
    pub v: usize,
}

impl Ord for FringeNode {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.distance < other.distance {
            Ordering::Less
        } else if self.distance > other.distance {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

impl PartialOrd for FringeNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for FringeNode {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

impl Eq for FringeNode {}

/**
Pushes a `FringeNode` into the `fringe` `BinaryHeap`.
*/
#[inline]
pub fn push_fringe_node(fringe: &mut BinaryHeap<FringeNode>, v: usize, w: usize, vw_dist: f64) {
    fringe.push(FringeNode {
        distance: -vw_dist, // negative because BinaryHeap is a max heap
        pred: v,
        v: w,
    });
}
