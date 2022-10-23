impl Solution {
    pub fn hardest_worker(n: i32, logs: Vec<Vec<i32>>) -> i32 {
        0
    }
}

use crate::{plat::leetcode::io::{InputReader, ToLeetcodeValueNode}, with_input};
pub const TEST_COUNT: usize = 1;

pub fn exec(r: &mut InputReader) -> impl ToLeetcodeValueNode {
    with_input!{ r => hardest_worker, 2 }
}

struct Solution;
