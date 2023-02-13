use std::ops::AddAssign;

use crate::utils::collections::{def_vec, IntoVecExt};

pub fn coordinate_compress<T: Ord>(a: &[T]) -> Vec<usize> {
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
