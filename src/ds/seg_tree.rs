use std::ops::{Range, RangeBounds};

use crate::utils::misc::{assert_range, unpack_range};

/// See AtCoder library for more context:
/// https://atcoder.github.io/ac-library/production/document_en/segtree.html
///
/// # Example
/// ```
/// #[derive(Copy, Clone)]
/// struct SegTreeSum(i32);
///
/// impl SegTreeValue for SegTreeSum {
///     fn op(l: Self, r: Self) -> Self {
///         Self(l.0 + r.0)
///     }
///     
///     fn e() -> Self {
///         Self(0)
///     }
/// }
/// ```
pub trait SegTreeValue: Copy {
    fn op(l: Self, r: Self) -> Self;

    /// Identity element: op(x, e()) = op(e(), x) = x
    fn e() -> Self;
}

pub struct SegTree<T> {
    n: usize,
    st: Vec<T>,
}

impl<T: SegTreeValue> SegTree<T> {
    pub fn with_len(n: usize) -> Self {
        Self {
            n,
            st: vec![T::e(); seg_tree_st_len(n)],
        }
    }

    pub fn new(a: &[T]) -> Self {
        let n = a.len();
        let mut ret = Self::with_len(n);
        if n > 0 {
            ret.build(a, &ret.root());
        }
        ret
    }

    pub fn len(&self) -> usize {
        self.n
    }

    pub fn get(&self, rng_bounds: impl RangeBounds<usize>) -> T {
        let rng = unpack_range(rng_bounds);
        assert_range(&rng, 0..self.n, true);
        if rng.is_empty() {
            T::e()
        } else {
            self.calc(&rng, &self.root())
        }
    }

    pub fn set(&mut self, i: usize, val: T) {
        assert!(i < self.n);
        self.update(i, val, &self.root());
    }

    /// [1 1 1 1 0 0]
    ///  l     ^   r
    pub fn max_prefix<P>(
        &self,
        rng_bounds: impl RangeBounds<usize>,
        mut predicate: P,
    ) -> Option<usize>
    where
        P: FnMut(T) -> bool,
    {
        let rng = unpack_range(rng_bounds);
        assert_range(&rng, 0..self.n, true);
        if !rng.is_empty() {
            match self.find_max_prefix(&rng, &mut predicate, &self.root(), T::e()) {
                MaxPrefixOutput::All(_) => Some(rng.end - 1),
                MaxPrefixOutput::Index(i) => {
                    if i > rng.start {
                        Some(i - 1)
                    } else {
                        None
                    }
                }
            }
        } else {
            None
        }
    }

    fn update(&mut self, i: usize, val: T, pos: &Pos) {
        if pos.single_point().is_some() {
            self.st[pos.st_i] = val;
        } else {
            let (ref left, ref right) = pos.split();
            if left.contains(i) {
                self.update(i, val, left);
            } else {
                self.update(i, val, right);
            }
            self.recalc(pos, left, right);
        }
    }

    fn calc(&self, range: &Range<usize>, pos: &Pos) -> T {
        if pos.is_inside(range) {
            return self.st[pos.st_i];
        }
        let (ref left, ref right) = pos.split();
        if !right.intersects(range) {
            self.calc(range, left)
        } else if !left.intersects(range) {
            self.calc(range, right)
        } else {
            T::op(self.calc(range, left), self.calc(range, right))
        }
    }

    fn build(&mut self, a: &[T], pos: &Pos) {
        if let Some(i) = pos.single_point() {
            self.st[pos.st_i] = a[i];
        } else {
            let (ref left, ref right) = pos.split();
            self.build(a, left);
            self.build(a, right);
            self.recalc(pos, left, right);
        }
    }

    fn find_max_prefix<P>(
        &self,
        rng: &Range<usize>,
        predicate: &mut P,
        pos: &Pos,
        carryover: T,
    ) -> MaxPrefixOutput<T>
    where
        P: FnMut(T) -> bool,
    {
        if pos.is_inside(rng) {
            let v = T::op(carryover, self.st[pos.st_i]);
            if (*predicate)(v) {
                return MaxPrefixOutput::All(v);
            }
        }
        if let Some(i) = pos.single_point() {
            return MaxPrefixOutput::Index(i);
        }
        let (ref left, ref right) = pos.split();
        if left.intersects(rng) {
            match self.find_max_prefix(rng, predicate, left, carryover) {
                MaxPrefixOutput::All(v) => {
                    if right.intersects(rng) {
                        self.find_max_prefix(rng, predicate, right, v)
                    } else {
                        MaxPrefixOutput::Index(left.range.end)
                    }
                }
                index => index,
            }
        } else {
            self.find_max_prefix(rng, predicate, right, carryover)
        }
    }

    fn root(&self) -> Pos {
        Pos {
            st_i: 1,
            range: 0..self.n,
        }
    }

    fn recalc(&mut self, pos: &Pos, left: &Pos, right: &Pos) {
        self.st[pos.st_i] = T::op(self.st[left.st_i], self.st[right.st_i]);
    }
}

pub(super) fn seg_tree_st_len(n: usize) -> usize {
    let mut ret = 1;
    while ret < n {
        ret *= 2;
    }
    ret * 2
}

pub(super) struct Pos {
    pub st_i: usize,
    pub range: Range<usize>,
}

impl Pos {
    #[inline]
    pub fn is_inside(&self, range: &Range<usize>) -> bool {
        range.start <= self.range.start && range.end >= self.range.end
    }

    #[inline]
    pub fn intersects(&self, range: &Range<usize>) -> bool {
        self.range.start < range.end && range.start < self.range.end
    }

    #[inline]
    pub fn contains(&self, i: usize) -> bool {
        self.range.contains(&i)
    }

    #[inline]
    pub fn single_point(&self) -> Option<usize> {
        if self.range.len() == 1 {
            Some(self.range.start)
        } else {
            None
        }
    }

    pub fn split(&self) -> (Pos, Pos) {
        let Pos {
            st_i,
            range: Range { start, end },
        } = *self;
        let mid = (start + end) / 2;
        (
            Pos {
                st_i: 2 * st_i,
                range: start..mid,
            },
            Pos {
                st_i: 2 * st_i + 1,
                range: mid..end,
            },
        )
    }
}

enum MaxPrefixOutput<T> {
    All(T),
    Index(usize),
}
