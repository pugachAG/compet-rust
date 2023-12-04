use std::ops::RangeBounds;

use crate::utils::misc::unpack_range;

pub struct Random {
    state: usize,
}

impl Random {
    pub fn new(seed: usize) -> Self {
        assert_ne!(seed, 0);
        Self { state: seed }
    }

    pub fn from_cur_time() -> Self {
        let seed = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as usize;
        Self::new(seed)
    }

    pub fn gen(&mut self) -> usize {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.state = x;
        x
    }

    pub fn gen_range(&mut self, rng_bounds: impl RangeBounds<usize>) -> usize {
        let rng = unpack_range(rng_bounds);
        assert!(rng.start < rng.end, "invalid range {rng:?}");
        rng.start + self.gen() % (rng.end - rng.start)
    }

    pub fn gen_f64(&mut self) -> f64 {
        (self.gen() as f64) / (std::usize::MAX as f64)
    }
}
