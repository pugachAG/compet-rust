pub struct Reducer<T> {
    acc: Option<T>,
    f: fn(T, T) -> T,
}

impl<T: Ord> Reducer<T> {
    pub fn min() -> Self {
        Self::new(T::min)
    }

    pub fn max() -> Self {
        Self::new(T::max)
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
