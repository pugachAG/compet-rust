use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use crate::utils::io::{InputSource, OutputTarget};

use self::parser::LeetcodeValueNode;

use super::defs::TreeNode;

mod parser;

pub struct InputReader {
    source: InputSource,
}

#[macro_export]
macro_rules! with_input {
    ($r:ident => $func:ident, 1) => {
        crate::plat::leetcode::solution::Solution::$func($r.read())
    };
    ($r:ident => $func:ident, 2) => {
        crate::plat::leetcode::solution::Solution::$func($r.read(), $r.read())
    };
    ($r:ident => $func:ident, 3) => {
        crate::plat::leetcode::solution::Solution::$func($r.read(), $r.read(), $r.read())
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
        self.target
            .writer()
            .write(s.to_string().as_bytes())
            .unwrap();
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

impl<T: FromLeetcodeValueNode> FromLeetcodeValueNode for Option<T> {
    fn from_leetcode_input_node(node: &LeetcodeValueNode) -> Self {
        match node {
            LeetcodeValueNode::Null => None,
            other => Some(T::from_leetcode_input_node(other)),
        }
    }
}

impl FromLeetcodeValueNode for Option<Rc<RefCell<TreeNode>>> {
    fn from_leetcode_input_node(node: &LeetcodeValueNode) -> Self {
        match node {
            LeetcodeValueNode::Array(a) => convert_to_tree_node(a),
            other => panic!("{:?} cannot be parsed to TreeNode, expected array", other),
        }
    }
}

fn create_tree_node(node: Option<&LeetcodeValueNode>) -> Option<Rc<RefCell<TreeNode>>> {
    match node {
        None | Some(LeetcodeValueNode::Null) => None,
        Some(LeetcodeValueNode::Int(v)) => Some(Rc::new(RefCell::new(TreeNode::new(*v as i32)))),
        other => panic!("expected int or null, got {:?}", other),
    }
}

fn convert_to_tree_node(a: &[LeetcodeValueNode]) -> Option<Rc<RefCell<TreeNode>>> {
    let mut q = VecDeque::<Rc<RefCell<TreeNode>>>::new();
    let root = create_tree_node(a.get(0));
    if let Some(ref rc) = root {
        q.push_back(rc.clone());
    } else {
        return None;
    }
    let mut i = 1;
    while let Some(rc) = q.pop_front() {
        let mut node = rc.borrow_mut();
        node.left = create_tree_node(a.get(i));
        node.right = create_tree_node(a.get(i + 1));
        i += 2;
        for child in [&node.left, &node.right].iter() {
            if let Some(rc) = child {
                q.push_back(rc.clone());
            }
        }
    }
    if i < a.len() {
        panic!("{:?} contains redundant elements, consumed {}", a, i + 1);
    }
    root
}

impl FromLeetcodeValueNode for String {
    fn from_leetcode_input_node(node: &LeetcodeValueNode) -> Self {
        if let LeetcodeValueNode::Str(s) = node {
            String::from(s)
        } else {
            panic!("{:?} is not str", node)
        }
    }
}

impl FromLeetcodeValueNode for bool {
    fn from_leetcode_input_node(node: &LeetcodeValueNode) -> Self {
        if let LeetcodeValueNode::Bool(v) = node {
            *v
        } else {
            panic!("{:?} is not bool", node)
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

impl<T: ToLeetcodeValueNode> ToLeetcodeValueNode for Option<T> {
    fn to_leetcode_value_node(&self) -> LeetcodeValueNode {
        match self {
            Some(v) => v.to_leetcode_value_node(),
            None => LeetcodeValueNode::Null,
        }
    }
}

impl ToLeetcodeValueNode for String {
    fn to_leetcode_value_node(&self) -> LeetcodeValueNode {
        LeetcodeValueNode::Str(String::from(self))
    }
}

impl ToLeetcodeValueNode for bool {
    fn to_leetcode_value_node(&self) -> LeetcodeValueNode {
        LeetcodeValueNode::Bool(*self)
    }
}

impl<T: ToLeetcodeValueNode> ToLeetcodeValueNode for Vec<T> {
    fn to_leetcode_value_node(&self) -> LeetcodeValueNode {
        LeetcodeValueNode::Array(self.iter().map(|el| el.to_leetcode_value_node()).collect())
    }
}
