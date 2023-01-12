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

pub trait SliceReversedExt {
    type Item;

    fn reversed(self) -> Vec<Self::Item>;
}

pub trait SliceSortedExt {
    type Item;

    fn sorted(self) -> Vec<Self::Item>;
}

pub trait SliceSortedByKeyExt {
    type Item;

    fn sorted_by_key<F, K>(self, f: F) -> Vec<Self::Item>
    where
        F: FnMut(&Self::Item) -> K,
        K: Ord;
}

impl<T: Ord + Clone> SliceSortedExt for &[T] {
    type Item = T;

    fn sorted(self) -> Vec<Self::Item> {
        let mut a = self.to_vec();
        a.sort();
        a
    }
}

impl<T: Clone> SliceSortedByKeyExt for &[T] {
    type Item = T;

    fn sorted_by_key<F, K>(self, f: F) -> Vec<Self::Item>
    where
        F: FnMut(&Self::Item) -> K,
        K: Ord,
    {
        let mut a = self.to_vec();
        a.sort_by_key(f);
        a
    }
}

impl<T: Clone> SliceReversedExt for &[T] {
    type Item = T;

    fn reversed(self) -> Vec<Self::Item> {
        let mut a = self.to_vec();
        a.reverse();
        a
    }
}
