pub mod d1;
pub mod d2;
pub mod d3;
pub mod d4;
pub mod d5;
pub mod d6;
pub mod d7;
pub mod d8;
pub mod d9;
pub mod d10;
pub mod d11;
pub mod d12;
pub mod d13;
pub mod d14;
pub mod d15;
pub mod d16;
pub mod d17;
pub mod d18;
pub mod d19;
pub mod d20;
pub mod d21;
pub mod d22;
#[allow(dead_code)]
pub mod d23;
pub mod d24;

use std::fmt::Debug;
use failure::Error;

#[allow(dead_code)]
fn check<T>(result: Result<T, Error>, expected: T)
where
    T: PartialEq + Eq + Debug,
{
    match result {
        Ok(result) => {
            assert_eq!(result, expected);
        }
        Err(e) => {
            for cause in e.causes() {
                println!("{}", cause);
            }

            println!("{}", e.backtrace());
            panic!("test failed");
        }
    }
}