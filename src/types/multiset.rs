use super::default_dict::DefaultDict;
use std::hash::Hash;

pub struct Multiset<T> {
    map: DefaultDict<T, usize>,
    len: usize,
}

impl<T: Hash + Eq> Multiset<T> {
    pub fn new() -> Self {
        Self {
            map: DefaultDict::new(),
            len: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn len_unique(&self) -> usize {
        self.map.len()
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    pub fn count(&self, v: &T) -> usize {
        *self.map.get(v).unwrap_or(&0)
    }

    pub fn contains(&self, v: &T) -> bool {
        self.map.contains_key(&v)
    }

    pub fn insert(&mut self, v: T) {
        self.map[v] += 1;
        self.len += 1;
    }

    pub fn remove_one(&mut self, v: &T) {
        if let Some(cnt) = self.map.get_mut(&v) {
            if *cnt > 1 {
                *cnt -= 1;
            } else {
                self.map.remove(&v);
            }
        }
    }

    pub fn remove_all(&mut self, v: &T) {
        self.map.remove(v);
    }

    pub fn clear(&mut self) {
        self.map.clear();
        self.len = 0;
    }
}

impl<T: Hash + Eq> From<Vec<T>> for Multiset<T> {
    fn from(value: Vec<T>) -> Self {
        let mut ret = Self::new();
        for v in value {
            ret.insert(v);
        }
        ret
    }
}

impl<T: Hash + Eq + Clone> From<&Vec<T>> for Multiset<T> {
    fn from(value: &Vec<T>) -> Self {
        let mut ret = Self::new();
        for v in value {
            ret.insert(v.clone());
        }
        ret
    }
}
