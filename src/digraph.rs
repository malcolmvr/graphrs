use std::hash::Hash;
use std::collections::HashMap;
use crate::{Node, Edge};


pub struct DiGraph<T, K, V> {
    nodes: HashMap<T, Node<T, K, V>>,
    edges: Vec<Edge<T>>,
}


impl<T, K, V> DiGraph<T, K, V> {


    pub fn new_from_nodes_and_edges(nodes: Vec<Node<T, K, V>>, edges: Vec<Edge<T>>) -> DiGraph<T, K, V>
    where T: Hash, T: Eq, T: Copy, K: Hash, K: Eq
    {
        let mut nodes_map = HashMap::with_capacity(nodes.len());
        for node in nodes {
            nodes_map.insert(node.name, node);
        }

        DiGraph { nodes: nodes_map, edges: edges }
    }


    pub fn new_from_tuples(nodes: Vec<(T, Option<HashMap<K, V>>)>, edges: Vec<(T, T)>) -> DiGraph<T, K, V>
        where T: Hash, T: Eq, T: Copy, K: Hash, K: Eq
    {
        let nodes_map = DiGraph::get_hashmap_for_nodes_vec(nodes);
    
        let mut edges_vec = Vec::<Edge<T>>::with_capacity(edges.len());
        for edge in edges {
            let _edge = Edge { u: edge.0, v: edge.1 };
            edges_vec.push(_edge);
        }

        DiGraph { nodes: nodes_map, edges: edges_vec }
    }


    fn get_hashmap_for_nodes_vec(nodes: Vec<(T, Option<HashMap<K, V>>)>) -> HashMap<T, Node<T, K, V>>
        where T: Hash, T: Eq, T: Copy, K: Hash, K: Eq
    {
        let mut nodes_map = HashMap::with_capacity(nodes.len());
        for node in nodes {
            let attr = match node.1 {
                Some(a) => a,
                None => HashMap::<K, V>::new(),
            };
            let _node = Node { name: node.0, attributes: attr };
            nodes_map.insert(_node.name, _node);
        }
        nodes_map
    }


    pub fn add_nodes(self, nodes: Vec<Node<T, K, V>>) -> DiGraph<T, K, V>
        where T: Hash, T: Eq, T: Copy, K: Hash, K: Eq
    {
        let mut nodes_map = HashMap::with_capacity(nodes.len());
        for (key, value) in self.nodes {
            nodes_map.insert(key, value);
        }
        for node in nodes {
            nodes_map.insert(node.name, node);
        }

        DiGraph { nodes: nodes_map, edges: self.edges }
    }


    pub fn add_nodes_from_tuples(self, nodes: Vec<(T, Option<HashMap<K, V>>)>) -> DiGraph<T, K, V>
        where T: Hash, T: Eq, T: Copy, K: Hash, K: Eq
    {
        let mut nodes_map = DiGraph::get_hashmap_for_nodes_vec(nodes);
        for (key, value) in self.nodes {
            nodes_map.insert(key, value);
        }

        DiGraph { nodes: nodes_map, edges: self.edges }
    }


    pub fn nodes(&self) -> &HashMap<T, Node<T, K, V>> {
        &self.nodes
    }


    pub fn edges(&self) -> &Vec<Edge<T>> {
        &self.edges
    }


}

