use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::{Deref, Index, IndexMut};

#[derive(Clone)]
pub struct DefaultDict<K, V> {
    map: HashMap<K, V>,
    default_value: V,
}

impl<K> DefaultDict<K, usize> {
    pub fn counter() -> Self {
        Self::new()
    }
}

impl<K, V: Default> DefaultDict<K, V> {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            default_value: V::default(),
        }
    }
}

impl<K: Hash + Eq, V: Default> Index<K> for DefaultDict<K, V> {
    type Output = V;

    fn index(&self, index: K) -> &Self::Output {
        self.map.get(&index).unwrap_or(&self.default_value)
    }
}

impl<K: Hash + Eq, V: Default> IndexMut<K> for DefaultDict<K, V> {
    fn index_mut(&mut self, index: K) -> &mut Self::Output {
        self.map.entry(index).or_insert_with(V::default)
    }
}

impl<K, V> IntoIterator for DefaultDict<K, V> {
    type Item = (K, V);
    type IntoIter = std::collections::hash_map::IntoIter<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.map.into_iter()
    }
}

impl<K, V> Deref for DefaultDict<K, V> {
    type Target = HashMap<K, V>;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl<K: Debug, V: Debug> Debug for DefaultDict<K, V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.map.fmt(f)
    }
}
