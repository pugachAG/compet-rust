use crate::math::combinatorics::generate_combinations;
use crate::plat::classic::includes::IntoVecExt;

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
