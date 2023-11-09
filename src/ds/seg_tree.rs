use std::ops::{Range, RangeBounds};

use crate::utils::misc::{assert_range, unpack_range};

/// # Example
/// ```
/// #[derive(Copy, Clone)]
/// struct SegTreeSum(i32);
///
/// impl SegTreeMonoid for SegTreeSum {
///     fn op(l: Self, r: Self) -> Self {
///         Self(l.0 + r.0)
///     }
///     
///     fn e() -> Self {
///         Self(0)
///     }
/// }
/// ```
pub trait SegTreeMonoid: Copy {
    fn op(l: Self, r: Self) -> Self;

    /// Identity element: op(x, e()) = op(e(), x) = x
    fn e() -> Self;
}

pub struct SegTree<T> {
    n: usize,
    st: Vec<T>,
}

impl<T: SegTreeMonoid> SegTree<T> {
    pub fn with_size(n: usize) -> Self {
        Self {
            n,
            st: vec![T::e(); Self::st_len(n)],
        }
    }

    pub fn new(a: &[T]) -> Self {
        let n = a.len();
        let mut ret = Self::with_size(n);
        if n > 0 {
            ret.build(a, &ret.root());
        }
        ret
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

    fn root(&self) -> Pos {
        Pos {
            st_i: 1,
            range: 0..self.n,
        }
    }

    fn recalc(&mut self, pos: &Pos, left: &Pos, right: &Pos) {
        self.st[pos.st_i] = T::op(self.st[left.st_i], self.st[right.st_i]);
    }

    fn st_len(n: usize) -> usize {
        let mut ret = 1;
        while ret < n {
            ret *= 2;
        }
        ret * 2
    }
}

struct Pos {
    st_i: usize,
    range: Range<usize>,
}

impl Pos {
    #[inline]
    fn is_inside(&self, range: &Range<usize>) -> bool {
        range.start <= self.range.start && range.end >= self.range.end
    }

    #[inline]
    fn intersects(&self, range: &Range<usize>) -> bool {
        self.range.start < range.end && range.start < self.range.end
    }

    #[inline]
    fn contains(&self, i: usize) -> bool {
        self.range.contains(&i)
    }

    #[inline]
    fn single_point(&self) -> Option<usize> {
        if self.range.len() == 1 {
            Some(self.range.start)
        } else {
            None
        }
    }

    fn split(&self) -> (Pos, Pos) {
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
