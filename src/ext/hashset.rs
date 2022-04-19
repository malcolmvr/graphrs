use std::collections::HashSet;
use std::hash::Hash;

/// Extends `HashSet` with the `without` method.
pub trait HashSetExt<T> {
    fn without(&self, value: &T) -> HashSet<T>;
}

impl<T> HashSetExt<T> for HashSet<T>
where
    T: Clone + Eq + Hash,
{
    /**
    Returns a copy of the `HashSet` that doesn't contain the specified `value`.
    */
    fn without(&self, value: &T) -> HashSet<T> {
        self.iter().filter(|v| *v != value).cloned().collect()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use itertools::Itertools;

    #[test]
    fn test_without_1() {
        let hs1: HashSet<i32> = vec![1, 2, 3].into_iter().collect();
        let hs2 = hs1.without(&2);
        assert_eq!(hs2.len(), 2);
        let v: Vec<i32> = hs2.into_iter().sorted().collect();
        assert_eq!(v, vec![1, 3]);
    }
}
