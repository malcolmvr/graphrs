use std::cmp::{Ord, Ordering, PartialOrd};
use std::fmt;
use std::fmt::{Debug, Display};
use std::hash::{Hash, Hasher};

/**
Represents a graph edge as (`u`, `v`).

Also allows `attributes`, as a `HashMap`, to be stored on an edge.
*/
#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Edge<T, A>
where
    T: PartialOrd + Send,
{
    /// The name of the first node of the edge.
    pub u: T,
    /// The name of the second node of the edge.
    pub v: T,
    /// Any attributes of the edge.
    pub attributes: Option<A>,
    /// The edge weight. For weighted `Graph` this should be a real number.
    /// For an unweighted `Graph` this should be `f64:NAN`.
    pub weight: f64,
}

impl<T, A> Edge<T, A>
where
    T: Eq + Clone + PartialOrd + Ord + Hash + Send + Sync + Display,
    A: Clone,
{
    /**
    Creates a new `Edge` with no attributes.

    # Arguments

    * `u`: The name of the first node of the edge.
    * `v`: The name of the second node of the edge.

    # Examples

    ```
    use graphrs::Edge;
    let edges = vec![
        Edge::<&str, ()>::new("n1", "n2"),
        Edge::<&str, ()>::new("n2", "n1"),
    ];
    ```
    */
    pub fn new(u: T, v: T) -> Edge<T, A> {
        Edge {
            u,
            v,
            attributes: None,
            weight: f64::NAN,
        }
    }

    /**
    Returns (v, u) if u > v.

    ```
    use graphrs::Edge;
    let edge1 = Edge::<&str, ()>::new("n2", "n1");
    let edge2 = edge1.ordered();
    assert_eq!(edge2.u, "n1");
    assert_eq!(edge2.v, "n2");
    ```
    */
    pub fn ordered(self: Edge<T, A>) -> Edge<T, A> {
        match self.u > self.v {
            true => self.reversed(),
            false => self,
        }
    }

    /**
    Reverses the edge. (u, v) -> (v, u)
    ```
    use graphrs::Edge;
    let edge1 = Edge::<&str, ()>::new("n2", "n1");
    let edge2 = edge1.reversed();
    assert_eq!(edge2.u, "n1");
    assert_eq!(edge2.v, "n2");
    ```
    */
    pub fn reversed(self: Edge<T, A>) -> Edge<T, A> {
        Edge {
            u: self.v,
            v: self.u,
            ..self
        }
    }

    /**
    Creates a (`u`, `v`) `Edge` with a specified `weight`.

    # Arguments

    * `u`: The name of the first node of the edge.
    * `v`: The name of the second node of the edge.
    * `weight`: The weight of the edge.

    # Examples

    ```
    use graphrs::Edge;
    let edges = vec![
        Edge::<&str, ()>::with_weight("n1", "n2", 1.0),
        Edge::<&str, ()>::with_weight("n2", "n1", 2.0),
    ];
    ```
    */
    pub fn with_weight(u: T, v: T, weight: f64) -> Edge<T, A> {
        Edge {
            u,
            v,
            attributes: None,
            weight,
        }
    }
}

impl<T: PartialEq + PartialOrd + Send + Sync, A> PartialEq for Edge<T, A> {
    fn eq(&self, other: &Self) -> bool {
        self.u == other.u && self.v == other.v
    }
}

impl<T: Eq + PartialOrd + Send + Sync, A> Eq for Edge<T, A> {}

impl<T: Debug + PartialOrd + Send + Sync, A> fmt::Debug for Edge<T, A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Node")
            .field("u", &self.u)
            .field("v", &self.v)
            .finish()
    }
}

impl<T: Display + PartialOrd + Send + Sync, A> fmt::Display for Edge<T, A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.u, self.v)
    }
}

impl<T: Hash + PartialOrd + Send + Sync, A> Hash for Edge<T, A> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.u.hash(state);
        self.v.hash(state);
    }
}

impl<T: Eq + PartialEq + PartialOrd + Send + Sync, A> PartialOrd for Edge<T, A> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Eq + PartialOrd + Send + Sync, A> Ord for Edge<T, A> {
    fn cmp(&self, other: &Self) -> Ordering {
        let u_cmp = self.u.partial_cmp(&other.u).unwrap();
        match u_cmp {
            Ordering::Equal => self.v.partial_cmp(&other.v).unwrap(),
            Ordering::Greater => u_cmp,
            Ordering::Less => u_cmp,
        }
    }
}
