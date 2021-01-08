use std::collections::HashMap;
use std::hash::Hash;

#[derive(Clone)]
pub struct Node<T, K, V> {
    pub name: T,
    pub attributes: Option<HashMap<K, V>>,
}

impl<T, K, V> Node<T, K, V> {
    pub fn from_name(name: T) -> Node<T, K, V> {
        Node {
            name,
            attributes: None,
        }
    }

    pub fn from_name_and_attribute_tuples(name: T, attributes: Vec<(K, V)>) -> Node<T, K, V>
    where
        T: Hash + Eq + Copy,
        K: Hash + Eq + Copy,
        V: Copy,
    {
        let mut attr = HashMap::new();
        for (key, value) in attributes {
            attr.insert(key, value);
        }
        Node {
            name,
            attributes: Some(attr),
        }
    }
}