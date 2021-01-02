use std::hash::Hash;
use std::collections::HashMap;


pub struct Edge<T, K, V> {
    pub u: T,
    pub v: T,
    pub weight: Option<V>,
    pub attributes: Option<HashMap<K, V>>,
}


impl<T, K, V> Edge<T, K, V> {


    pub fn with_weight(u: T, v: T, weight: V) -> Edge<T, K, V>
        where K: Hash, K: Eq
    {
        Edge { u, v, weight: Some(weight), attributes: None }
    }


}
