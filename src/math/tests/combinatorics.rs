use crate::math::combinatorics::{generate_combinations, next_permutation};
use crate::plat::classic::includes::{IntoVecExt, Str};

#[test]
fn generate_combinations_basic() {
    check_combinations(2, 0, &["00"]);
    check_combinations(2, 2, &["11"]);
    check_combinations(4, 2, &["1100", "1010", "1001", "0110", "0101", "0011"]);
}

#[track_caller]
fn check_combinations(n: usize, k: usize, expected: &[&str]) {
    assert_eq!(
        generate_combinations(n, k),
        expected
            .iter()
            .map(|s| s.chars().map(|c| c == '1').into_vec())
            .into_vec(),
        "n = {n}, k = {k}"
    )
}

#[test]
fn next_permutation_basic() {
    check_no_next_permutation("");
    check_no_next_permutation("1");
    check_permutations_chain(&["12", "21"]);
    check_permutations_chain(&["123", "132", "213", "231", "312", "321"]);
}

#[test]
fn next_permutation_big() {
    let mut prev = Str::from("12345");
    let mut tot = 1;
    loop {
        let mut nxt = prev.clone();
        if !next_permutation(&mut nxt) {
            break;
        }
        assert!(prev < nxt, "next permutation of {prev} cannot be {nxt}");
        prev = nxt;
        tot += 1;
    }
    assert_eq!(tot, 5 * 4 * 3 * 2 * 1, "missed some permutations");
}

#[track_caller]
fn check_no_next_permutation(s: &str) {
    let mut actual = Str::from(s);
    let ret = next_permutation(&mut actual);
    assert_eq!(
        ret, false,
        "expected no next permutation for {s}, got {actual}"
    );
}

#[track_caller]
fn check_next_permutation(s: &str, expected: &str) {
    let mut actual = Str::from(s);
    assert!(
        next_permutation(&mut actual),
        "failed to generate next permutation for {}",
        s
    );
    assert_eq!(
        actual,
        Str::from(expected),
        "next permutation doesn't match"
    )
}

#[track_caller]
fn check_permutations_chain(chain: &[&str]) {
    for (i, _) in chain.iter().enumerate().skip(1) {
        check_next_permutation(chain[i - 1], chain[i]);
    }
    check_no_next_permutation(chain.last().unwrap());
}
