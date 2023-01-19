use std::fmt::Debug;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, Rem};

pub trait Integer:
    Copy
    + Debug
    + Ord
    + From<u8>
    + AddAssign<Self>
    + Add<Self, Output = Self>
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
        + Ord
        + From<u8>
        + AddAssign<Self>
        + Add<Self, Output = Self>
        + Mul<Self, Output = Self>
        + Rem<Self, Output = Self>
        + Div<Self, Output = Self>
        + DivAssign<Self>
{
}
