use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt;
use std::fmt::{Debug, Display};
use std::hash::{Hash, Hasher};

/**
Represents a graph node, with `name` and `attributes`.
**/
#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Node<T, K, V> {
    pub name: T,
    pub attributes: Option<HashMap<K, V>>,
}

impl<T, K, V> Node<T, K, V> {

    /**
    Returns a `Node` with the specified `name` and no attributes.

    ```
    use graphrs::Node;
    let node = Node::<&str, &str, &f64>::from_name("n1");
    ```
    **/
    pub fn from_name(name: T) -> Node<T, K, V> {
        Node {
            name,
            attributes: None,
        }
    }

    /**
    Returns a `Node` with the specified `name` and populates the `Node.attributes`
    `HashMap` with the attributes tuples in the `attributes` parameter.

    ```
    use graphrs::Node;
    let node = Node::from_name_and_attribute_tuples("n1", vec![("a", 1.0), ("b", 1.0)]);
    ```
    **/
    pub fn from_name_and_attribute_tuples(name: T, attributes: Vec<(K, V)>) -> Node<T, K, V>
    where
        T: Hash + Eq + Copy,
        K: Hash + Eq + Copy,
        V: Copy,
    {
        Node {
            name,
            attributes: Some(attributes.into_iter().collect::<HashMap<K, V>>()),
        }
    }
}

impl<T: Eq + Ord, K, V> Ord for Node<T, K, V> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.cmp(&other.name)
    }
}

impl<T: Eq + PartialOrd + Ord, K, V> PartialOrd for Node<T, K, V> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: std::cmp::PartialEq, K, V> PartialEq for Node<T, K, V> {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl<T: Eq, K, V> Eq for Node<T, K, V> {}

impl<T: Debug, K, V> fmt::Debug for Node<T, K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Node").field("name", &self.name).finish()
    }
}

impl<T: Display, K, V> fmt::Display for Node<T, K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl<T: Hash, K, V> Hash for Node<T, K, V> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}
