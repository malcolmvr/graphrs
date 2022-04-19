use itertools::Itertools;

pub struct GroupByCount<I: Iterator> {
    #[allow(clippy::type_complexity)]
    inner: itertools::structs::GroupBy<I::Item, I, fn(&I::Item) -> I::Item>,
}

impl<I> Iterator for GroupByCount<I>
where
    I: Iterator,
    I::Item: PartialEq,
{
    type Item = (I::Item, usize);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner
            .into_iter()
            .next()
            .map(|(key, group)| (key, group.count()))
    }
}

pub trait IteratorExt: Iterator {
    fn group_by_count(self) -> GroupByCount<Self>
    where
        Self: Sized;
}

impl<I> IteratorExt for I
where
    I: Iterator,
    I::Item: Clone + PartialEq,
{
    fn group_by_count(self) -> GroupByCount<Self>
    where
        Self: Sized,
    {
        GroupByCount {
            inner: self.group_by(|i| i.clone()),
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
        let result: HashMap<i32, usize> = data.into_iter().sorted().group_by_count().collect();
        assert_eq!(result.get(&-2).unwrap(), &2);
        assert_eq!(result.get(&0).unwrap(), &1);
        assert_eq!(result.get(&1).unwrap(), &3);
        assert_eq!(result.get(&2).unwrap(), &1);
    }
}
