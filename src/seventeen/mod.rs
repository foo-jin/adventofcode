#![allow(dead_code)]

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day19;
pub mod day20;
pub mod day21;
pub mod day22;
pub mod day23;
pub mod day24;
pub mod day25;

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
