extern crate adventofcode;
extern crate clap;
extern crate failure;

use adventofcode::{seventeen, sixteen, Result};
use clap::{App, Arg};

fn main() -> Result<()> {
    let matches = App::new("Advent of Code in Rust")
        .author("Frank <frank.049@hotmail.com>")
        .about("Advent of code solutions in Rust")
        .arg(
            Arg::with_name("day")
                .short("d")
                .required(true)
                .help("The day of the calendar to solve")
                .validator(|s| {
                    s.parse::<u8>()
                        .or_else(|_| Err("day must be a positive integer".to_string()))
                        .and_then(|v| match v {
                            1...25 => Ok(()),
                            _ => Err("'day' must be in the range (1...25)".to_string()),
                        })
                })
                .takes_value(true),
        )
        .arg(
            Arg::with_name("edition")
                .short("e")
                .required(true)
                .help("The edition of adventofcode to solve the problem of")
                .validator(|s| {
                    s.parse::<u16>()
                        .or_else(|_| Err("year must be a positive integer".to_string()))
                        .and_then(|v| match v {
                            2016...2017 => Ok(()),
                            _ => Err("'year' must be in the range (2016...2017)".to_string()),
                        })
                })
                .takes_value(true),
        )
        .get_matches();

    let edition: u16 = matches.value_of("edition").unwrap().parse().unwrap();
    let day: u8 = matches.value_of("day").unwrap().parse().unwrap();

    run(edition, day)
}

fn run(edition: u16, day: u8) -> Result<()> {
    match edition {
        2016 => {
            use sixteen::*;
            match day {
                1 => day1::solve(),
                2 => day2::solve(),
                3 => day3::solve(),
                4 => day4::solve(),
                5 => day5::solve(),
                _ => unimplemented!(),
            }
        }
        2017 => {
            use seventeen::*;
            match day {
                1 => day1::solve(),
                2 => day2::solve(),
                3 => day3::solve(),
                4 => day4::solve(),
                5 => day5::solve(),
                6 => day6::solve(),
                7 => day7::solve(),
                8 => day8::solve(),
                9 => day9::solve(),
                10 => day10::solve(),
                11 => day11::solve(),
                12 => day12::solve(),
                13 => day13::solve(),
                14 => day14::solve(),
                15 => day15::solve(),
                16 => day16::solve(),
                17 => day17::solve(),
                18 => day18::solve(),
                19 => day19::solve(),
                20 => day20::solve(),
                21 => day21::solve(),
                22 => day22::solve(),
                23 => day23::solve(),
                24 => day24::solve(),
                25 => day25::solve(),
                _ => unreachable!(),
            }
        }
        _ => unreachable!(),
    }
}
