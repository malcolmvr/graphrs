use std::cmp::Ordering;
use std::fmt;
use std::fmt::{Debug, Display};
use std::hash::{Hash, Hasher};

/**
Represents a graph node, with `name` and `attributes`.
*/
#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Node<T: Send, A> {
    pub name: T,
    pub attributes: Option<A>,
}

impl<T, A> Node<T, A>
where
    T: Eq + Clone + PartialOrd + Ord + Hash + Send + Sync + Display,
    A: Clone,
{
    /**
    Returns a `Node` with the specified `name` and no attributes.

    # Arguments

    `name`: The name of the node.

    # Examples

    ```
    use graphrs::Node;
    let node = Node::<&str, ()>::from_name("n1");
    ```
    */
    pub fn from_name(name: T) -> Node<T, A> {
        Node {
            name,
            attributes: None,
        }
    }

    /**
    Returns a `Node` with the specified `name` and `attributes`.

    # Arguments

    `name`: The name of the node.
    `name`: The attributes for the node.

    # Examples

    ```
    use graphrs::Node;

    #[derive(Clone, Copy)]
    struct Attributes {
        a: i32,
        b: f64
    }

    let node = Node::from_name_and_attributes("n1", Attributes {a: 3, b: 4.5});
    ```
    */
    pub fn from_name_and_attributes(name: T, attributes: A) -> Node<T, A>
    where
        T: Hash + Eq + Clone,
    {
        Node {
            name,
            attributes: Some(attributes),
        }
    }
}

impl<T: Eq + Ord + Send + Sync, A> Ord for Node<T, A> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.cmp(&other.name)
    }
}

impl<T: Eq + PartialOrd + Ord + Send + Sync, A> PartialOrd for Node<T, A> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: std::cmp::PartialEq + Send + Sync, A> PartialEq for Node<T, A> {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl<T: Eq + Send + Sync, A> Eq for Node<T, A> {}

impl<T: Debug + Send + Sync, A> fmt::Debug for Node<T, A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Node").field("name", &self.name).finish()
    }
}

impl<T: Display + Send + Sync, A> fmt::Display for Node<T, A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl<T: Hash + Send + Sync, A> Hash for Node<T, A> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}
