use crate::types::integer::Integer;

use super::seg_tree::SegTreeValue;

#[derive(Clone, Copy)]
pub struct SegTreeSum<T: Integer>(pub T);

impl<T: Integer> SegTreeSum<T> {
    pub fn new(v: T) -> Self {
        Self(v)
    }
}

impl<T: Integer> SegTreeValue for SegTreeSum<T> {
    fn op(l: Self, r: Self) -> Self {
        Self(l.0 + r.0)
    }

    fn e() -> Self {
        Self(0.into())
    }
}
