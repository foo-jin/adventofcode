mod parsing;

use std::cmp::min;

use self::parsing::parse_directions;
use super::Result;

type Position = (usize, usize);

const START1: Position = (2, 2);
const KEYPAD1: &[&[char]] = &[
    &['0', '0', '0', '0', '0'],
    &['0', '1', '2', '3', '0'],
    &['0', '4', '5', '6', '0'],
    &['0', '7', '8', '9', '0'],
    &['0', '0', '0', '0', '0'],
];

const START2: Position = (1, 3);
const KEYPAD2: &[&[char]] = &[
    &['0', '0', '0', '0', '0', '0', '0'],
    &['0', '0', '0', '1', '0', '0', '0'],
    &['0', '0', '2', '3', '4', '0', '0'],
    &['0', '5', '6', '7', '8', '9', '0'],
    &['0', '0', 'A', 'B', 'C', '0', '0'],
    &['0', '0', '0', 'D', '0', '0', '0'],
    &['0', '0', '0', '0', '0', '0', '0'],
];

#[derive(Debug, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn bathroom_code(keypad: &[&[char]], start: Position, input: &[Vec<Direction>]) -> String {
    let (mut x, mut y) = start;
    input
        .into_iter()
        .map(|directions| {
            directions.into_iter().for_each(|d| {
                use self::Direction::*;
                let (x_, y_) = match d {
                    Up => (x, y - 1),
                    Down => (x, y + 1),
                    Left => (x - 1, y),
                    Right => (x + 1, y),
                };

                if keypad[y_][x_] != '0' {
                    x = x_;
                    y = y_;
                }
            });

            keypad[y][x]
        })
        .collect()
}

pub fn solve() -> Result<()> {
    let input = super::get_input()?;
    let directions = parse_directions(&input)?;

    let part1 = bathroom_code(KEYPAD1, START1, &directions);
    let part2 = bathroom_code(KEYPAD2, START2, &directions);

    super::print_output(2, part1, part2)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple_bathroom_code() {
        let input = parse_directions("ULL\nRRDDD\nLURDL\nUUUUD\n").unwrap();
        let result = bathroom_code(KEYPAD1, START1, &input);
        assert_eq!(result, "1985".to_string());
    }

    #[test]
    fn complex_bathroom_code() {
        let input = parse_directions("ULL\nRRDDD\nLURDL\nUUUUD\n").unwrap();
        let result = bathroom_code(KEYPAD2, START2, &input);
        assert_eq!(result, "5DB3".to_string());
    }
}
