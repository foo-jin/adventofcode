extern crate adventofcode;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate log;
#[macro_use]
extern crate quicli;

use adventofcode::{seventeen, sixteen};
use quicli::prelude::*;

#[derive(Debug, StructOpt)]
struct Cli {
    /// The edition of adventofcode the problem belongs to
    edition: u32,
    /// The day of the event the problem corresponds to
    day: u32,
    #[structopt(flatten)]
    verbosity: Verbosity,
}

main!(|args: Cli, log_level: verbosity| {
    debug!("{:?}", args);
    let _ = match args.edition {
        2016 => {
            use sixteen::*;
            match args.day {
                1 => day1::solve(),
                2 => day2::solve(),
                3 => day3::solve(),
                4 => day4::solve(),
                5 => day5::solve(),
                6 => day6::solve(),
                7 => day7::solve(),
                _ => bail!("<day> must be an integer in the range (1...25)"),
            }
        }
        2017 => {
            use seventeen::*;
            match args.day {
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
                _ => bail!("<day> must be an integer in the range (1...25)"),
            }
        }
        _ => bail!("<year> must be an integer in the range (2016...2017)"),
    };
});
