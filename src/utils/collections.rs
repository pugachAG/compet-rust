use std::collections::{HashMap, HashSet};
use std::hash::Hash;

pub type Vec2<T> = Vec<Vec<T>>;
pub type Vec3<T> = Vec<Vec<Vec<T>>>;

pub fn def_vec<T: Default>(n: usize) -> Vec<T> {
    (0..n).map(|_| T::default()).into_vec()
}

pub fn def_vec2<T: Default>(n: usize, m: usize) -> Vec2<T> {
    (0..n).map(|_| def_vec(m)).into_vec()
}

pub fn def_vec3<T: Default>(n: usize, m: usize, k: usize) -> Vec3<T> {
    (0..n).map(|_| def_vec2(m, k)).into_vec()
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

pub trait IntoSetExt {
    type Item;

    fn into_set(self) -> HashSet<Self::Item>;
}

impl<T: Iterator> IntoSetExt for T
where
    T::Item: Eq + Hash,
{
    type Item = T::Item;

    fn into_set(self) -> HashSet<Self::Item> {
        self.collect()
    }
}

pub trait IntoMapExt {
    type Key;
    type Value;

    fn into_map(self) -> HashMap<Self::Key, Self::Value>;
}

impl<K: Eq + Hash, V, T: Iterator<Item = (K, V)>> IntoMapExt for T {
    type Key = K;
    type Value = V;

    fn into_map(self) -> HashMap<Self::Key, Self::Value> {
        self.collect()
    }
}

pub trait SliceReversedExt {
    type Item;

    fn reversed(self) -> Vec<Self::Item>;
}

impl<T: Clone> SliceReversedExt for &[T] {
    type Item = T;

    fn reversed(self) -> Vec<Self::Item> {
        let mut a = self.to_vec();
        a.reverse();
        a
    }
}

pub trait SliceSortedExt {
    type Item;

    fn sorted(self) -> Vec<Self::Item>;
}

impl<T: Ord + Clone> SliceSortedExt for &[T] {
    type Item = T;

    fn sorted(self) -> Vec<Self::Item> {
        let mut a = self.to_vec();
        a.sort();
        a
    }
}

pub trait SliceSortedByKeyExt {
    type Item;

    fn sorted_by_key<F, K>(self, f: F) -> Vec<Self::Item>
    where
        F: FnMut(&Self::Item) -> K,
        K: Ord;
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

pub trait SliceMinMaxExt {
    type Item;

    fn min_element(self) -> Self::Item;

    fn max_element(self) -> Self::Item;
}

impl<T: Clone + Ord> SliceMinMaxExt for &[T] {
    type Item = T;

    fn min_element(self) -> Self::Item {
        self.iter().min().unwrap().clone()
    }

    fn max_element(self) -> Self::Item {
        self.iter().max().unwrap().clone()
    }
}

pub trait SliceSumExt {
    type Item;

    fn sum(self) -> Self::Item;
}

impl<T: Clone + std::iter::Sum<T>> SliceSumExt for &[T] {
    type Item = T;

    fn sum(self) -> Self::Item {
        self.iter().cloned().sum::<Self::Item>()
    }
}
