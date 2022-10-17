use super::io::Io;

pub const IS_MULTITEST: bool = false;

pub fn solve(io: &mut Io) {
    let n = io.reader.read::<i32>();
    io.printer.print(&n);
}