#![feature(i128_type)]
#![feature(inclusive_range_syntax)]
#![feature(match_default_bindings)]

#[macro_use]
extern crate nom;
extern crate failure;
extern crate bit_vec;
extern crate rayon;

pub mod sixteen;
pub mod seventeen;
pub mod parsing;