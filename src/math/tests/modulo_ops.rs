use crate::math::modulo_ops::{modulo_combinations, modulo_inv, modulo_pow, Factorials};

const MOD: u64 = 10u64.pow(9) + 7;

#[test]
fn modulo_pow_simple() {
    let v = 2u64;
    for exp in 0..50 {
        let actual = modulo_pow(v, exp, MOD);
        let expected = v.pow(exp as u32) % MOD;
        assert_eq!(actual, expected, "exp = {exp}");
    }
}

#[test]
fn modulo_inv_simple() {
    for v in 1..100 {
        let actual = modulo_inv(v, MOD);
        assert_eq!((actual * v) % MOD, 1, "v = {v}, inv = {actual}");
    }
}

#[test]
fn modulo_combinations_simple() {
    for n in 0..=10 {
        for k in 0..=n {
            check(n, k);
        }
    }
    fn check(n: usize, k: usize) {
        let fact = |n: usize| (1..=n).fold(1, |acc, v| acc * v as u64);
        let expected = fact(n) / (fact(k) * fact(n - k));
        let actual = modulo_combinations(&Factorials::new(n, 998244353), n, k);
        assert_eq!(actual, expected, "n = {n}, k = {k}");
    }
}
