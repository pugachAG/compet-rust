use std::cmp::Ordering;
use std::ops::{Bound, RangeBounds};

use crate::types::integer::Integer;

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

pub struct BinSearch<T: Integer> {
    left: Boundary<T>,
    right: Boundary<T>,
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

    pub fn update_left(self, ok: bool) {
        self.bs.set_left(self.val, ok);
    }

    pub fn update_right(self, ok: bool) {
        self.bs.set_right(self.val, ok);
    }
}

impl<T: Integer> BinSearch<T> {
    pub fn new(range: impl RangeBounds<T>) -> Self {
        let val_left = match range.start_bound() {
            Bound::Included(val) => *val,
            Bound::Excluded(val) => *val + 1.into(),
            Bound::Unbounded => panic!("unbounded start is not supported"),
        };
        let val_right = match range.end_bound() {
            Bound::Included(val) => *val,
            Bound::Excluded(val) => *val - 1.into(),
            Bound::Unbounded => panic!("unbounded end is not supported"),
        };
        Self {
            left: Boundary::unconfirmed(val_left),
            right: Boundary::unconfirmed(val_right),
            ans: if val_left <= val_right {
                None
            } else {
                Some(None)
            },
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

#[cfg(test)]
mod tests {
    use crate::plat::classic::includes::IntoVecExt;

    use super::BinSearch;

    #[test]
    fn test_bin_search() {
        const MIN_VAL: i32 = 1;
        const MAX_VAL: i32 = 4;
        let a = (MIN_VAL..=MAX_VAL).flat_map(|v| vec![v; 3]).into_vec();
        let n = a.len();
        for l in 0..n {
            for r in l..n {
                for v in MIN_VAL - 1..=MAX_VAL + 1 {
                    check_last_le(&a, l, r, v);
                    check_first_gt(&a, l, r, v);
                }
            }
        }
    }

    #[test]
    fn test_debug() {
        check_first_gt(&[1, 1, 1, 2], 0, 3, 1);
    }

    fn check_last_le(a: &[i32], l: usize, r: usize, v: i32) {
        let expected = (l..=r).filter(|&i| a[i] <= v).last();
        let actual = bin_search_last_le(a, l, r, v);
        check_index(actual, expected, a, l, r, v);
    }

    fn check_first_gt(a: &[i32], l: usize, r: usize, v: i32) {
        let expected = (l..=r).filter(|&i| a[i] > v).next();
        let actual = bin_search_first_gt(a, l, r, v);
        check_index(actual, expected, a, l, r, v);
    }

    #[track_caller]
    fn check_index(
        actual: Option<usize>,
        expected: Option<usize>,
        a: &[i32],
        l: usize,
        r: usize,
        v: i32,
    ) {
        assert_eq!(actual, expected, "[{l}, {r}] {v} in {:?}", &a[l..=r]);
    }

    fn bin_search_last_le(a: &[i32], l: usize, r: usize, v: i32) -> Option<usize> {
        let mut bs = BinSearch::new(l..=r);
        let mut iterations = 0;
        while let Some(token) = bs.next() {
            let i = token.val();
            if a[i] <= v {
                token.update_left(true);
            } else {
                token.update_right(false);
            }
            iterations += 1;
            assert!(iterations <= 40, "too many iterations");
        }
        bs.answer()
    }

    fn bin_search_first_gt(a: &[i32], l: usize, r: usize, v: i32) -> Option<usize> {
        let mut bs = BinSearch::new(l..=r);
        let mut iterations = 0;
        while let Some(token) = bs.next() {
            let i = token.val();
            if a[i] > v {
                token.update_right(true);
            } else {
                token.update_left(false);
            }
            iterations += 1;
            assert!(iterations <= 40, "too many iterations");
        }
        bs.answer()
    }
}
