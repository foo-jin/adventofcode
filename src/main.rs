extern crate adventofcode;

use adventofcode::sixteen::d1;
use std::fs::File;
use std::io::prelude::*;

const l: [u32; 12] = [3, 9, 4, 1, 5, 4, 2, 4, 2, 2, 10, 7];

fn width(i: u32, j: u32) -> u32 {
    if i > j {
        panic!("invalid input: i > j");
    }
    let mut result = i - j - 1;
    let i = i as usize;
    let j = j as usize;
    let slice = &l[i .. j];
    for k in slice {
        result += k;
    }
    result
}

fn cost(len: u32, i: u32, j: u32) -> u32 {
    (len - width(i, j)).pow(3)
}

fn T(j: u32, len: u32) -> &'a u32 {
    if j == 0 {
        &0
    } else {
        let mut results = Vec::new();
        for i in 0 .. j-1 {
            if width(i+1, j) <= len {
                continue;
            }
            let x = T(i, len) + cost(len, i+1, j);
            results.push(x);
        }
        results.iter().max().unwrap().clone()
    }
    
}

fn main() {
    // let filename = "./data/2016/d1";
    // let mut f = File::open(filename)
    //     .expect("error opening the file");
    // let mut contents = String::new();
    // f.read_to_string(&mut contents)
    //     .expect("error reading the file");

    // let result = d1::find_cycle(&contents);
    // println!("Distance: {}", result);

}
