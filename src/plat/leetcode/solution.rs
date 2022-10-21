impl Solution {
    pub fn find_max_k(nums: Vec<i32>) -> i32 {
        nums.len() as i32
    }
}

use crate::{plat::leetcode::io::{InputReader, LeetcodeToString}, with_input};
pub const TEST_COUNT: usize = 1;

pub fn exec(r: &mut InputReader) -> impl LeetcodeToString {
    with_input!{ r => find_max_k, 1 }
}

struct Solution;
