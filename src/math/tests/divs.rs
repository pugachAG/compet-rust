use crate::math::divs::{calc_all_divisors, calc_prime_divisors};
use crate::plat::classic::includes::IntoVecExt;
use crate::types::integer::Integer;

#[test]
fn test_calc_prime_divisors() {
    check_prime(1, &[]);
    check_prime(2, &[2]);
    check_prime::<u8>(8, &[2]);
    check_prime::<u32>(6, &[2, 3]);
    check_prime::<usize>(26, &[2, 13]);
}

#[test]
fn test_calc_all_divisors() {
    check_all(1, &[1]);
    check_all(2, &[1, 2]);
    check_all(4, &[1, 4, 2]);
    check_all::<usize>(36, &[1, 36, 2, 18, 3, 12, 4, 9, 6]);
}

fn check_prime<T: Integer>(v: T, expected: &[T]) {
    assert_eq!(
        calc_prime_divisors(v).into_vec(),
        expected.to_vec(),
        "{v:?}"
    );
}

fn check_all<T: Integer>(v: T, expected: &[T]) {
    assert_eq!(calc_all_divisors(v).into_vec(), expected.to_vec(), "{v:?}");
}
