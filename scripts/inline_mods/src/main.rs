use std::env;
use std::path::Path;

use inline::inline_main;

pub mod filter;
pub mod inline;
pub mod use_parser;
pub mod utils;

fn main() {
    let args: Vec<String> = env::args().collect();
    if let [_, arg1, arg2] = &args[..] {
        let src_path = Path::new(arg1);
        let dest_path = Path::new(arg2);
        inline_main(src_path, dest_path);
    } else {
        panic!("expected 2 args, got {:?}", &args[1..]);
    }
}
