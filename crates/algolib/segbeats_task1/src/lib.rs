#![allow(dead_code, unused_variables)]
use std::ops::{Add, AddAssign, RangeBounds};

#[derive(Debug, Clone, PartialEq)]
struct SegbeatsTask1<T> {
    table: Vec<Node<T>>,
}

impl<T: Elm> SegbeatsTask1<T> {
    fn new(src: &[T]) -> Self {
        todo!()
    }
    fn change_min(&mut self, range: impl RangeBounds<usize>, x: T) {
        todo!()
    }
    fn query_max(&self, range: impl RangeBounds<usize>) -> T {
        todo!()
    }
    fn query_sum(&self, range: impl RangeBounds<usize>) -> T {
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq, Copy, Eq)]
struct Node<T> {
    max: [T; 2],
    c_max: usize,
    sum: T,
}
impl<T: Elm> Node<T> {
    fn new(x: T) -> Self {
        Node {
            max: [x, T::min_value()],
            c_max: 1,
            sum: x,
        }
    }
    fn merge(left: Node<T>, right: Node<T>) -> Self {
        use std::cmp::Ordering;
        let [a, b] = left.max;
        let [c, d] = right.max;
        let sum = left.sum + right.sum;
        match a.cmp(&c) {
            Ordering::Equal => Node {
                max: [a, b.max(d)],
                c_max: left.c_max + right.c_max,
                sum,
            },
            Ordering::Greater => Node {
                max: [a, c.max(d)],
                c_max: left.c_max,
                sum,
            },
            Ordering::Less => Node {
                max: [c, a.max(b)],
                c_max: right.c_max,
                sum,
            },
        }
    }
}

pub trait Elm: Sized + std::fmt::Debug + Copy + Ord + Add<Output = Self> + AddAssign {
    fn min_value() -> Self;
}
impl Elm for i32 {
    fn min_value() -> Self {
        std::i32::MAX
    }
}

#[cfg(test)]
mod tests {
    mod impl_query;
    mod queries;
    mod vector;

    use super::SegbeatsTask1;
    use queries::{ChangeMin, QueryMax, QuerySum};
    use query_test::impl_help;
    use rand::prelude::*;
    use vector::{Len, Value, Vector};

    type Tester<T, G> = query_test::Tester<StdRng, Vector<T>, SegbeatsTask1<T>, G>;

    #[test]
    fn test_i32() {
        #[derive(Debug, Clone, PartialEq, Copy, Eq)]
        struct G {}
        impl_help! {Len, |rng| rng.gen_range(1, 20); }
        impl_help! {Value<i32>, |rng| rng.gen_range(0, 28); }

        let mut tester = Tester::<i32, G>::new(StdRng::seed_from_u64(42));
        for _ in 0..4 {
            tester.initialize();
            for _ in 0..100 {
                let command = tester.rng_mut().gen_range(0, 3);
                match command {
                    0 => tester.mutate::<ChangeMin<_>>(),
                    2 => tester.compare::<QueryMax<_>>(),
                    3 => tester.compare::<QuerySum<_>>(),
                    _ => unreachable!(),
                }
            }
        }
    }
}
