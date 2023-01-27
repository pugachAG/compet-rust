use crate::math::modulo_ops::{modulo_inv, modulo_pow};

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
