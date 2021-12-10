use std::cmp::{Ord, Ordering, PartialOrd};
use std::collections::HashMap;
use std::fmt;
use std::fmt::{Debug, Display};
use std::hash::{Hash, Hasher};

/**
An enumeration that specifies which way to look at an edge side.
`U` treats edges as (u, v). `V` treats edges as (v, u).
**/
pub enum EdgeSide {
    U,
    V,
}

/**
Represents a graph edge as (`u`, `v`).

Also allows `attributes`, as a `HashMap`, to be stored on an edge.
**/
#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Edge<T, K, V>
where
    T: PartialOrd,
{
    pub u: T,
    pub v: T,
    pub attributes: Option<HashMap<K, V>>,
}

impl<T: std::cmp::PartialOrd, K, V> Edge<T, K, V> {

    /// Creates a new `Edge` with no attributes.
    /**
    Creates a (`u`, `v`) `Edge` with a {`name`: `value`} attribute.

    ```
    use graphrs::Edge;
    let edges = vec![
        Edge::<&str, &str, &f64>::new("n1", "n2"),
        Edge::<&str, &str, &f64>::new("n2", "n1"),
    ];
    ```
    **/
    pub fn new(u: T, v: T) -> Edge<T, K, V> {
        Edge {
            u,
            v,
            attributes: None,
        }
    }

    /**
    Returns (v, u) if u > v.

    ```
    use graphrs::Edge;
    let edge1 = Edge::<&str, &str, &f64>::new("n2", "n1");
    let edge2 = edge1.ordered();
    // edge2 is ("n1", "n2")
    ```
    **/
    pub fn ordered(self: Edge<T, K, V>) -> Edge<T, K, V> {
        return match self.u > self.v {
            true => self.reversed(),
            false => self,
        };
    }

    /**
    Reverses the edge. (u, v) -> (v, u)
    ```
    use graphrs::Edge;
    let edge1 = Edge::<&str, &str, &f64>::new("n2", "n1");
    let edge2 = edge1.reversed();
    // edge2 is ("n1", "n2")
    ```
    **/
    pub fn reversed(self: Edge<T, K, V>) -> Edge<T, K, V> {
        Edge {
            u: self.v,
            v: self.u,
            ..self
        }
    }

    /**
    Creates a (`u`, `v`) `Edge` with a {`name`: `value`} attribute.

    ```
    use graphrs::Edge;
    let edges = vec![
        Edge::with_attribute("n1", "n2", "weight", &1.0),
        Edge::with_attribute("n2", "n1", "weight", &2.0),
    ];
    ```
    **/
    pub fn with_attribute(u: T, v: T, name: K, value: V) -> Edge<T, K, V>
    where
        K: Hash,
        K: Eq,
    {
        let attr = vec![(name, value)].into_iter().collect::<HashMap<K, V>>();
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
