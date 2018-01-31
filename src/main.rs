extern crate adventofcode;
extern crate clap;
extern crate failure;

use adventofcode::seventeen::*;
use clap::{App, Arg};

fn main() {
    let matches = App::new("Advent of Code in Rust")
        .author("Frank <frank.049@hotmail.com>")
        .about("Advent of code solutions in Rust")
        .arg(
            Arg::with_name("day")
                .required(true)
                .help("The d of the calendar to solve")
                .validator(|s| {
                    s.parse::<u8>()
                        .or(Err("day must be an integer".to_string()))
                        .and_then(|v| match v {
                            1...25 => Ok(()),
                            _ => Err("day must be in the range [1, 25]".to_string()),
                        })
                }),
        )
        .get_matches();

    let day: u8 = matches.value_of("day").unwrap().parse().unwrap();

    if let Err(e) = run(day) {
        e.causes().for_each(|cause| println!("{}", cause));
        println!("{}", e.backtrace());
    }
}

fn run(day: u8) -> Result<()> {
    let solve = match day {
        1 => d1::solve,
        2 => d2::solve,
        3 => d3::solve,
        4 => d4::solve,
        5 => d5::solve,
        6 => d6::solve,
        7 => d7::solve,
        8 => d8::solve,
        9 => d9::solve,
        10 => d10::solve,
        11 => d11::solve,
        12 => d12::solve,
        13 => d13::solve,
        14 => d14::solve,
        15 => d15::solve,
        16 => d16::solve,
        17 => d17::solve,
        18 => d18::solve,
        19 => d19::solve,
        20 => d20::solve,
        21 => d21::solve,
        22 => d22::solve,
        23 => d23::solve,
        24 => d24::solve,
        25 => d25::solve,
        _ => unreachable!(),
    };

    solve()
}
