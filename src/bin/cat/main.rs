extern crate coreutils;

use std::env;
use coreutils::cat;

static ARG_ERROR: &'static str = "Need to specify filepath: 'cat path/to/file'";

fn main() {
    let path = env::args().nth(1).expect(ARG_ERROR);
    match cat(path) {
        Err(e) => println!("{}", e),
        _ => (),
    };
}
