use std::fmt::{Debug, Write};
use std::ops::ShlAssign;

use crate::ds::bitset::Bitset;
use crate::plat::classic::includes::IntoVecExt;
use crate::utils::rand::Random;

#[test]
fn bitset_set_random() {
    for n in 1..=10 {
        test_set_random(1, 2 * n);
    }
    test_set_random(64, 200);
    test_set_random(65, 200);
    test_set_random(64 * 3, 1000);
    test_set_random(150, 1000);
}

#[test]
fn bitset_set_all() {
    for n in 1..=128 {
        let mut actual = Bitset::new(n);
        let mut expected = NaiveBitset::new(n);
        for v in [true, false] {
            actual.set_all(v);
            expected.set_all(v);
            assert_bitset_eq(&actual, &expected);
        }
    }
}

#[test]
fn bitset_shl_random() {
    test_shl_random(1, 10);
    test_shl_random(2, 50);
    test_shl_random(10, 1000);
    test_shl_random(64, 1000);
    test_shl_random(65, 1000);
    test_shl_random(64 * 3, 100);
    test_shl_random(150, 100);
}

#[track_caller]
fn test_set_random(n: usize, iters: usize) {
    let mut rand = Random::new(42);
    let mut actual = Bitset::new(n);
    let mut expected = NaiveBitset::new(n);
    for _ in 0..iters {
        let i = rand.gen_range(0..n);
        let v = rand.gen_range(0..=1) == 1;
        let before = expected.clone();
        actual.set(i, v);
        expected.set(i, v);
        assert_bitset_op(&actual, &expected, &before, format!("set({i}, {v})"));
    }
}

#[track_caller]
fn test_shl_random(n: usize, iters: usize) {
    let mut rand = Random::new(42);
    let mut all_bits = (0..n).into_vec();
    for _ in 0..iters {
        let one_cnt = rand.gen_range(0..=n);
        rand.shuffle(&mut all_bits);
        let mut actual = Bitset::new(n);
        let mut expected = NaiveBitset::new(n);
        for &i in all_bits.iter().take(one_cnt) {
            actual.set(i, true);
            expected.set(i, true);
        }
        assert_bitset_eq(&actual, &expected);
        let before = expected.clone();
        let d = rand.gen_range(0..=n);
        actual <<= d;
        expected <<= d;
        assert_bitset_op(&actual, &expected, &before, format!("<<= {d}"));
    }
}

#[track_caller]
fn assert_bitset_op(actual: &Bitset, expected: &NaiveBitset, before: &NaiveBitset, op: String) {
    assert_eq!(
        format!("{actual:?}"),
        format!("{expected:?}"),
        "op: {op}, before: {before:?}"
    );
}

#[track_caller]
fn assert_bitset_eq(actual: &Bitset, expected: &NaiveBitset) {
    assert_eq!(format!("{actual:?}"), format!("{expected:?}"));
}

#[derive(Clone)]
struct NaiveBitset {
    bits: Vec<bool>,
}

impl NaiveBitset {
    pub fn new(n: usize) -> Self {
        Self {
            bits: vec![false; n],
        }
    }

    #[allow(dead_code)]
    pub fn get(&self, i: usize) -> bool {
        self.bits[i]
    }

    pub fn set(&mut self, i: usize, v: bool) {
        self.bits[i] = v;
    }

    pub fn set_all(&mut self, v: bool) {
        for bt in self.bits.iter_mut() {
            *bt = v;
        }
    }

    pub fn len(&self) -> usize {
        self.bits.len()
    }
}

impl ShlAssign<usize> for NaiveBitset {
    fn shl_assign(&mut self, d: usize) {
        for i in (0..self.len()).rev() {
            self.bits[i] = i.checked_sub(d).map(|j| self.bits[j]).unwrap_or(false);
        }
    }
}

impl Debug for NaiveBitset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for &bt in &self.bits {
            f.write_char(if bt { '1' } else { '0' })?
        }
        Ok(())
    }
}
