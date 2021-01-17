use crate::Node;
use std::collections::HashMap;
use std::hash::Hash;

pub enum AttributeMergeStrategy {
    Replace,
    Update,
}

pub fn merge_attributes<'a, K, V>(
    existing_attributes: &Option<HashMap<K, V>>,
    new_attributes: &Option<HashMap<K, V>>,
    merge_strategy: &AttributeMergeStrategy,
) -> Option<HashMap<K, V>>
where
    K: Hash + Eq + Copy,
    V: Copy,
{
    let merged = match merge_strategy {
        AttributeMergeStrategy::Replace => merge_attributes_replace(new_attributes),
        AttributeMergeStrategy::Update => {
            merge_attributes_update(existing_attributes, new_attributes)
        }
    };
    merged
}

fn merge_attributes_replace<'a, K, V>(
    new_attributes: &Option<HashMap<K, V>>,
) -> Option<HashMap<K, V>>
where
    K: Hash + Eq + Copy,
    V: Copy,
{
    if new_attributes.is_none() {
        None
    } else {
        new_attributes.clone()
    }
}

fn merge_attributes_update<'a, K, V>(
    existing_attributes: &Option<HashMap<K, V>>,
    new_attributes: &Option<HashMap<K, V>>,
) -> Option<HashMap<K, V>>
where
    K: Hash + Eq + Copy,
    V: Copy,
{
    let existing = match existing_attributes {
        None => vec![],
        Some(a) => a.clone().into_iter().collect::<Vec<(K, V)>>(),
    };
    let new = match new_attributes {
        None => vec![],
        Some(a) => a.clone().into_iter().collect::<Vec<(K, V)>>(),
    };
    let merged = existing.into_iter().chain(new).collect::<HashMap<K, V>>();
    Some(merged)
}

pub fn get_node_with_merged_attributes<'a, T, K, V>(
    existing_node1: &Node<T, K, V>,
    new_node: &Node<T, K, V>,
    merge_strategy: &AttributeMergeStrategy,
) -> Node<T, K, V>
where
    T: Hash + Eq + Copy,
    K: Hash + Eq + Copy,
    V: Copy,
{
    Node {
        name: existing_node1.name,
        attributes: merge_attributes(
            &existing_node1.attributes,
            &new_node.attributes,
            &merge_strategy,
        ),
    }
}
