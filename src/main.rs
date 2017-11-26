extern crate adventofcode;

use adventofcode::day1;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let filename = "./data/D1A2";
    let mut f = File::open(filename)
        .expect("error opening the file");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("error reading the file");

    let result = day1::find_cycle(&contents);
    println!("Distance: {}", result);
}