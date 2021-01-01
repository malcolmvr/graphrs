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
    nodes: HashMap<T, Node<T, K, V>>,
    edges: Vec<Edge<T>>,
}

impl<T, K, V> DiGraph<T, K, V> {

    pub fn new(nodes: Vec<(T, Option<HashMap<K, V>>)>, edges: Vec<(T, T)>) -> DiGraph<T, K, V>
        where T: Hash, T: Eq, T: Copy, K: Hash, K: Eq
    {
        let mut nodes_map = HashMap::new();
        for node in nodes {
            let attr = match node.1 {
                Some(a) => a,
                None => HashMap::<K, V>::new(),
            };
            let _node = Node { name: node.0, attributes: attr };
            nodes_map.insert(_node.name, _node);
        }
    
        let mut edges_vec = Vec::<Edge<T>>::with_capacity(edges.len());
        for edge in edges {
            let _edge = Edge { u: edge.0, v: edge.1 };
            edges_vec.push(_edge);
        }

        DiGraph { nodes: nodes_map, edges: edges_vec }
    }

    pub fn nodes(&self) -> &HashMap<T, Node<T, K, V>> {
        &self.nodes
    }

    pub fn edges(&self) -> &Vec<Edge<T>> {
        &self.edges
    }

}

