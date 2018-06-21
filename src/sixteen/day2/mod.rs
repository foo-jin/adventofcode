mod parsing;

use super::Result;

#[derive(Debug, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub fn solve() -> Result<()> {
    let input = super::get_input()?;
    parsing::parse_line("asdf")?;

    super::print_output(2, -1, -1)
}
