use crate::plat::classic::includes::IntoVecExt;

use crate::algo::bin_search::BinSearch;

#[test]
fn test_bin_search() {
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
    let mut bs = BinSearch::new(l..=r);
    let mut iterations = 0;
    while let Some(token) = bs.next() {
        let i = token.val();
        if a[i] <= v {
            token.update_left(true);
        } else {
            token.update_right(false);
        }
        iterations += 1;
        assert!(iterations <= 40, "too many iterations");
    }
    bs.answer()
}

fn bin_search_first_gt(a: &[i32], l: usize, r: usize, v: i32) -> Option<usize> {
    let mut bs = BinSearch::new(l..=r);
    let mut iterations = 0;
    while let Some(token) = bs.next() {
        let i = token.val();
        if a[i] > v {
            token.update_right(true);
        } else {
            token.update_left(false);
        }
        iterations += 1;
        assert!(iterations <= 40, "too many iterations");
    }
    bs.answer()
}
