extern crate adventofcode;

use adventofcode::sixteen::d1;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let filename = "./data/2016/d1";
    let mut f = File::open(filename)
        .expect("error opening the file");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("error reading the file");

    let result = d1::find_cycle(&contents);
    println!("Distance: {}", result);
}