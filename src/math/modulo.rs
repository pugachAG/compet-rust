use std::fmt::Debug;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
use std::str::FromStr;

use super::modulo_ops::{modulo_combinations, modulo_inv, modulo_pow, Factorials};

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct Modulo<const MOD: u64>(u64);

impl<const MOD: u64> Modulo<MOD> {
    pub const ZERO: Self = Modulo(0);
    pub const ONE: Self = Modulo(1);

    #[inline]
    pub fn new(v: u64) -> Self {
        Self(v % MOD)
    }

    pub fn combinatorics(n: usize) -> ModuloCombinatorics<MOD> {
        ModuloCombinatorics::new(n)
    }

    pub fn val(&self) -> u64 {
        self.0
    }

    pub fn pow(&self, exp: u64) -> Self {
        Self::new(modulo_pow(self.0, exp, MOD))
    }

    pub fn inv(&self) -> Self {
        Self::new(modulo_inv(self.0, MOD))
    }
}

impl<const MOD: u64> Add for Modulo<MOD> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self((self.0 + rhs.0) % MOD)
    }
}

impl<const MOD: u64> Add<u64> for Modulo<MOD> {
    type Output = Self;

    fn add(self, rhs: u64) -> Self::Output {
        self + Self::new(rhs)
    }
}

impl<const MOD: u64> Add<usize> for Modulo<MOD> {
    type Output = Self;

    fn add(self, rhs: usize) -> Self::Output {
        self + Self::new(rhs as u64)
    }
}

impl<const MOD: u64> Add<i32> for Modulo<MOD> {
    type Output = Self;

    fn add(self, rhs: i32) -> Self::Output {
        self + Self::new(rhs as u64)
    }
}

impl<const MOD: u64, T> AddAssign<T> for Modulo<MOD>
where
    Self: Add<T, Output = Self>,
{
    fn add_assign(&mut self, rhs: T) {
        *self = *self + rhs;
    }
}

impl<const MOD: u64> Sub for Modulo<MOD> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self((self.0 + MOD - rhs.0) % MOD)
    }
}

impl<const MOD: u64> Sub<u64> for Modulo<MOD> {
    type Output = Self;

    fn sub(self, rhs: u64) -> Self::Output {
        self - Self::new(rhs)
    }
}

impl<const MOD: u64> Sub<usize> for Modulo<MOD> {
    type Output = Self;

    fn sub(self, rhs: usize) -> Self::Output {
        self - Self::new(rhs as u64)
    }
}

impl<const MOD: u64> Sub<i32> for Modulo<MOD> {
    type Output = Self;

    fn sub(self, rhs: i32) -> Self::Output {
        self - Self::new(rhs as u64)
    }
}

impl<const MOD: u64, T> SubAssign<T> for Modulo<MOD>
where
    Self: Sub<T, Output = Self>,
{
    fn sub_assign(&mut self, rhs: T) {
        *self = *self - rhs;
    }
}

impl<const MOD: u64> Mul for Modulo<MOD> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0 % MOD)
    }
}

impl<const MOD: u64> Mul<u64> for Modulo<MOD> {
    type Output = Self;

    fn mul(self, rhs: u64) -> Self::Output {
        self * Self::new(rhs)
    }
}

impl<const MOD: u64> Mul<usize> for Modulo<MOD> {
    type Output = Self;

    fn mul(self, rhs: usize) -> Self::Output {
        self * Self::new(rhs as u64)
    }
}

impl<const MOD: u64> Mul<i32> for Modulo<MOD> {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        self * Self::new(rhs as u64)
    }
}

impl<const MOD: u64, T> MulAssign<T> for Modulo<MOD>
where
    Self: Mul<T, Output = Self>,
{
    fn mul_assign(&mut self, rhs: T) {
        *self = *self * rhs;
    }
}

impl<const MOD: u64> Div for Modulo<MOD> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        assert_ne!(rhs.0, 0, "division by zero");
        self * rhs.inv()
    }
}

impl<const MOD: u64> Div<u64> for Modulo<MOD> {
    type Output = Self;

    fn div(self, rhs: u64) -> Self::Output {
        self / Self::new(rhs)
    }
}

impl<const MOD: u64> Div<usize> for Modulo<MOD> {
    type Output = Self;

    fn div(self, rhs: usize) -> Self::Output {
        self / Self::new(rhs as u64)
    }
}

impl<const MOD: u64> Div<i32> for Modulo<MOD> {
    type Output = Self;

    fn div(self, rhs: i32) -> Self::Output {
        self / Self::new(rhs as u64)
    }
}

impl<const MOD: u64, T> DivAssign<T> for Modulo<MOD>
where
    Self: Div<T, Output = Self>,
{
    fn div_assign(&mut self, rhs: T) {
        *self = *self / rhs;
    }
}

impl<const MOD: u64> Debug for Modulo<MOD> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        u64::fmt(&self.0, f)
    }
}

impl<const MOD: u64> ToString for Modulo<MOD> {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl<const MOD: u64> From<u64> for Modulo<MOD> {
    fn from(value: u64) -> Self {
        Self::new(value)
    }
}

impl<const MOD: u64> From<Modulo<MOD>> for u64 {
    fn from(value: Modulo<MOD>) -> Self {
        value.0
    }
}

impl<const MOD: u64> From<usize> for Modulo<MOD> {
    fn from(value: usize) -> Self {
        Self::new(value as u64)
    }
}

impl<const MOD: u64> FromStr for Modulo<MOD> {
    type Err = <u64 as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(u64::from_str(s)?))
    }
}

pub struct ModuloCombinatorics<const MOD: u64> {
    factorials: Factorials,
}

impl<const MOD: u64> ModuloCombinatorics<MOD> {
    pub fn new(n: usize) -> Self {
        Self {
            factorials: Factorials::new(n, MOD),
        }
    }

    pub fn factorial(&self, n: usize) -> Modulo<MOD> {
        Modulo::new(self.factorials.f(n))
    }

    pub fn combinations(&self, n: usize, k: usize) -> Modulo<MOD> {
        Modulo::new(modulo_combinations(&self.factorials, n, k))
    }
}
