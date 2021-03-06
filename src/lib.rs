#![feature(test)]
#![feature(iterator_step_by)]
#![feature(range_contains)]
#![feature(type_ascription)]
#![feature(nll)]
#![feature(exact_chunks)]
#![feature(try_from)]

extern crate bit_vec;
extern crate crossbeam;
extern crate crossbeam_channel;
extern crate crypto;
#[macro_use]
extern crate failure;
extern crate fnv;
#[macro_use]
extern crate itertools;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
// #[macro_use]
// extern crate maplit;
#[macro_use]
extern crate nom;
extern crate parking_lot;
extern crate rayon;
extern crate regex;
extern crate test;

pub mod seventeen;
pub mod sixteen;

use std::fmt::{Debug, Display};
use std::io::{self, Read};
use std::result;

use failure::Error;

pub type Result<T> = result::Result<T, Error>;

#[allow(dead_code)]
pub fn check<T>(result: Result<T>, expected: T)
where
    T: PartialEq + Eq + Debug,
{
    match result {
        Ok(result) => {
            assert_eq!(result, expected);
        }
        Err(e) => {
            e.causes().for_each(|cause| println!("{:?}", cause));
            println!("{:?}", e.backtrace());
            panic!("test failed");
        }
    }
}

pub fn get_input() -> Result<String> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    info!("input retrieved");
    Ok(buffer)
}

pub fn print_output<S: Display, T: Display>(day: u8, part1: T, part2: S) -> Result<()> {
    println!("Day {}:\nPart 1: {}\nPart 2: {}", day, part1, part2);
    Ok(())
}
