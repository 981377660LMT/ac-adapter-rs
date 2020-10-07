use alg_traits::Identity;
use std::ops::{Range, RangeBounds};

#[derive(Debug, Clone, PartialEq)]
pub struct Segtree<T: Identity> {
    len: usize,
    table: Vec<T::Value>,
}
impl<T: Identity> Segtree<T> {
    pub fn from_slice(src: &[T::Value]) -> Self {
        let mut table = src.iter().chain(src.iter()).cloned().collect::<Vec<_>>();
        let len = src.len();
        for i in (1..len).rev() {
            table[i] = T::op(table[2 * i].clone(), table[2 * i + 1].clone())
        }
        Segtree { len, table }
    }
    pub fn set(&mut self, mut i: usize, x: T::Value) {
        i += self.len;
        self.table[i] = x;
        i >>= 1;
        while 0 != i {
            self.update(i);
            i >>= 1;
        }
    }

    pub fn fold(&self, range: impl RangeBounds<usize>) -> T::Value {
        let Range { mut start, mut end } = open(self.len, range);
        start += self.len;
        end += self.len;
        let mut left = T::identity();
        let mut right = T::identity();
        while start != end {
            if start % 2 == 1 {
                T::op_left(&mut left, self.table[start].clone());
                start += 1;
            }
            if end % 2 == 1 {
                end -= 1;
                T::op_right(self.table[end].clone(), &mut right);
            }
            start >>= 1;
            end >>= 1;
        }
        T::op(left, right)
    }

    pub fn search_forward(
        &self,
        range: impl RangeBounds<usize>,
        mut pred: impl FnMut(&T::Value) -> bool,
    ) -> usize {
        let Range { mut start, mut end } = open(self.len, range);
        if start == end {
            start
        } else {
            start += self.len;
            end += self.len;
            let orig_end = end;
            let mut crr = T::identity();
            let mut shift = 0;
            while start != end {
                if start % 2 == 1 {
                    let nxt = T::op(crr.clone(), self.table[start].clone());
                    if !pred(&nxt) {
                        return self.search_subtree_forward(crr, start, pred);
                    }
                    crr = nxt;
                    start += 1;
                }
                start >>= 1;
                end >>= 1;
                shift += 1;
            }
            for p in (0..shift).rev() {
                let end = (orig_end >> p) - 1;
                if end % 2 == 0 {
                    let nxt = T::op(crr.clone(), self.table[end].clone());
                    if !pred(&nxt) {
                        return self.search_subtree_forward(crr, end, pred);
                    }
                    crr = nxt;
                }
            }
            orig_end - self.len
        }
    }

    pub fn search_backward(
        &self,
        range: impl RangeBounds<usize>,
        mut pred: impl FnMut(&T::Value) -> bool,
    ) -> usize {
        let Range { mut start, mut end } = open(self.len, range);
        if start == end {
            start
        } else {
            start += self.len;
            end += self.len;
            let orig_start_m1 = start - 1;
            let mut crr = T::identity();
            let mut shift = 0;
            while start != end {
                if end % 2 == 1 {
                    end -= 1;
                    let nxt = T::op(self.table[end].clone(), crr.clone());
                    if !pred(&nxt) {
                        return self.search_subtree_backward(crr, end, pred);
                    }
                    crr = nxt;
                }
                start = (start + 1) >> 1;
                end >>= 1;
                shift += 1;
            }
            for p in (0..shift).rev() {
                let start = (orig_start_m1 >> p) + 1;
                if start % 2 == 1 {
                    let nxt = T::op(self.table[start].clone(), crr.clone());
                    if !pred(&nxt) {
                        return self.search_subtree_backward(crr, start, pred);
                    }
                    crr = nxt;
                }
            }
            orig_start_m1 + 1 - self.len
        }
    }

