use std::str::FromStr;

use crate::utils::io::{InputSource, OutputTarget};

pub struct InputReader {
    source: InputSource<'static>,
}

#[macro_export]
macro_rules! with_input {
    ($r:ident => $func:ident, 1) => {
        crate::plat::leetcode::solution::Solution::$func($r.read())
    };
    ($r:ident => $func:ident, 2) => {
        crate::plat::leetcode::solution::Solution::$func($r.read(), $r.read())
    };
}

pub struct OutputPrinter {
    target: OutputTarget,
}

impl InputReader {
    pub fn read<T: LeetcodeFromString>(&mut self) -> T {
        let mut line = String::new();
        self.source.reader().read_line(&mut line).unwrap();
        T::from_leetcode_string(&line)
    }
}

impl Default for InputReader {
    fn default() -> Self {
        Self {
            source: InputSource::from_env(),
        }
    }
}

impl OutputPrinter {
    pub fn print(&mut self, v: &impl LeetcodeToString) {
        let s = v.to_leetcode_string();
        self.target.writer().write(s.as_bytes()).unwrap();
    }
}

impl Default for OutputPrinter {
    fn default() -> Self {
        Self {
            target: OutputTarget::from_env(),
        }
    }
}

pub trait LeetcodeFromString {
    fn from_leetcode_string(s: &str) -> Self;
}

pub trait LeetcodeToString {
    fn to_leetcode_string(&self) -> String;
}

macro_rules! impl_leetcode_from_string {
    ($($type:ty),+) => {
        $(
            impl LeetcodeFromString for $type {
                fn from_leetcode_string(s: &str) -> Self {
                    <$type>::from_str(s.trim()).unwrap()
                }
            }
        )*
    };
}

impl_leetcode_from_string!(i32, i64);

impl LeetcodeFromString for String {
    fn from_leetcode_string(s: &str) -> Self {
        String::from(strip(s, '"', '"'))
    }
}

impl<T> LeetcodeFromString for Vec<T>
where
    T: LeetcodeFromString,
{
    fn from_leetcode_string(s: &str) -> Self {
        String::from(strip(s, '[', ']'))
            .split(",")
            .map(|s| T::from_leetcode_string(s))
            .collect()
    }
}

fn strip(s: &str, ch_first: char, ch_last: char) -> &str {
    let s = s.trim();
    assert_eq!(s.chars().nth(0), Some(ch_first));
    assert_eq!(s.chars().nth(s.len() - 1), Some(ch_last));
    &s[1..s.len() - 1]
}

macro_rules! impl_printable {
    ($($type:ty),+) => {
        $(
            impl LeetcodeToString for $type {
                fn to_leetcode_string(&self) -> String {
                    <$type>::to_string(self)
                }
            }
        )*
    };
}

impl_printable!(i32, i64, &str);
