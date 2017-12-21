extern crate adventofcode;
extern crate failure;

use std::fs::File;
use std::io::prelude::*;

use failure::Error;

use adventofcode::seventeen::*;

fn main() {
    if let Err(e) = run() {
        for cause in e.causes() {
            println!("{}", cause);
        }

        println!("{}", e.backtrace());
    }
}

fn run() -> Result<(), Error> {
    let filename = "./data/input";
    let mut f = File::open(filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let result = d22::run(&contents)?;
    println!("Result: {}", result);
    Ok(())
}
