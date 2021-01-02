// use itertools::Itertools;
use std::hash::Hash;
use std::collections::HashMap;
use crate::{Node, Edge};


pub enum MergeStrategy {
    Replace,
    Update,
}


pub struct DiGraph<T, K, V> {
    nodes: HashMap<T, Node<T, K, V>>,
    edges: Vec<Edge<T, K, V>>,
    // successors: HashMap<T, Vec<T>>,
}


impl<T, K, V> DiGraph<T, K, V> {


    pub fn new_from_nodes_and_edges(nodes: Vec<Node<T, K, V>>, edges: Vec<Edge<T, K, V>>) -> DiGraph<T, K, V>
        where T: Hash + Eq + Copy, K: Hash + Eq + Copy, V: Copy
    {
        let mut nodes_map = HashMap::with_capacity(nodes.len());
        for node in nodes {
            nodes_map.insert(node.name, node);
        }

        // let successors = get_successors(edges);

        DiGraph { nodes: nodes_map, edges: edges }
    }


    /// Adds nodes to the DiGraph.
    /// If `merge_attributes` is true then nodes that were already defined
    /// will have their attributes merged. If false then their attributes
    /// will be replaced by the attributes in the `nodes` argument.
    pub fn add_or_update_nodes(self, nodes: Vec<Node<T, K, V>>, merge_strategy: MergeStrategy) -> DiGraph<T, K, V>
        where T: Hash + Eq + Copy, K: Hash + Eq + Copy, V: Copy
    {
        let mut nodes_map = HashMap::with_capacity(nodes.len());
        for (key, value) in self.nodes {
            nodes_map.insert(key, value);
        }
        for node in nodes {
            if nodes_map.contains_key(&node.name) {
                let mut existing_node = nodes_map.get_mut(&node.name).unwrap();
                existing_node.attributes = merge_attributes(&existing_node.attributes, &node.attributes, &merge_strategy);
            } else {
                nodes_map.insert(node.name, node);
            }
        }

        DiGraph { nodes: nodes_map, edges: self.edges }
    }


    pub fn nodes(&self) -> &HashMap<T, Node<T, K, V>> {
        &self.nodes
    }


    pub fn edges(&self) -> &Vec<Edge<T, K, V>> {
        &self.edges
    }


    // fn get_successors(edges: &Vec<Edge<T>>) -> HashMap<T, Vec<T>> {
    //     let mut successors = HashMap::with_capacity(10);
    // }

}


fn merge_attributes<'a, K, V>(existing_attributes: &Option<HashMap<K, V>>, new_attributes: &Option<HashMap<K, V>>, merge_strategy: &MergeStrategy) -> Option<HashMap<K, V>>
    where K: Hash + Eq + Copy, V: Copy
{
    let merged = match merge_strategy {
        MergeStrategy::Replace => {
            merge_attributes_replace(new_attributes)
        },
        MergeStrategy::Update => {
            merge_attributes_update(existing_attributes, new_attributes)
        }
    };
    merged
}


fn merge_attributes_replace<'a, K, V>(new_attributes: &Option<HashMap<K, V>>) -> Option<HashMap<K, V>>
    where K: Hash + Eq + Copy, V: Copy
{
    if new_attributes.is_none() {
        None
    } else {
        let mut merged = HashMap::new();
        if new_attributes.is_some() {
            for (key, value) in new_attributes.as_ref().unwrap() {
                merged.insert(key.clone(), value.clone());
            }
        }
        Some(merged)
    }
}


fn merge_attributes_update<'a, K, V>(existing_attributes: &Option<HashMap<K, V>>, new_attributes: &Option<HashMap<K, V>>) -> Option<HashMap<K, V>>
    where K: Hash + Eq + Copy, V: Copy
{
    let mut merged = HashMap::new();
    if existing_attributes.is_some() {
        for (key, value) in existing_attributes.as_ref().unwrap() {
            merged.insert(key.clone(), value.clone());
        }
    }
    if new_attributes.is_some() {
        for (key, value) in new_attributes.as_ref().unwrap() {
            merged.insert(key.clone(), value.clone());
        }
    }
    Some(merged)
}
