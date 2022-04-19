use std::collections::HashSet;
use std::hash::Hash;

/// Extends `HashSet` with the `without` method.
pub trait VecExt<T> {
    fn to_hashset(&self) -> HashSet<T>;
}

impl<T> VecExt<T> for Vec<T>
where
    T: Clone + Eq + Hash,
{
    /**
    Returns a copy of the `HashSet` that doesn't contain the specified `value`.
    */
    fn to_hashset(&self) -> HashSet<T> {
        self.iter().cloned().collect()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_to_hashset_1() {
        let result: HashSet<i32> = vec![1, 1, 2, 3].to_hashset();
        assert_eq!(result.len(), 3);
        assert!(result.contains(&1));
        assert!(result.contains(&2));
        assert!(result.contains(&3));
    }
}
