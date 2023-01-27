use crate::math::sqrt::{is_perfect_square, isqrt};

#[test]
fn test_isqrt_small_numbers() {
    for v in 0..=1000000u64 {
        check_sqrt(v);
    }
}

#[test]
fn test_isqrt_large_numbers() {
    for shift in 0..10 {
        for v in 0..=100000u64 {
            check_sqrt((u64::MAX >> shift) - v);
        }
    }
}

#[test]
fn test_isqrt_perfect_squares() {
    const LARGE: u64 = 10u64.pow(9);
    for v in 0..=100000u64 {
        check_sqrt(v.pow(2));
        check_sqrt((LARGE - v).pow(2));
    }
}

#[test]
fn test_is_prefect_square() {
    assert!(is_perfect_square(0));
    assert!(is_perfect_square(1));
    assert!(!is_perfect_square(2));
    assert!(is_perfect_square(4));
    assert!(is_perfect_square(100));
    assert!(!is_perfect_square(101));
}

#[track_caller]
fn check_sqrt(v: u64) {
    let actual = isqrt(v);
    let sq = actual.checked_mul(actual).expect("overflow");
    assert!(sq <= v, "{v}: {actual} is too big");
    if let Some(sq_nxt) = (actual + 1).checked_mul(actual + 1) {
        assert!(sq_nxt > v, "{v}: {actual} is too small");
    }
}
