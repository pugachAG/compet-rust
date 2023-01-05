pub mod solution;
pub mod io;
pub mod includes;

pub fn run() {
    let mut io = io::Io::default();
    let test_cnt = if solution::IS_MULTITEST {
        io.reader.read::<usize>()
    } else { 1 };
    for _ in 0..test_cnt {
        solution::solve(&mut io);
    }
}