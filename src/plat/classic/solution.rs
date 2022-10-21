use crate::{input, output};

use super::io::Io;

pub const IS_MULTITEST: bool = false;

pub fn solve(io: &mut Io) {
    input! { io =>
        (n, m): usize,
        a: [[i32; m]; n]
    }
    output! { io =>
        n, m;
        sl(a[0]);
        nl(a[1]);
    }
}