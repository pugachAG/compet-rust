use std::collections::HashSet;

use crate::math::combinations::combinations;
use crate::math::combinatorics::factorial;

#[test]
fn combinations_basic() {
    let mut actual = Vec::new();
    let mut combs = combinations(3, 2);
    while let Some(comb) = combs.next() {
        actual.push(comb.to_vec());
    }
    assert_eq!(actual, vec![vec![0, 1], vec![0, 2], vec![1, 2]]);
}

#[test]
fn combinations_big() {
    let n = 10;
    let k = 4;
    let mut all = HashSet::new();
    let mut combs = combinations(n, k);
    while let Some(comb) = combs.next() {
        assert_eq!(comb.len(), k);
        for i in 1..k {
            assert!(comb[i] > comb[i - 1]);
        }
        assert!(comb[k - 1] < n);
        all.insert(comb.to_vec());
    }
    assert_eq!(all.len(), factorial(n) / (factorial(k) * factorial(n - k)))
}
