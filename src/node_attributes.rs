use std::collections::HashMap;
use std::ops::Index;

pub struct NodeAttributes<'a> {
    pub attributes: HashMap<&'a str, &'a f64>,
}

impl<'a> NodeAttributes<'a> {

    pub fn new() -> NodeAttributes<'a> {
        NodeAttributes { attributes: HashMap::<&'a str, &'a f64>::new() }
    }

    pub fn merge_attributes<'b>(attr1: &NodeAttributes<'b>, attr2: &NodeAttributes<'b>) -> NodeAttributes<'b> {
        let mut combined = NodeAttributes::new();
        for (key, value) in attr1.attributes.iter() {
            combined.attributes.insert(key, value);    
        }
        for (key, value) in attr2.attributes.iter() {
            combined.attributes.insert(key, value);
        }
        combined
    }
    
}

impl Index<&str> for NodeAttributes<'_> {
    type Output = f64;
    fn index(&self, name: &str) -> &Self::Output {
        self.attributes[name]
    }
}
