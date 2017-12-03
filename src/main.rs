extern crate adventofcode;

use adventofcode::seventeen::d3;
// use std::fs::File;
// use std::io::prelude::*;

fn main() {
    // let filename = "./data/2017/d2-input";
    // let mut f = File::open(filename)
    //     .expect("error opening the file");
    // let mut contents = String::new();
    // f.read_to_string(&mut contents)
    //     .expect("error reading the file");

    let result = d3::firstlarger(265149);
    println!("Result: {}", result);
}
