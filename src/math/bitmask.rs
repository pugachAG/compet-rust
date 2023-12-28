pub struct BitMask(usize);

impl BitMask {
    pub fn val(&self) -> usize {
        self.0
    }

    pub fn ones(&self) -> impl Iterator<Item = usize> {
        let (v1, v2) = (self.0, self.0);
        (0..usize::BITS as usize)
            .take_while(move |&bt| (1 << bt) <= v1)
            .filter(move |bt| ((1 << bt) & v2) > 0)
    }

    pub fn all_count(n: usize) -> usize {
        1 << n
    }

    pub fn all(n: usize) -> impl Iterator<Item = Self> {
        (0..Self::all_count(n)).map(|mask| Self(mask))
    }

    pub fn submasks(&self) -> SubmaskIter {
        SubmaskIter::new(self.0)
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
