use crate::types::integer::Integer;

pub fn gcd<T: Integer>(a: T, b: T) -> T {
    if a == T::from(0) {
        b
    } else {
        gcd(b % a, a)
    }
}

pub struct ExtendedGcdResult {
    pub gcd: i64,
    pub x: i64,
    pub y: i64,
}

/// a*x + b*y = gcd(a, b)
pub fn extended_gcd(a: i64, b: i64) -> ExtendedGcdResult {
    if a == 0 {
        return ExtendedGcdResult { gcd: b, x: 0, y: 1 };
    }
    let prev = extended_gcd(b % a, a);
    ExtendedGcdResult {
        gcd: prev.gcd,
        x: prev.y - (b / a) * prev.x,
        y: prev.x,
    }
}

/// Finds (x, y) solution for a*x + b*y = c
pub fn find_integer_point(a: i64, b: i64, c: i64) -> Option<(i64, i64)> {
    if a == 0 && b == 0 {
        return if c == 0 { Some((0, 0)) } else { None };
    }
    let ExtendedGcdResult { gcd, x, y } = extended_gcd(a, b);
    if c % gcd != 0 {
        return None;
    }
    let d = c / gcd;
    Some((x * d, y * d))
}
