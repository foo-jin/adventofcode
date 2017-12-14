#![feature(i128_type)]
#![feature(inclusive_range_syntax)]

#[macro_use]
extern crate nom;
extern crate failure;
extern crate bit_vec;

pub mod sixteen;
pub mod seventeen;
pub mod parsing;