extern crate coreutils;

use std::env;
use coreutils::cat;

static PATH_ERROR: &'static str = "Need to specify filepath: 'cat path/to/file'";

fn main() {
    let path = match env::args().nth(1) {
        Some(p) => p,
        None => return println!("{}", PATH_ERROR),
    };

    match cat(path) {
        Err(e) => println!("{}", e),
        _ => (),
    };
}
