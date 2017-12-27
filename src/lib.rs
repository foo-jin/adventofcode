#![feature(i128_type)]
#![feature(inclusive_range_syntax)]
#![feature(match_default_bindings)]
#![feature(test)]
#![feature(iterator_step_by)]
#![feature (range_contains)]
#![feature(type_ascription)]

#[macro_use]
extern crate nom;
#[macro_use]
extern crate failure;
extern crate bit_vec;
extern crate rayon;
extern crate test;
extern crate itertools;

pub mod sixteen;
pub mod seventeen;
pub mod parsing;