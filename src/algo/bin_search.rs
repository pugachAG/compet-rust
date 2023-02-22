use std::cmp::Ordering;
use std::ops::{Bound, Range, RangeBounds};

use crate::types::integer::Integer;

/// # Examples
/// ```
/// assert_eq!(bin_search_range(&[1, 2, 2, 3], 2..3), 1..3);
/// ```
pub fn bin_search_range<T: Ord, B: RangeBounds<T>>(a: &[T], range: B) -> Range<usize> {
    let n = a.len();
    if let Some(l) = bin_search_start(a, range.start_bound()) {
        if let Some(d_r) = bin_search_end(&a[l..], range.end_bound()) {
            return l..l + d_r + 1;
        } else {
            l..l
        }
    } else {
        n..n
    }
}

fn bin_search_start<T: Ord>(a: &[T], b: Bound<&T>) -> Option<usize> {
    if matches!(b, Bound::Unbounded) && !a.is_empty() {
        return Some(0);
    }
    let mut bs = BinSearch::suffix(0..a.len());
    while let Some(token) = bs.next() {
        let cur = &a[token.val];
        let ok = match b {
            Bound::Included(v) => cur >= v,
            Bound::Excluded(v) => cur > v,
            Bound::Unbounded => unreachable!(),
        };
        token.update(ok);
    }
    bs.answer()
}

fn bin_search_end<T: Ord>(a: &[T], b: Bound<&T>) -> Option<usize> {
    let n = a.len();
    if matches!(b, Bound::Unbounded) && !a.is_empty() {
        return Some(n - 1);
    }
    let mut bs = BinSearch::prefix(0..n);
    while let Some(token) = bs.next() {
        let cur = &a[token.val];
        let ok = match b {
            Bound::Included(v) => cur <= v,
            Bound::Excluded(v) => cur < v,
            Bound::Unbounded => unreachable!(),
        };
        token.update(ok);
    }
    bs.answer()
}

#[derive(Debug)]
struct Boundary<T: Integer> {
    val: T,
    confirmed: bool,
}

impl<T: Integer> Boundary<T> {
    fn new(val: T, confirmed: bool) -> Self {
        Self { val, confirmed }
    }

    fn confirmed(val: T) -> Self {
        Self::new(val, true)
    }

    fn unconfirmed(val: T) -> Self {
        Self::new(val, false)
    }
}

#[derive(Copy, Clone)]
pub enum BinSearchLayout {
    Prefix,
    Suffix,
}

/// # Example
/// ```
/// fn isqrt(x: u32) -> u32 {
///     let mut bs = BinSearch::prefix(0..=x);
///     while let Some(token) = bs.next() {
///         let ok = token.val().pow(2) <= x;
///         token.update(ok);
///     }
///     bs.answer().unwrap()
/// }
/// assert_eq!(isqrt(30), 5);
/// ```
pub struct BinSearch<T: Integer> {
    left: Boundary<T>,
    right: Boundary<T>,
    layout: BinSearchLayout,
    ans: Option<Option<T>>,
}

pub struct BinSearchDecisionToken<'a, T: Integer> {
    val: T,
    bs: &'a mut BinSearch<T>,
}

impl<T: Integer> BinSearchDecisionToken<'_, T> {
    pub fn val(&self) -> T {
        self.val
    }

    pub fn update(self, ok: bool) {
        self.bs.update(self.val, ok);
    }
}

impl<T: Integer> BinSearch<T> {
    /// [1 1 1 1 0 0]
    ///        ^
    pub fn prefix(range: impl RangeBounds<T>) -> Self {
        Self::new(range, BinSearchLayout::Prefix)
    }

    /// [0 0 0 1 1 1 1]
    ///        ^
    pub fn suffix(range: impl RangeBounds<T>) -> Self {
        Self::new(range, BinSearchLayout::Suffix)
    }

    pub fn new(range: impl RangeBounds<T>, layout: BinSearchLayout) -> Self {
        let val_left_incl = match range.start_bound() {
            Bound::Included(val) => *val,
            Bound::Excluded(val) => *val + 1.into(),
            Bound::Unbounded => panic!("unbounded start is not supported"),
        };
        let val_right_excl = match range.end_bound() {
            Bound::Included(val) => *val + 1.into(),
            Bound::Excluded(val) => *val,
            Bound::Unbounded => panic!("unbounded end is not supported"),
        };
        if val_left_incl >= val_right_excl {
            Self {
                left: Boundary::unconfirmed(val_left_incl),
                // keep left exclusive to avoid underflow when val_right_excl == 0usize
                right: Boundary::unconfirmed(val_right_excl),
                layout,
                ans: Some(None),
            }
        } else {
            Self {
                left: Boundary::unconfirmed(val_left_incl),
                right: Boundary::unconfirmed(val_right_excl - 1.into()),
                layout,
                ans: None,
            }
        }
    }

    pub fn next(&mut self) -> Option<BinSearchDecisionToken<T>> {
        if self.ans.is_some() {
            return None;
        }
        let mut val = (self.left.val + self.right.val) / 2.into();
        if val == self.left.val && self.left.confirmed {
            val += 1.into();
        }
        Some(BinSearchDecisionToken { val, bs: self })
    }

    pub fn answer(&self) -> Option<T> {
        if let Some(v) = self.ans {
            v
        } else {
            panic!("binary search is not done")
        }
    }

    fn update(&mut self, val: T, ok: bool) {
        match (self.layout, ok) {
            (BinSearchLayout::Prefix, true) | (BinSearchLayout::Suffix, false) => {
                self.set_left(val, ok)
            }
            (BinSearchLayout::Prefix, false) | (BinSearchLayout::Suffix, true) => {
                self.set_right(val, ok)
            }
        }
    }

    fn set_left(&mut self, val: T, ok: bool) {
        if ok {
            self.left = Boundary::confirmed(val);
            self.on_boundary_changed();
        } else if val == self.right.val {
            self.ans = Some(self.confirmed_val());
        } else {
            self.left = Boundary::unconfirmed(val + 1.into());
            self.on_boundary_changed();
        }
    }

    fn set_right(&mut self, val: T, ok: bool) {
        if ok {
            self.right = Boundary::confirmed(val);
            self.on_boundary_changed();
        } else if val == self.left.val {
            self.ans = Some(self.confirmed_val());
        } else {
            self.right = Boundary::unconfirmed(val - 1.into());
            self.on_boundary_changed();
        }
    }

    fn on_boundary_changed(&mut self) {
        assert!(
            !(self.left.confirmed && self.right.confirmed && self.left.val < self.right.val),
            "invariant violation: left={} and right={} boundaries should not be both confirmed",
            self.left.val,
            self.right.val
        );
        match self.left.val.cmp(&self.right.val) {
            Ordering::Less => {}
            Ordering::Equal => {
                let confirmed_val = self.confirmed_val();
                if confirmed_val.is_some() {
                    self.ans = Some(confirmed_val);
                }
            }
            Ordering::Greater => {
                self.ans = Some(self.confirmed_val());
            }
        }
    }

    fn confirmed_val(&self) -> Option<T> {
        if self.left.confirmed {
            Some(self.left.val)
        } else if self.right.confirmed {
            Some(self.right.val)
        } else {
            None
        }
    }
}
