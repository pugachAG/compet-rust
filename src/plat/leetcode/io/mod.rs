use crate::utils::io::{InputSource, OutputTarget};

use self::parser::LeetcodeValueNode;

mod parser;

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
    pub fn read<T: FromLeetcodeValueNode>(&mut self) -> T {
        let mut line = String::new();
        self.source.reader().read_line(&mut line).unwrap();
        let node = LeetcodeValueNode::parse_node(&line);
        T::from_leetcode_input_node(&node)
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
    pub fn print(&mut self, v: &impl ToLeetcodeValueNode) {
        let s = v.to_leetcode_value_node();
        self.target.writer().write(s.to_string().as_bytes()).unwrap();
    }
}

impl Default for OutputPrinter {
    fn default() -> Self {
        Self {
            target: OutputTarget::from_env(),
        }
    }
}

pub trait FromLeetcodeValueNode {
    fn from_leetcode_input_node(node: &LeetcodeValueNode) -> Self;
}

pub trait ToLeetcodeValueNode {
    fn to_leetcode_value_node(&self) -> LeetcodeValueNode;
}

macro_rules! impl_from_leetcode_int {
    ($($type:ty),+) => {
        $(
            impl FromLeetcodeValueNode for $type {
                fn from_leetcode_input_node(node: &LeetcodeValueNode) -> Self {
                    if let LeetcodeValueNode::Int(v) = node {
                        (*v) as $type
                    } else {
                        panic!("{:?} is not int", node)
                    }
                }
            }
        )*
    };
}

impl_from_leetcode_int!(i32, i64);

impl FromLeetcodeValueNode for String {
    fn from_leetcode_input_node(node: &LeetcodeValueNode) -> Self {
        if let LeetcodeValueNode::Str(s) = node {
            String::from(s)
        } else {
            panic!("{:?} is not str", node)
        }
    }
}

impl<T: FromLeetcodeValueNode> FromLeetcodeValueNode for Vec<T> {
    fn from_leetcode_input_node(node: &LeetcodeValueNode) -> Self {
        if let LeetcodeValueNode::Array(a) = node {
            a.iter().map(|el| T::from_leetcode_input_node(el)).collect()
        } else {
            panic!("{:?} is not array", node)
        }
    }
}

macro_rules! impl_to_leetcode_int {
    ($($type:ty),+) => {
        $(
            impl ToLeetcodeValueNode for $type {
                fn to_leetcode_value_node(&self) -> LeetcodeValueNode {
                    LeetcodeValueNode::Int(*self as i64)
                }
            }
        )*
    };
}

impl_to_leetcode_int!(i32, i64);

impl ToLeetcodeValueNode for String {
    fn to_leetcode_value_node(&self) -> LeetcodeValueNode {
        LeetcodeValueNode::Str(String::from(self))
    }
}

impl<T: ToLeetcodeValueNode> ToLeetcodeValueNode for Vec<T> {
    fn to_leetcode_value_node(&self) -> LeetcodeValueNode {
        LeetcodeValueNode::Array(self.iter().map(|el| el.to_leetcode_value_node()).collect())
    }
}
