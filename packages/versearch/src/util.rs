use std::iter::Peekable;

/// An intersecting iterator
pub struct InterIter<I: Iterator> {
    iters: Vec<Peekable<I>>,
}

impl<I: Iterator> InterIter<I>
where
    I::Item: Ord,
{
    pub fn new<ItersType>(in_iters: ItersType) -> Self
    where
        ItersType: IntoIterator,
        ItersType::Item: IntoIterator<Item = I::Item, IntoIter = I>,
    {
        let mut iters: Vec<Peekable<I>> = Vec::new();
        for iter in in_iters {
            iters.push(iter.into_iter().peekable());
        }
        InterIter { iters }
    }
}

impl<I: Iterator> Iterator for InterIter<I>
where
    I::Item: Ord,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        while self.iters.iter_mut().all(|i| i.peek().is_some()) {
            let mut iter_iters = self.iters.iter_mut();
            let check = iter_iters.next()?.peek()?;
            if iter_iters.all(|j| j.peek() == Some(&check)) {
                // If all iterators are at the same current value, advance
                // every iterator and emit result!
                let value = self.iters[0].next()?;
                for iter in self.iters[1..].iter_mut() {
                    iter.next();
                }
                return Some(value);
            } else {
                // Otherwise only increment the minimum vector
                let mut iter_iter = self.iters.iter_mut();
                let mut least = iter_iter.next()?;
                for iter in iter_iter {
                    if iter.peek()? < least.peek()? {
                        least = iter;
                    }
                }
                least.next();
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::InterIter;

    #[test]
    fn multiple_iters() {
        let v1 = vec![1, 2, 3, 4, 5];
        let v2 = vec![3, 4, 5, 6];
        let v3 = vec![4, 5, 6, 7];
        let res: Vec<&usize> = InterIter::new(vec![v1.iter(), v2.iter(), v3.iter()]).collect();
        assert_eq!(res, vec![&4, &5]);
    }

    #[test]
    fn one_iter() {
        let v1 = vec![1, 2, 3, 4, 5];
        let res: Vec<&usize> = InterIter::new(vec![v1.iter()]).collect();
        assert_eq!(res, vec![&1, &2, &3, &4, &5]);
    }

    #[test]
    fn no_iters() {
        let res: Vec<&usize> =
            InterIter::new(Vec::new() as Vec<core::slice::Iter<usize>>).collect();
        assert_eq!(res, Vec::new() as Vec<&usize>);
    }

    #[test]
    fn empty_iters() {
        let v1 = vec![];
        let v2 = vec![];
        let v3 = vec![];
        let res: Vec<&usize> = InterIter::new(vec![v1.iter(), v2.iter(), v3.iter()]).collect();
        assert_eq!(res, Vec::new() as Vec<&usize>);
    }

    #[test]
    fn multiple_vecs() {
        let v1 = vec![1, 2, 3, 4, 5];
        let v2 = vec![3, 4, 5, 6];
        let v3 = vec![4, 5, 6, 7];
        let res: Vec<usize> = InterIter::new(vec![v1, v2, v3]).collect();
        assert_eq!(res, vec![4, 5]);
    }

    #[test]
    fn one_vec() {
        let v1 = vec![1, 2, 3, 4, 5];
        let res: Vec<usize> = InterIter::new(vec![v1]).collect();
        assert_eq!(res, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn no_vecs() {
        let res: Vec<usize> = InterIter::new(Vec::new() as Vec<Vec<usize>>).collect();
        assert_eq!(res, Vec::new() as Vec<usize>);
    }

    #[test]
    fn empty_vecs() {
        let v1 = vec![];
        let v2 = vec![];
        let v3 = vec![];
        let res: Vec<usize> = InterIter::new(vec![v1, v2, v3]).collect();
        assert_eq!(res, Vec::new() as Vec<usize>);
    }
}
