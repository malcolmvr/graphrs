use std::hash::Hash;
use std::collections::{HashMap, HashSet};
use crate::{Node, Edge, EdgeSide};
use crate::merge_attributes::{merge_attributes, MergeStrategy};


// pub enum MissingNodeStrategy {
//     Create,
//     Err,
// }


pub struct DiGraph<T, K, V> {
    nodes: HashMap<T, Node<T, K, V>>,
    edges: HashMap<(T, T), Edge<T, K, V>>,
    successors: HashMap<T, HashSet<T>>,
    predecessors: HashMap<T, HashSet<T>>,
}


impl<T, K, V> DiGraph<T, K, V> {


    pub fn add_or_update_edges(mut self, edges: Vec<Edge<T, K, V>>) -> DiGraph<T, K, V>
        where T: Hash + Eq + Copy, K: Hash + Eq + Copy, V: Copy
    {
        self.successors = add_edges_to_successors_or_predecessors(self.successors, &edges, EdgeSide::U);
        self.predecessors = add_edges_to_successors_or_predecessors(self.predecessors, &edges, EdgeSide::V);

        for edge in edges {
            self.edges.insert((edge.u, edge.v), edge);
        }
    
        DiGraph { nodes: self.nodes, edges: self.edges, predecessors: self.predecessors, successors: self.successors }
    }


    /// Adds nodes to the DiGraph or updates the attributes of existing nodes.
    /// `merge_strategy` describes how existing and new attributes are to be merged.
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

        DiGraph { nodes: nodes_map, edges: self.edges, predecessors: self.predecessors, successors: self.successors }
    }


    pub fn get_all_edges(&self) -> Vec<&Edge<T, K, V>> {
        self.edges.values().collect::<Vec<&Edge<T, K, V>>>()
    }


    pub fn get_all_nodes(&self) -> Vec<&Node<T, K, V>> {
        self.nodes.values().collect::<Vec<&Node<T, K, V>>>()
    }


    pub fn get_edge(&self, u: T, v: T) -> Option<&Edge<T, K, V>>
        where T: Hash + Eq + Copy
    {
        self.edges.get(&(u, v))
    }


    pub fn get_node(&self, name: &T) -> Option<&Node<T, K, V>>
        where T: Hash + Eq + Copy
    {
        self.nodes.get(name)
    }


    pub fn get_predecessors_map(&self) -> &HashMap<T, HashSet<T>> {
        &self.predecessors
    }


    pub fn get_predecessor_nodes(&self, node_name: &T) -> Option<Vec<&Node<T, K, V>>>
        where T: Hash + Eq + Copy, 
    {
        let pred = self.predecessors.get(node_name);
        match pred {
            None => None,
            Some(hashset) => {
                Some(self.get_nodes_for_names(&hashset))
            }
        }
    }


    pub fn get_successors_map(&self) -> &HashMap<T, HashSet<T>> {
        &self.successors
    }


    pub fn get_successor_nodes(&self, node_name: &T) -> Option<Vec<&Node<T, K, V>>>
        where T: Hash + Eq + Copy, 
    {
        let pred = self.successors.get(node_name);
        match pred {
            None => None,
            Some(hashset) => {
                Some(self.get_nodes_for_names(&hashset))
            }
        }
    }


    pub fn new_from_nodes_and_edges(nodes: Vec<Node<T, K, V>>, edges: Vec<Edge<T, K, V>>) -> DiGraph<T, K, V>
        where T: Hash + Eq + Copy, K: Hash + Eq + Copy, V: Copy
    {
        let mut nodes_map = HashMap::with_capacity(nodes.len());
        for node in nodes {
            nodes_map.insert(node.name, node);
        }

        let successors = dedupe_and_group_edges(&edges, EdgeSide::U);
        let predecessors = dedupe_and_group_edges(&edges, EdgeSide::V);

        let mut edges_map = HashMap::with_capacity(edges.len());
        for edge in edges {
            edges_map.insert((edge.u, edge.v), edge);
        }

        DiGraph { nodes: nodes_map, edges: edges_map, successors, predecessors }
    }


    // PRIVATE METHODS


    fn get_nodes_for_names(&self, names: &HashSet<T>) -> Vec<&Node<T, K, V>>
        where T: Hash + Eq + Copy, 
    {
        let mut nodes = Vec::new();
        for node_name in names {
            nodes.push(self.nodes.get(node_name).unwrap());
        }
        nodes
    }


    // fn get_edges_for_successors(&self) -> Vec<Edge<T, K, V>>
    //     where T: Hash + Eq + Copy, K: Hash + Eq + Copy, V: Copy
    // {
    //     let mut edges = Vec::new();
    //     for (key, value) in self.successors {
    //         for v in value {
    //             edge = Edge { u: key.clone(), v: key.clone(), weight:  }
    //         }
    //     }
    //     edges
    // }


}


fn dedupe_and_group_edges<T, K, V>(edges: &Vec<Edge<T, K, V>>, by: EdgeSide) -> HashMap<T, HashSet<T>>
    where T: Hash + Eq + Copy, K: Hash + Eq + Copy, V: Copy
{
    let mut hashmap = HashMap::<T, HashSet<T>>::new();
    for edge in edges {
        let key = match by { EdgeSide::U => edge.u, EdgeSide::V => edge.v };
        let value = match by { EdgeSide::U => edge.v, EdgeSide::V => edge.u };
        if !hashmap.contains_key(&key) {
            hashmap.insert(key, HashSet::<T>::new());
        }
        let hashset = hashmap.get_mut(&key).unwrap();
        hashset.insert(value.clone());
    }
    hashmap
}


fn add_edges_to_successors_or_predecessors<T, K, V>(mut pred_or_succ: HashMap<T, HashSet<T>>, edges: &Vec<Edge<T, K, V>>, side: EdgeSide) -> HashMap<T, HashSet<T>>
    where T: Hash + Eq + Copy
{
    for edge in edges {
        let key = match side { EdgeSide::U => edge.u, EdgeSide::V => edge.v };
        let e = pred_or_succ.get_mut(&key);
        if e != None {
            e.unwrap().insert(edge.v);
        } else {
            let mut hashset = HashSet::new();
            let value = match side { EdgeSide::U => edge.v, EdgeSide::V => edge.u };
            hashset.insert(value);
            pred_or_succ.insert(edge.u, hashset);
        }
    }
    pred_or_succ
}

