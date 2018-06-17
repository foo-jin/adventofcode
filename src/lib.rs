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
