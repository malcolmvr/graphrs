use std::collections::HashMap;
use std::fmt;
use std::fmt::{Debug, Display};
use std::hash::{Hash, Hasher};
use std::cmp::{PartialOrd, Ord, Ordering};

pub enum EdgeSide {
    U,
    V,
}

#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Edge<T, K, V> where T: PartialOrd {
    pub u: T,
    pub v: T,
    pub attributes: Option<HashMap<K, V>>,
}

impl<T: std::cmp::PartialOrd, K, V> Edge<T, K, V> {
    pub fn new(u: T, v: T) -> Edge<T, K, V> {
        Edge {
            u,
            v,
            attributes: None,
        }
    }

    pub fn ordered(self: Edge<T, K, V>) -> Edge<T, K, V> {
        return match self.u > self.v {
            true => self.reversed(),
            false => self,
        };
    }

    pub fn reversed(self: Edge<T, K, V>) -> Edge<T, K, V> {
        Edge {
            u: self.v,
            v: self.u,
            ..self
        }
    }

    pub fn with_attribute(u: T, v: T, name: K, value: V) -> Edge<T, K, V>
    where
        K: Hash,
        K: Eq,
    {
        let attr = vec![(name, value)].into_iter().collect::<HashMap::<K, V>>();
        Edge {
            u,
            v,
            attributes: Some(attr),
        }
    }
}

impl<T: PartialEq + PartialOrd, K, V> PartialEq for Edge<T, K, V> {
    fn eq(&self, other: &Self) -> bool {
        self.u == other.u && self.v == other.v
    }
}

impl<T: Eq + PartialOrd, K, V> Eq for Edge<T, K, V> {}

impl<T: Debug + PartialOrd, K, V> fmt::Debug for Edge<T, K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Node")
            .field("u", &self.u)
            .field("v", &self.v)
            .finish()
    }
}

impl<T: Display + PartialOrd, K, V> fmt::Display for Edge<T, K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.u, self.v)
    }
}

impl<T: Hash + PartialOrd, K, V> Hash for Edge<T, K, V> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.u.hash(state);
        self.v.hash(state);
    }
}

impl<T: Eq + PartialEq + PartialOrd, K, V> PartialOrd for Edge<T, K, V> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Eq + PartialOrd, K, V> Ord for Edge<T, K, V> {
    fn cmp(&self, other: &Self) -> Ordering {
        let u_cmp = self.u.partial_cmp(&other.u).unwrap();
        match u_cmp {
            Ordering::Equal => self.v.partial_cmp(&other.v).unwrap(),
            Ordering::Greater => u_cmp,
            Ordering::Less => u_cmp,
        }
    }
}