    fn update(&mut self, i: usize) {
        self.table[i] = T::op(self.table[2 * i].clone(), self.table[2 * i + 1].clone())
    }
    fn search_subtree_forward(
        &self,
        mut crr: T::Value,
        mut root: usize,
        mut pred: impl FnMut(&T::Value) -> bool,
    ) -> usize {
        while root < self.len {
            let nxt = T::op(crr.clone(), self.table[root * 2].clone());
            root = if pred(&nxt) {
                crr = nxt;
                root * 2 + 1
            } else {
                root * 2
            };
        }
        root - self.len
    }
    fn search_subtree_backward(
        &self,
        mut crr: T::Value,
        mut root: usize,
        mut pred: impl FnMut(&T::Value) -> bool,
    ) -> usize {
        while root < self.len {
            let nxt = T::op(self.table[root * 2 + 1].clone(), crr.clone());
            root = if pred(&nxt) {
                crr = nxt;
                root * 2
            } else {
                root * 2 + 1
            };
        }
        root + 1 - self.len
    }
}

fn open(len: usize, range: impl RangeBounds<usize>) -> Range<usize> {
    use std::ops::Bound::*;
    (match range.start_bound() {
        Unbounded => 0,
        Included(&x) => x,
        Excluded(&x) => x + 1,
    })..(match range.end_bound() {
        Excluded(&x) => x,
        Included(&x) => x + 1,
        Unbounded => len,
    })
}

#[cfg(test)]
mod tests {
    mod impl_query;
    use alg_inversion_number::{InversionMerge, InversionValue};
    use rand::prelude::*;
    use test_vector2::{queries, Vector};

    type Tester<T, G> = query_test::Tester<StdRng, Vector<T>, crate::Segtree<T>, G>;

    #[test]
    fn test_add_u32() {
        use alg_traits::arith::Add;
        struct G {}
        impl test_vector2::GenLen for G {
            fn gen_len(rng: &mut impl Rng) -> usize {
                rng.gen_range(1, 20)
            }
        }
        impl test_vector2::GenValue<u32> for G {
            fn gen_value(rng: &mut impl Rng) -> u32 {
                rng.gen_range(0, 20)
            }
        }
        impl test_vector2::GenKey<u32> for G {
            fn gen_key(rng: &mut impl Rng) -> u32 {
                rng.gen_range(0, 100)
            }
        }

        struct P {}
        impl queries::Pred<u32, u32> for P {
            fn pred(x: &u32, y: &u32) -> bool {
                x <= y
            }
        }

        let mut tester = Tester::<Add<u32>, G>::new(StdRng::seed_from_u64(42));
        for _ in 0..4 {
            tester.initialize();
            for _ in 0..100 {
                let command = tester.rng_mut().gen_range(0, 4);
                match command {
                    0 => tester.mutate::<queries::Set<_>>(),
                    1 => tester.compare::<queries::Fold<_>>(),
                    2 => tester.judge::<queries::SearchForward<_, _, P>>(),
                    3 => tester.judge::<queries::SearchBackward<_, _, P>>(),
                    _ => unreachable!(),
                }
            }
        }
    }

    #[test]
    fn test_inversion_value() {
        type Value = InversionValue;
        struct G {}
        impl test_vector2::GenLen for G {
            fn gen_len(rng: &mut impl Rng) -> usize {
                rng.gen_range(1, 20)
            }
        }
        impl test_vector2::GenValue<Value> for G {
            fn gen_value(rng: &mut impl Rng) -> Value {
                InversionValue::from_bool(rng.gen_ratio(1, 2))
            }
        }
        impl test_vector2::GenKey<u64> for G {
            fn gen_key(rng: &mut impl Rng) -> u64 {
                rng.gen_range(0, 20)
            }
        }
        struct P {}
        impl queries::Pred<Value, u64> for P {
            fn pred(x: &Value, y: &u64) -> bool {
                x.inversion < *y
            }
        }

        let mut tester = Tester::<InversionMerge, G>::new(StdRng::seed_from_u64(42));
        for _ in 0..4 {
            tester.initialize();
            for _ in 0..100 {
                let command = tester.rng_mut().gen_range(0, 4);
                match command {
                    0 => tester.mutate::<queries::Set<_>>(),
                    1 => tester.compare::<queries::Fold<_>>(),
                    2 => tester.judge::<queries::SearchForward<_, _, P>>(),
                    3 => tester.judge::<queries::SearchBackward<_, _, P>>(),
                    _ => unreachable!(),
                }
            }
        }
    }
}
