use std::fmt::Debug;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
use std::str::FromStr;

use super::modulo_ops::{modulo_inv, modulo_pow};

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct Modulo<const MOD: u64>(u64);

impl<const MOD: u64> Modulo<MOD> {
    #[inline]
    pub fn new(v: u64) -> Self {
        Self(v % MOD)
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

impl<const MOD: u64> FromStr for Modulo<MOD> {
    type Err = <u64 as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(u64::from_str(s)?))
    }
}

#[cfg(test)]
mod tests {
    use super::Modulo;

    type Mod = Modulo<11>;

    #[test]
    fn test_modulo_new() {
        check_modulo(Mod::new(17), 6);
    }

    #[test]
    fn test_modulo_add() {
        let mut v = Mod::new(6);
        check_modulo(v + 7, 2);
        check_modulo(v + Mod::new(1), 7);
        v += 10;
        check_modulo(v, 5);
        v += Mod::new(1);
        check_modulo(v, 6);
    }

    #[test]
    fn test_modulo_sub() {
        let mut v = Mod::new(6);
        check_modulo(v - 7, 10);
        check_modulo(v - Mod::new(1), 5);
        v -= 2;
        check_modulo(v, 4);
        v -= Mod::new(10);
        check_modulo(v, 5);
    }

    #[test]
    fn test_modulo_mul() {
        let mut v = Mod::new(5);
        check_modulo(v * 7, 2);
        check_modulo(v * Mod::new(2), 10);
        v *= 3;
        check_modulo(v, 4);
        v *= Mod::new(10);
        check_modulo(v, 7);
    }

    #[test]
    fn test_modulo_div() {
        let mut v = Mod::new(6);
        check_modulo(v / 3, 2);
        check_modulo(v / Mod::new(2), 3);
        v /= 6;
        check_modulo(v, 1);
    }

    #[test]
    fn test_modulo_pow() {
        let v = Mod::new(7);
        check_modulo(v.pow(2), 5);
        check_modulo(v.pow(123121424123123123), 2);
    }

    #[test]
    fn test_modulo_inv() {
        check_modulo(Mod::new(7).inv(), 8);
    }

    #[track_caller]
    fn check_modulo<const MOD: u64>(actual: Modulo<MOD>, expected: u64) {
        assert_eq!(actual, Modulo::<MOD>(expected));
    }
}