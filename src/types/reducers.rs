#[derive(Clone)]
pub struct Reducer<T> {
    acc: Option<T>,
    f: fn(T, T) -> T,
}

impl<T: PartialOrd> Reducer<T> {
    pub fn min() -> Self {
        Self::new(|cur, upd| if upd.lt(&cur) { upd } else { cur })
    }

    pub fn max() -> Self {
        Self::new(|cur, upd| if upd.gt(&cur) { upd } else { cur })
    }
}

impl<T> Reducer<T> {
    pub fn new(f: fn(T, T) -> T) -> Self {
        Self { acc: None, f }
    }

    pub fn update(&mut self, v: T) {
        self.acc = Some(if let Some(cur) = std::mem::take(&mut self.acc) {
            (self.f)(cur, v)
        } else {
            v
        })
    }

    pub fn done(self) -> Option<T> {
        self.acc
    }

    pub fn acc(&self) -> Option<&T> {
        self.acc.as_ref()
    }
}

pub struct TopN<T, const N: usize> {
    acc: [Option<T>; N],
    f: fn(&T, &T) -> bool,
}

impl<T: PartialOrd, const N: usize> TopN<T, N> {
    pub fn min() -> Self {
        Self::new(|cur, upd| upd.lt(cur))
    }

    pub fn max() -> Self {
        Self::new(|cur, upd| upd.gt(cur))
    }
}

impl<T: PartialOrd, const N: usize> TopN<T, N> {
    pub fn new(f: fn(&T, &T) -> bool) -> Self {
        Self {
            acc: std::array::from_fn(|_| None),
            f,
        }
    }

    pub fn update(&mut self, v: T) {
        let mut cand = v;
        for slot in &mut self.acc {
            if let Some(cur) = slot {
                if (self.f)(cur, &cand) {
                    std::mem::swap(cur, &mut cand);
                }
            } else {
                *slot = Some(cand);
                break;
            }
        }
    }

    pub fn done(self) -> [Option<T>; N] {
        self.acc
    }

    pub fn acc(&self) -> &[Option<T>; N] {
        &self.acc
    }
}
