use std::hash::Hash;
// use std::hash::{Hash, Hasher};
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


// impl<T, K, V>  Hash for Edge<T, K, V> 
//     where T: Hash
// {
//     fn hash<H: Hasher>(&self, state: &mut H) {
//         self.u.hash(state);
//         self.v.hash(state);
//     }
// }


// impl<T, K, V> Clone for Edge<T, K, V>
//     where T: Hash + Eq + Copy, K: Hash + Eq + Copy, V: Copy
// {
//     fn clone(&self) -> Edge<T, K, V> {
//         Edge { }
//     }
// }

pub enum EdgeSide {
    U,
    V,
}

