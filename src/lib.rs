#![feature(test)]
#![feature(iterator_step_by)]
#![feature(range_contains)]
#![feature(type_ascription)]

extern crate bit_vec;
extern crate clap;
extern crate crossbeam;
extern crate crossbeam_channel;
#[macro_use]
extern crate failure;
extern crate fnv;
extern crate itertools;
#[macro_use]
extern crate nom;
extern crate parking_lot;
extern crate rayon;
extern crate test;

pub mod parsing;
pub mod seventeen;
pub mod sixteen;

use std::fmt::Debug;
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
    Ok(buffer)
}
