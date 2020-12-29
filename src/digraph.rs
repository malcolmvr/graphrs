use std::collections::HashMap;
use crate::node_attributes::NodeAttributes;

pub struct DiGraph<'a> {
    adj: HashMap<&'a str, HashMap<&'a str, NodeAttributes<'a>>>,
    node: HashMap<&'a str, NodeAttributes<'a>>,
    pred: HashMap<&'a str, HashMap<&'a str, NodeAttributes<'a>>>,
    succ: HashMap<&'a str, HashMap<&'a str, NodeAttributes<'a>>>,
}

impl<'a> DiGraph<'a> {

    pub fn new() -> DiGraph<'a> {
        let adj = HashMap::<&str, HashMap<&str, NodeAttributes<'a>>>::new();
        let node = HashMap::<&'a str, NodeAttributes<'a>>::new();
        let pred = HashMap::<&str, HashMap<&str, NodeAttributes<'a>>>::new();
        let succ = HashMap::<&str, HashMap<&str, NodeAttributes<'a>>>::new();
        DiGraph { adj, node, pred, succ }
    }

    pub fn add_node(&mut self, node_for_adding: &'a str, attr: Option<NodeAttributes<'a>>) -> &mut DiGraph<'a> {
        let _attr = match attr {
            Some(a) => a,
            None => NodeAttributes::new(),
        };
        if !self.succ.contains_key(node_for_adding) {
            self.succ.insert(node_for_adding, HashMap::<&str, NodeAttributes<'a>>::new());
            self.pred.insert(node_for_adding, HashMap::<&str, NodeAttributes<'a>>::new());
            self.node.insert(node_for_adding, _attr);
        } else {
            self.update_node_attributes(node_for_adding, &_attr);
        }
        self
    }

    pub fn add_edge(&mut self, u: &'a str, v: &'a str, attr: Option<NodeAttributes<'a>>) {
        let _attr = match attr {
            Some(a) => a,
            None => NodeAttributes::new(),
        };
        if !self.node.contains_key(u) {
            self.node.insert(u, NodeAttributes::new());
            self.adj.insert(u, HashMap::<&str, NodeAttributes<'a>>::new());
        }
        if !self.node.contains_key(v) {
            self.node.insert(v, NodeAttributes::new());
            self.adj.insert(v, HashMap::<&str, NodeAttributes<'a>>::new());
        }
        let current_attr = self.adj.get_mut(u).unwrap().get_mut(v).unwrap();
        let merged = NodeAttributes::merge_attributes(&current_attr, &_attr);
        *current_attr = merged;
    }

    pub fn get_node_attributes(&self, n: &str) -> &NodeAttributes<'a> {
        &self.node[n]
    }

    pub fn update_node_attributes(&mut self, n: &str, attr: &NodeAttributes<'a>) {
        let current_attr = self.get_node_attributes(n);
        let combined = NodeAttributes::merge_attributes(&current_attr, &attr);
        *self.node.get_mut(n).unwrap() = combined;
    }
}

