pub fn def_vec<T: Default>(n: usize) -> Vec<T> {
    (0..n).map(|_| T::default()).into_vec()
}

pub trait IntoVecExt {
    type Item;
    fn into_vec(self) -> Vec<Self::Item>;
}

impl<T: Iterator> IntoVecExt for T {
    type Item = T::Item;
    fn into_vec(self) -> Vec<Self::Item> {
        self.collect::<Vec<_>>()
    }
}
