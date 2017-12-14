extern crate adventofcode;

use adventofcode::seventeen::*;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let filename = "./data/input";
    let mut f = File::open(filename).expect("error opening the file");

    let mut contents = String::new();
    f.read_to_string(&mut contents).expect(
        "error reading the file",
    );

    let result = d14::regions(&contents).unwrap();
    println!("Result: {}", result)
}
