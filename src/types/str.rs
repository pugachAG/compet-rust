use std::fmt::Debug;
use std::ops::{Deref, DerefMut};

#[derive(Default, Clone)]
pub struct Str(pub Vec<u8>);

impl Str {
    pub fn empty() -> Self {
        Self::default()
    }

    pub fn new(n: usize, ch: u8) -> Self {
        Self(vec![ch; n])
    }

    pub fn as_str(&self) -> &str {
        std::str::from_utf8(self).unwrap()
    }

    pub fn count(&self, ch: u8) -> usize {
        self.iter().filter(|&&c| c == ch).count()
    }
}

impl ToString for Str {
    fn to_string(&self) -> String {
        self.as_str().to_owned()
    }
}

impl From<&[u8]> for Str {
    fn from(value: &[u8]) -> Self {
        Self(value.to_vec())
    }
}

impl From<&str> for Str {
    fn from(value: &str) -> Self {
        Self::from(value.as_bytes())
    }
}

impl From<String> for Str {
    fn from(value: String) -> Self {
        Self(value.into())
    }
}

impl AsRef<[u8]> for Str {
    fn as_ref(&self) -> &[u8] {
        self
    }
}

impl Deref for Str {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Str {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl IntoIterator for Str {
    type Item = u8;
    type IntoIter = std::vec::IntoIter<u8>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a Str {
    type Item = &'a u8;
    type IntoIter = std::slice::Iter<'a, u8>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> IntoIterator for &'a mut Str {
    type Item = &'a mut u8;
    type IntoIter = std::slice::IterMut<'a, u8>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl Debug for Str {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
