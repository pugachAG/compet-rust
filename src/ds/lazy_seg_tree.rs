use std::mem;
use std::ops::{Range, RangeBounds};

use crate::utils::misc::{assert_range, unpack_range};

use super::seg_tree::{seg_tree_st_len, Pos, SegTreeValue};

pub trait LazySegTreeUpdate<V: SegTreeValue>: Copy {
    fn apply(upd: Self, val: V) -> V;
    fn combine(upd: Self, other: Self) -> Self;
    fn id() -> Self;
}

type Entry<V, U> = (V, U);

pub struct LazySegTree<V, U> {
    n: usize,
    st: Vec<Entry<V, U>>,
}

impl<V: SegTreeValue, U: LazySegTreeUpdate<V>> LazySegTree<V, U> {
    pub fn with_size(n: usize) -> Self {
        Self {
            n,
            st: vec![(V::e(), U::id()); seg_tree_st_len(n)],
        }
    }

    pub fn new(a: &[V]) -> Self {
        let n = a.len();
        let mut ret = Self::with_size(n);
        if n > 0 {
            ret.build(a, &ret.root());
        }
        ret
    }

    pub fn get(&mut self, rng_bounds: impl RangeBounds<usize>) -> V {
        let rng = unpack_range(rng_bounds);
        assert_range(&rng, 0..self.n, true);
        if rng.is_empty() {
            V::e()
        } else {
            self.calc(&rng, &self.root())
        }
    }

    pub fn apply(&mut self, rng_bounds: impl RangeBounds<usize>, upd: U) {
        let rng = unpack_range(rng_bounds);
        assert_range(&rng, 0..self.n, true);
        if !rng.is_empty() {
            self.update(&rng, upd, &self.root());
        }
    }

    fn calc(&mut self, range: &Range<usize>, pos: &Pos) -> V {
        if pos.is_inside(range) {
            return self.at(pos);
        }
        let (ref left, ref right) = pos.split();
        self.push_down(pos, left, right);
        if !right.intersects(range) {
            self.calc(range, left)
        } else if !left.intersects(range) {
            self.calc(range, right)
        } else {
            V::op(self.calc(range, left), self.calc(range, right))
        }
    }

    fn at(&self, pos: &Pos) -> V {
        let (val, upd) = self.st[pos.st_i];
        return U::apply(upd, val);
    }

    fn update(&mut self, range: &Range<usize>, upd: U, pos: &Pos) {
        if !pos.intersects(range) {
            return;
        }
        if pos.is_inside(range) {
            self.combine_update(pos, upd);
        } else {
            let (ref left, ref right) = pos.split();
            self.push_down(pos, left, right);
            self.update(range, upd, left);
            self.update(range, upd, right);
            self.recalc(pos, left, right);
        }
    }

    fn push_down(&mut self, pos: &Pos, left: &Pos, right: &Pos) {
        let new_val = self.at(pos);
        let (_, upd) = mem::replace(&mut self.st[pos.st_i], (new_val, U::id()));
        self.combine_update(left, upd);
        self.combine_update(right, upd);
    }

    fn combine_update(&mut self, pos: &Pos, upd: U) {
        let cur = &mut self.st[pos.st_i].1;
        *cur = U::combine(*cur, upd);
    }

    fn build(&mut self, a: &[V], pos: &Pos) {
        if let Some(i) = pos.single_point() {
            self.st[pos.st_i].0 = a[i];
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
        self.st[pos.st_i] = (V::op(self.at(left), self.at(right)), U::id());
    }
}
