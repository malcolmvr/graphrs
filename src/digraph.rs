use std::hash::Hash;
use std::collections::HashMap;

pub struct Node<T, K, V> {
    pub name: T,
    pub attributes: HashMap<K, V>,
}

pub struct Edge<T> {
    pub u: T,
    pub v: T,
}

pub struct DiGraph<T, K, V> {
    pub nodes: HashMap<T, Node<T, K, V>>,
    pub edges: Vec<Edge<T>>,
}

impl<T, K, V> DiGraph<T, K, V> {

    pub fn new(nodes: Vec<(T, HashMap<K, V>)>, edges: Vec<(T, T)>) -> DiGraph<T, K, V>
        where T: Hash, T: Eq, T: Copy, K: Hash, K: Eq
    {
        let mut nodes_map = HashMap::new();

        for node in nodes {
            let _node = Node { name: node.0, attributes: node.1 };
            nodes_map.insert(_node.name, _node);
        }
    
        let mut edges_vec = Vec::<Edge<T>>::with_capacity(edges.len());
        for edge in edges {
            let u = nodes_map[&edge.0].name.clone();
            let v = nodes_map[&edge.1].name.clone();
            let _edge = Edge { u, v };
            edges_vec.push(_edge);
        }

        DiGraph { nodes: nodes_map, edges: edges_vec }
    }

}

