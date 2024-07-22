use std::collections::{BTreeMap, BTreeSet};
use std::iter::FromIterator;
use std::ops::{AddAssign, RangeBounds, Sub};

use crate::types::integer::Integer;
use crate::utils::collections::{def_vec, IntoVecExt};
use crate::utils::misc::{assert_range, unpack_range};

pub fn div_up<T: Integer>(v: T, d: T) -> T {
    (v + d - T::from(1)) / d
}

pub fn permutation_index(a: &[usize]) -> Vec<usize> {
    let n = a.len();
    let mut ans = vec![n; n];
    for (i, &v) in a.iter().enumerate() {
        assert!(v < n && ans[v] == n, "not a permutation");
        ans[v] = i;
    }
    ans
}

pub fn coordinate_compress_map<T: Ord>(iter: impl Iterator<Item = T>) -> BTreeMap<T, usize> {
    BTreeMap::from_iter(
        BTreeSet::from_iter(iter)
            .into_iter()
            .enumerate()
            .map(|(i, v)| (v, i)),
    )
}

pub fn coordinate_compress_unique<T: Ord>(a: &[T]) -> Vec<usize> {
    let n = a.len();
    let mut indices = (0..n).into_vec();
    indices.sort_by_key(|&i| &a[i]);
    let mut ret = def_vec(n);
    for (j, &i) in indices.iter().enumerate() {
        ret[i] = j;
    }
    ret
}

pub fn prefix_sum<T: AddAssign<T> + From<u8> + Copy>(a: &[T], prepend_zero: bool) -> Vec<T> {
    let n = a.len();
    let mut ret = Vec::with_capacity(n + (if prepend_zero { 1 } else { 0 }));
    let mut cur = T::from(0);
    if prepend_zero {
        ret.push(cur);
    }
    for &v in a {
        cur += v;
        ret.push(cur);
    }
    ret
}

pub struct SliceRangeSum<T> {
    pref: Vec<T>,
}

impl<T: AddAssign<T> + Sub<Output = T> + From<u8> + Copy> SliceRangeSum<T> {
    pub fn new(a: &[T]) -> Self {
        Self {
            pref: prefix_sum(a, true),
        }
    }

    pub fn sum(&self, rng_bounds: impl RangeBounds<usize>) -> T {
        let n = self.pref.len() - 1;
        let rng = unpack_range(rng_bounds);
        assert_range(&rng, 0..n, true);
        self.pref[rng.end] - self.pref[rng.start]
    }
}
