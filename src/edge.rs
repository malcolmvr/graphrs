use std::collections::HashMap;
use std::fmt;
use std::fmt::{Debug, Display};
use std::hash::{Hash, Hasher};

pub enum EdgeSide {
    U,
    V,
}

#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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

impl<T: std::cmp::PartialEq, K, V> PartialEq for Edge<T, K, V> {
    fn eq(&self, other: &Self) -> bool {
        self.u == other.u && self.v == other.v
    }
}

impl<T: Debug, K, V> fmt::Debug for Edge<T, K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Node")
            .field("u", &self.u)
            .field("v", &self.v)
            .finish()
    }
}

impl<T: Display, K, V> fmt::Display for Edge<T, K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.u, self.v)
    }
}

impl<T: Hash, K, V> Hash for Edge<T, K, V> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.u.hash(state);
        self.v.hash(state);
    }
}
