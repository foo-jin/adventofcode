extern crate adventofcode;

use adventofcode::seventeen::d9;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let filename = "./data/2017/d9-input";
    let mut f = File::open(filename).expect("error opening the file");

    let mut contents = String::new();
    f.read_to_string(&mut contents).expect(
        "error reading the file",
    );

    let result = d9::count_groups(&contents);
    println!("Result: {}", result)
}
