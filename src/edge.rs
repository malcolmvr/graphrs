use std::hash::Hash;
// use std::hash::{Hash, Hasher};
use std::collections::HashMap;

#[derive(Clone)]
pub struct Edge<T, K, V> {
    pub u: T,
    pub v: T,
    pub weight: Option<V>,
    pub attributes: Option<HashMap<K, V>>,
}

impl<T, K, V> Edge<T, K, V> {
    pub fn with_weight(u: T, v: T, weight: V) -> Edge<T, K, V>
    where
        K: Hash,
        K: Eq,
    {
        Edge {
            u,
            v,
            weight: Some(weight),
            attributes: None,
        }
    }
}

pub enum EdgeSide {
    U,
    V,
}
