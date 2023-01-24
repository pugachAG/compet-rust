use std::fmt::{Debug, Display};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, Rem, Sub, SubAssign};

pub trait Integer:
    Copy
    + Debug
    + Display
    + Ord
    + From<u8>
    + Add<Self, Output = Self>
    + AddAssign<Self>
    + Sub<Self, Output = Self>
    + SubAssign<Self>
    + Mul<Self, Output = Self>
    + Rem<Self, Output = Self>
    + Div<Self, Output = Self>
    + DivAssign<Self>
where
    Self: Sized,
{
}

impl<T> Integer for T where
    T: Copy
        + Debug
        + Display
        + Ord
        + From<u8>
        + Add<Self, Output = Self>
        + AddAssign<Self>
        + Sub<Self, Output = Self>
        + SubAssign<Self>
        + Mul<Self, Output = Self>
        + Rem<Self, Output = Self>
        + Div<Self, Output = Self>
        + DivAssign<Self>
{
}
