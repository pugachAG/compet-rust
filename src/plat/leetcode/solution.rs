impl Solution {
    pub fn echo(n: i32) -> i32 {
        n
    }
}

use crate::{plat::leetcode::io::{InputReader, ToLeetcodeValueNode}, with_input};
pub const TEST_COUNT: usize = 1;

pub fn exec(r: &mut InputReader) -> impl ToLeetcodeValueNode {
    with_input!{ r => echo, 1 }
}

struct Solution;
