use std::fmt::{Debug, Write};
use std::ops::{BitOr, BitOrAssign, ShlAssign};

use crate::math::utils::div_up;

#[derive(Clone)]
pub struct Bitset {
    n: usize,
    blocks: Vec<usize>,
}

const BLOCK_SZ: usize = usize::BITS as usize;
const BLOCK_ALL_0: usize = usize::MIN;
const BLOCK_ALL_1: usize = usize::MAX;

impl Bitset {
    pub fn new(n: usize) -> Self {
        Self {
            n,
            blocks: vec![BLOCK_ALL_0; div_up(n, BLOCK_SZ)],
        }
    }

    pub fn get(&self, i: usize) -> bool {
        assert!(i < self.n);
        (self.block(i) & Self::mask(i)) > 0
    }

    pub fn set(&mut self, i: usize, v: bool) {
        if self.get(i) != v {
            *self.block_mut(i) ^= Self::mask(i);
        }
    }

    pub fn set_all(&mut self, v: bool) {
        let block_val = if v { BLOCK_ALL_1 } else { BLOCK_ALL_0 };
        for block in self.blocks.iter_mut() {
            *block = block_val;
        }
        self.apply_last_block_mask();
    }

    pub fn len(&self) -> usize {
        self.n
    }

    pub fn bits<'a>(&'a self) -> impl Iterator<Item = bool> + 'a {
        (0..self.n).map(|i| self.get(i))
    }

    pub fn shl_or(&mut self, d: usize) {
        self.shl_op(d, |old, new| new.bitor(old));
    }

    fn shl_op(&mut self, d: usize, op: fn(usize, usize) -> usize) {
        let r = d % BLOCK_SZ;
        let mask_l = (1 << r) - 1;
        let shl_r = r as u32;
        let shr_l = (BLOCK_SZ - r) as u32;
        for i in (0..self.blocks.len()).rev() {
            let j = i.checked_sub(d / BLOCK_SZ);
            let block_r = j.map(|j| self.blocks[j]).unwrap_or(BLOCK_ALL_0);
            let block_l = j
                .and_then(|j| j.checked_sub(1))
                .map(|j| self.blocks[j])
                .unwrap_or(BLOCK_ALL_0);
            let upd = (block_r.wrapping_shl(shl_r)) | (block_l.wrapping_shr(shr_l) & mask_l);
            let block = &mut self.blocks[i];
            *block = op(*block, upd);
        }
        self.apply_last_block_mask();
    }

    #[inline]
    fn mask(i: usize) -> usize {
        1 << (i % BLOCK_SZ)
    }

    #[inline]
    fn block(&self, i: usize) -> usize {
        self.blocks[i / BLOCK_SZ]
    }

    #[inline]
    fn block_mut(&mut self, i: usize) -> &mut usize {
        &mut self.blocks[i / BLOCK_SZ]
    }

    #[inline]
    fn apply_last_block_mask(&mut self) {
        let r = self.n % BLOCK_SZ;
        if r > 0 {
            *self.blocks.last_mut().unwrap() &= (1 << r) - 1;
        }
    }
}

impl BitOrAssign<&Self> for Bitset {
    fn bitor_assign(&mut self, rhs: &Self) {
        assert_eq!(self.len(), rhs.len());
        for (block, other_block) in self.blocks.iter_mut().zip(rhs.blocks.iter()) {
            *block |= other_block
        }
    }
}

impl ShlAssign<usize> for Bitset {
    fn shl_assign(&mut self, d: usize) {
        self.shl_op(d, |_old, new| new);
    }
}

impl Debug for Bitset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for bt in self.bits() {
            f.write_char(if bt { '1' } else { '0' })?
        }
        Ok(())
    }
}
