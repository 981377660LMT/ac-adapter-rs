use crate::{query, solve, utils, Vector};
use std::ops::Range;
use type_traits::Identity;

impl<T> solve::Mutate<query::Set<T>> for Vector<T> {
    fn mutate(&mut self, (i, x): (usize, T)) {
        self.0[i] = x;
    }
}
impl<T: Identity> solve::Solve<query::Fold<T>> for Vector<T> {
    fn solve(&self, range: Range<usize>) -> T {
        self.0[range].iter().cloned().fold(T::identity(), T::op)
    }
}
impl<T, U, P> solve::Judge<query::ForwardUpperBoundByKey<T, U, P>> for Vector<T>
where
    T: Identity,
    U: Ord,
    P: utils::Project<T, U>,
{
    fn judge(&self, (range, value): (Range<usize>, U), i: usize) -> bool {
        let fold = |range| P::project(<Self as solve::Solve<query::Fold<T>>>::solve(self, range));
        let Range { start, end } = range;
        i == end || range.contains(&i) && (fold(start..i)..fold(start..i + 1)).contains(&value)
    }
}
