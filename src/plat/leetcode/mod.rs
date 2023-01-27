use self::io::{InputReader, OutputPrinter};

pub mod solution;

pub mod defs;
pub mod io;

pub fn run() {
    let mut reader = InputReader::default();
    let mut printer = OutputPrinter::default();
    for _ in 0..solution::TEST_COUNT {
        let res = solution::exec(&mut reader);
        printer.print(&res);
        printer.print(&String::from("\n"));
    }
}
