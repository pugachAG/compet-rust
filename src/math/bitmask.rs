use std::ops::{BitAnd, BitOr, BitXor};

#[derive(Clone, Copy)]
pub struct BitMask(usize);

impl BitMask {
    pub fn val(&self) -> usize {
        self.0
    }

    #[inline]
    pub fn has_one_at(&self, i: usize) -> bool {
        (self.0 & (1 << i)) > 0
    }

    pub fn one_indices(self) -> impl Iterator<Item = usize> {
        self.bit_indices().filter(move |&bt| self.has_one_at(bt))
    }

    pub fn all_count(n: usize) -> usize {
        1 << n
    }

    pub fn all_ones(n: usize) -> BitMask {
        BitMask(Self::all_count(n).saturating_sub(1))
    }

    pub fn all(n: usize) -> impl Iterator<Item = Self> {
        (0..Self::all_count(n)).map(|mask| Self(mask))
    }

    pub fn submasks(self) -> SubmaskIter {
        SubmaskIter::new(self.0)
    }

    fn bit_indices(&self) -> impl Iterator<Item = usize> {
        let v1 = self.0;
        (0..usize::BITS as usize).take_while(move |&bt| (1 << bt) <= v1)
    }
}

pub struct SubmaskIter {
    mask: usize,
    cur: Option<usize>,
}

impl SubmaskIter {
    fn new(mask: usize) -> Self {
        Self {
            mask,
            cur: Some(mask),
        }
    }
}

impl Iterator for SubmaskIter {
    type Item = BitMask;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(cur) = self.cur {
            self.cur = if cur > 0 {
                Some((cur - 1) & self.mask)
            } else {
                None
            };
            Some(BitMask(cur))
        } else {
            None
        }
    }
}

impl BitXor for BitMask {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.val() ^ rhs.val())
    }
}

impl BitOr for BitMask {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.val() | rhs.val())
    }
}

impl BitAnd for BitMask {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.val() & rhs.val())
    }
}
