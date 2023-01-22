/// https://github.com/python/cpython/blob/main/Modules/mathmodule.c#L1793
pub fn isqrt(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }
    let c = (num_bits(n) - 1) / 2;
    let shift = 31 - c;
    let u = approximate_isqrt(n << (2 * shift)) >> shift;
    if u * u > n {
        u - 1
    } else {
        u
    }
}

pub fn is_perfect_square(n: u64) -> bool {
    let sq = isqrt(n);
    sq * sq == n
}

#[inline]
fn num_bits(n: u64) -> usize {
    std::mem::size_of::<u64>() * 8 - n.leading_zeros() as usize
}

#[inline]
fn approximate_isqrt(n: u64) -> u64 {
    const APPROXIMATE_ISQRT_TAB: [u64; 192] = [
        128, 129, 130, 131, 132, 133, 134, 135, 136, 137, 138, 139, 140, 141, 142, 143, 144, 144,
        145, 146, 147, 148, 149, 150, 151, 151, 152, 153, 154, 155, 156, 156, 157, 158, 159, 160,
        160, 161, 162, 163, 164, 164, 165, 166, 167, 167, 168, 169, 170, 170, 171, 172, 173, 173,
        174, 175, 176, 176, 177, 178, 179, 179, 180, 181, 181, 182, 183, 183, 184, 185, 186, 186,
        187, 188, 188, 189, 190, 190, 191, 192, 192, 193, 194, 194, 195, 196, 196, 197, 198, 198,
        199, 200, 200, 201, 201, 202, 203, 203, 204, 205, 205, 206, 206, 207, 208, 208, 209, 210,
        210, 211, 211, 212, 213, 213, 214, 214, 215, 216, 216, 217, 217, 218, 219, 219, 220, 220,
        221, 221, 222, 223, 223, 224, 224, 225, 225, 226, 227, 227, 228, 228, 229, 229, 230, 230,
        231, 232, 232, 233, 233, 234, 234, 235, 235, 236, 237, 237, 238, 238, 239, 239, 240, 240,
        241, 241, 242, 242, 243, 243, 244, 244, 245, 246, 246, 247, 247, 248, 248, 249, 249, 250,
        250, 251, 251, 252, 252, 253, 253, 254, 254, 255, 255, 255,
    ];
    let mut u = APPROXIMATE_ISQRT_TAB[((n >> 56) - 64) as usize];
    u = (u << 7) + (n >> 41) / u;
    u.wrapping_shl(15) + (n >> 17) / u
}

#[cfg(test)]
mod tests {
    use super::{isqrt, is_perfect_square};

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

    fn check_sqrt(v: u64) {
        let actual = isqrt(v);
        let sq = actual.checked_mul(actual).expect("overflow");
        assert!(sq <= v, "{v}: {actual} is too big");
        if let Some(sq_nxt) = (actual + 1).checked_mul(actual + 1) {
            assert!(sq_nxt > v, "{v}: {actual} is too small");
        }
    }
}