pub fn modulo_pow(v: u64, exp: u64, modulo: u64) -> u64 {
    let mut ans = 1;
    let mut r = exp;
    let mut cur = v;
    while r > 0 {
        if (r & 1) == 1 {
            ans = (ans * cur) % modulo;
        }
        cur = (cur * cur) % modulo;
        r >>= 1;
    }
    ans
}

pub fn modulo_inv(v: u64, modulo: u64) -> u64 {
    assert!(v != 0, "can't calculate inverse of 0");
    modulo_pow(v, modulo - 2, modulo)
}

#[cfg(test)]
mod tests {
    use super::{modulo_inv, modulo_pow};

    const MOD: u64 = 10u64.pow(9) + 7;

    #[test]
    fn test_modulo_pow() {
        let v = 2u64;
        for exp in 0..50 {
            let actual = modulo_pow(v, exp, MOD);
            let expected = v.pow(exp as u32) % MOD;
            assert_eq!(actual, expected, "exp = {exp}");
        }
    }

    #[test]
    fn test_modulo_inv() {
        for v in 1..100 {
            let actual = modulo_inv(v, MOD);
            assert_eq!((actual * v) % MOD, 1, "v = {v}, inv = {actual}");
        }
    }
}
