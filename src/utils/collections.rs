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

pub trait VecReversedExt {
    fn reversed(&self) -> Self;
}

pub trait VecSortedExt {
    fn sorted(&self) -> Self;
}

pub trait VecSortedByKeyExt {
    type Item;

    fn sorted_by_key<F, K>(&self, f: F) -> Self
    where
        F: FnMut(&Self::Item) -> K,
        K: Ord;
}

impl<T: Ord + Clone> VecSortedExt for Vec<T> {
    fn sorted(&self) -> Self {
        let mut res = self.to_vec();
        res.sort();
        res
    }
}

impl<T: Clone> VecSortedByKeyExt for Vec<T> {
    type Item = T;

    fn sorted_by_key<F, K>(&self, f: F) -> Self
    where
        F: FnMut(&Self::Item) -> K,
        K: Ord,
    {
        let mut res = self.to_vec();
        res.sort_by_key(f);
        res
    }
}

impl<T: Clone> VecReversedExt for Vec<T> {
    fn reversed(&self) -> Self {
        let mut res = self.to_vec();
        res.reverse();
        res
    }
}
