use std::ops::Bound;
use std::ops::Range;
use std::ops::RangeBounds;
use std::ops::RangeInclusive;

use crate::algo::bin_search::bin_search_range;
use crate::plat::classic::includes::IntoVecExt;

use crate::algo::bin_search::BinSearch;

#[test]
fn bin_search_range_doc() {
    assert_eq!(bin_search_range(&[1, 2, 2, 3], 2..3), 1..3);
}

#[test]
fn bin_search_range_comprehensive() {
    const MIN_VAL: i32 = 1;
    const MAX_VAL: i32 = 4;
    let a = (MIN_VAL..=MAX_VAL).flat_map(|v| vec![v; 3]).into_vec();
    let n = a.len();
    for l in 0..n {
        for r in l..n {
            let cur = &a[l..r];
            for start in gen_bounds(MIN_VAL - 1..=MAX_VAL + 1) {
                for end in gen_bounds(MIN_VAL - 1..=MAX_VAL + 1) {
                    let range = (start, end);
                    let actual = bin_search_range(cur, range);
                    let expected = bin_search_range_naive(cur, range);
                    assert_eq!(actual, expected, "bin_search_range({cur:?}, {range:?})");
                }
            }
        }
    }
}

fn gen_bounds(range: RangeInclusive<i32>) -> Vec<Bound<i32>> {
    let mut ret = vec![Bound::Unbounded];
    for v in range {
        ret.push(Bound::Included(v));
        ret.push(Bound::Excluded(v));
    }
    ret
}

fn bin_search_range_naive<B: RangeBounds<i32>>(a: &[i32], range: B) -> Range<usize> {
    let n = a.len();
    let mut ret = n..n;
    for (i, &u) in a.iter().enumerate() {
        let ok_start = match range.start_bound() {
            Bound::Included(&v) => u >= v,
            Bound::Excluded(&v) => u > v,
            Bound::Unbounded => true,
        };
        if ok_start && ret.start == n {
            ret = i..i;
        }
        if range.contains(&u) {
            ret.end = i + 1;
        }
    }
    ret
}

#[test]
fn bin_search_doc() {
    fn isqrt(x: u32) -> u32 {
        let mut bs = BinSearch::prefix(0..=x);
        while let Some(token) = bs.next() {
            let ok = token.val().pow(2) <= x;
            token.update(ok);
        }
        bs.answer().unwrap()
    }
    assert_eq!(isqrt(30), 5);
}

#[test]
fn bin_search_sorted_vec() {
    const MIN_VAL: i32 = 1;
    const MAX_VAL: i32 = 4;
    let a = (MIN_VAL..=MAX_VAL).flat_map(|v| vec![v; 3]).into_vec();
    let n = a.len();
    for l in 0..n {
        for r in l..n {
            for v in MIN_VAL - 1..=MAX_VAL + 1 {
                check_last_le(&a, l, r, v);
                check_first_gt(&a, l, r, v);
            }
        }
    }
}

fn check_last_le(a: &[i32], l: usize, r: usize, v: i32) {
    let expected = (l..=r).filter(|&i| a[i] <= v).last();
    let actual = bin_search_last_le(a, l, r, v);
    check_index(actual, expected, a, l, r, v);
}

fn check_first_gt(a: &[i32], l: usize, r: usize, v: i32) {
    let expected = (l..=r).filter(|&i| a[i] > v).next();
    let actual = bin_search_first_gt(a, l, r, v);
    check_index(actual, expected, a, l, r, v);
}

#[track_caller]
fn check_index(
    actual: Option<usize>,
    expected: Option<usize>,
    a: &[i32],
    l: usize,
    r: usize,
    v: i32,
) {
    assert_eq!(actual, expected, "[{l}, {r}] {v} in {:?}", &a[l..=r]);
}

fn bin_search_last_le(a: &[i32], l: usize, r: usize, v: i32) -> Option<usize> {
    let mut bs = BinSearch::prefix(l..=r);
    let mut iterations = 0;
    while let Some(token) = bs.next() {
        let i = token.val();
        token.update(a[i] <= v);
        iterations += 1;
        assert!(iterations <= 40, "too many iterations");
    }
    bs.answer()
}

fn bin_search_first_gt(a: &[i32], l: usize, r: usize, v: i32) -> Option<usize> {
    let mut bs = BinSearch::suffix(l..=r);
    let mut iterations = 0;
    while let Some(token) = bs.next() {
        let i = token.val();
        token.update(a[i] > v);
        iterations += 1;
        assert!(iterations <= 40, "too many iterations");
    }
    bs.answer()
}
