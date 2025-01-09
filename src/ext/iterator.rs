use itertools::Itertools;
use nohash::{IntSet, IsEnabled};
use std::hash::Hash;

pub struct ChunkByCount<I: Iterator> {
    #[allow(clippy::type_complexity)]
    inner: itertools::structs::ChunkBy<I::Item, I, fn(&I::Item) -> I::Item>,
}

impl<I> Iterator for ChunkByCount<I>
where
    I: Iterator,
    I::Item: PartialEq,
{
    type Item = (I::Item, usize);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner
            .into_iter()
            .next()
            .map(|(key, chunk)| (key, chunk.count()))
    }
}

pub struct UniqueByNoHash<I: Iterator, V, F> {
    inner: I,
    seen: IntSet<V>,
    f: F,
}

impl<I, V, F> Iterator for UniqueByNoHash<I, V, F>
where
    I: Iterator,
    V: IsEnabled + Hash + Eq,
    F: FnMut(&I::Item) -> V,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().filter(|x| {
            let key = (self.f)(x);
            let r = !self.seen.contains(&key);
            self.seen.insert(key);
            r
        })
    }
}

pub trait IteratorExt: Iterator {
    fn chunk_by_count(self) -> ChunkByCount<Self>
    where
        Self: Sized;
    fn unique_by_no_hash<V, F>(self, f: F) -> UniqueByNoHash<Self, V, F>
    where
        Self: Sized,
        V: IsEnabled + Hash + Eq,
        for<'a> F: FnMut(&'a Self::Item) -> V;
}

impl<I> IteratorExt for I
where
    I: Iterator,
    I::Item: Clone + PartialEq,
{
    fn chunk_by_count(self) -> ChunkByCount<Self>
    where
        Self: Sized,
    {
        ChunkByCount {
            inner: self.chunk_by(|i| i.clone()),
        }
    }
    fn unique_by_no_hash<V, F>(self, f: F) -> UniqueByNoHash<Self, V, F>
    where
        Self: Sized,
        V: IsEnabled + Hash + Eq,
        F: FnMut(&I::Item) -> V,
    {
        UniqueByNoHash::<Self, V, F> {
            inner: self,
            seen: IntSet::<V>::default(),
            f,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_group_by_count_1() {
        let data = vec![1, 3, -2, -2, 1, 0, 1, 2];
        let result: HashMap<i32, usize> = data.into_iter().sorted().chunk_by_count().collect();
        assert_eq!(result.get(&-2).unwrap(), &2);
        assert_eq!(result.get(&0).unwrap(), &1);
        assert_eq!(result.get(&1).unwrap(), &3);
        assert_eq!(result.get(&2).unwrap(), &1);
    }
}
