use crate::math::combinatorics::{combinations_count, factorial};

#[test]
fn factorial_basic() {
    assert_eq!(factorial(0), 1);
    assert_eq!(factorial(1), 1);
    assert_eq!(factorial(5), 120);
}

#[test]
fn combinations_count_basic() {
    assert_eq!(combinations_count(27, 9), 4686825);
    assert_eq!(combinations_count(27, 18), 4686825);
}
