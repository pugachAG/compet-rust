use std::collections::BTreeSet;

use crate::algo::knapsack::knapsack_all;

#[test]
pub fn knapsack_basic() {
    check_knapsack_all(&[], &[0]);
    check_knapsack_all(&[1], &[0, 1]);
    check_knapsack_all(&[1, 3], &[0, 1, 3, 4]);
    check_knapsack_all(&[1, 2, 5], &[0, 1, 2, 3, 5, 6, 7, 8]);
    check_knapsack_all(&[2, 3, 2], &[0, 2, 3, 4, 5, 7]);
}

#[track_caller]
fn check_knapsack_all(a: &[usize], expected: &[usize]) {
    let expected: BTreeSet<usize> = expected.iter().cloned().collect();
    let actual: BTreeSet<usize> = knapsack_all(a, None)
        .bits()
        .enumerate()
        .filter(|p| p.1)
        .map(|p| p.0)
        .collect();
    assert_eq!(actual, expected);
}
