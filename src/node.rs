use std::hash::Hash;
use std::collections::HashMap;


pub struct Node<T, K, V> {
    pub name: T,
    pub attributes: HashMap<K, V>,
}


impl<T, K, V> Node<T, K, V> {


    pub fn from_name(name: T) -> Node<T, K, V> {
        Node { name, attributes: HashMap::new() }
    }


    pub fn from_name_and_weight(name: T, weight_name: K, weight: V) -> Node<T, K, V>
        where K: Hash, K: Eq
    {
        let mut attributes = HashMap::new();
        attributes.insert(weight_name, weight);
        Node { name, attributes }
    }


    // pub fn from_name
}
