#![feature(i128_type)]
#![feature(inclusive_range_syntax)]
#![feature(match_default_bindings)]
#![feature(test)]
#![feature(iterator_step_by)]
#![feature(range_contains)]
#![feature(type_ascription)]
#![feature(nll)]
#![feature(universal_impl_trait)]

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
extern crate rayon;
extern crate test;
extern crate parking_lot;

pub mod sixteen;
pub mod seventeen;
pub mod parsing;
