pub fn def_vec<T: Default>(n: usize) -> Vec<T> {
    (0..n).map(|_| T::default()).collect()
}

pub trait AsVecExt {
    type Item;
    fn as_vec(self) -> Vec<Self::Item>;
}

impl<T: Iterator> AsVecExt for T {
    type Item = T::Item;
    fn as_vec(self) -> Vec<Self::Item> {
        self.collect::<Vec<_>>()
    }
}
