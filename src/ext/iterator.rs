use itertools::Itertools;

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

pub trait IteratorExt: Iterator {
    fn chunk_by_count(self) -> ChunkByCount<Self>
    where
        Self: Sized;
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
