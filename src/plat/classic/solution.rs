use std::time::Instant;

use super::io::Io;

pub const IS_MULTITEST: bool = false;

pub fn solve(io: &mut Io) {
    let n = 1_000_000;
    let start = Instant::now();
    let mut s: i64 = 0;
    for _ in 0..n {
        s += io.reader.read::<i32>() as i64;
    }
    println!("{}", s);
    println!("{:?}", start.elapsed());
}