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
        let mut merged = HashMap::new();
        if new_attributes.is_some() {
            for (key, value) in new_attributes.as_ref().unwrap() {
                merged.insert(key.clone(), value.clone());
            }
        }
        Some(merged)
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
