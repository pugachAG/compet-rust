use crate::math::combinatorics::factorial;

#[test]
fn factorial_basic() {
    assert_eq!(factorial(0), 1);
    assert_eq!(factorial(1), 1);
    assert_eq!(factorial(5), 120);
}
