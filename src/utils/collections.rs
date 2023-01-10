pub fn def_vec<T: Default>(n: usize) -> Vec<T> {
    (0..n).map(|_| T::default()).collect()
}