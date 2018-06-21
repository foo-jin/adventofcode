use failure::err_msg;
use fnv::FnvHashSet as HashSet;
use seventeen::Result;

use self::Direction::{East, North, South, West};
use self::Rotation::{Left, Right};

#[derive(Clone, Copy, Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn right(&self) -> Direction {
        match *self {
            North => East,
            South => West,
            East => South,
            West => North,
        }
    }

    fn left(&self) -> Direction {
        match *self {
            North => West,
            South => East,
            East => North,
            West => South,
        }
    }

    fn turn(&self, r: Rotation) -> Direction {
        match r {
            Right => self.right(),
            Left => self.left(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Rotation {
    Right,
    Left,
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new() -> Point {
        Point { x: 0, y: 0 }
    }

    fn distance(&self, p: Point) -> u32 {
        ((self.x - p.x).abs() + (self.y - p.y).abs()) as u32
    }

    fn distance_from_start(&self) -> u32 {
        self.distance(Point::new())
    }

    fn to_line(self, end: Point) -> Line {
        Line { current: self, end }
    }
}

#[derive(Debug)]
struct Position {
    direction: Direction,
    location: Point,
}

impl Position {
    fn new() -> Position {
        Position {
            direction: Direction::North,
            location: Point::new(),
        }
    }

    fn travel(&self, k: i32) -> Point {
        let Point { x, y } = self.location;

        let (x, y) = match self.direction {
            North => (x, y + k),
            South => (x, y - k),
            East => (x + k, y),
            West => (x - k, y),
        };

        Point { x, y }
    }

    fn turn(&mut self, r: Rotation) {
        self.direction = self.direction.turn(r);
    }

    fn exec(&mut self, (rotation, distance): Move) -> Position {
        self.turn(rotation);
        let location = self.travel(distance);
        let direction = self.direction;
        Position {
            direction,
            location,
        }
    }
}

struct Line {
    current: Point,
    end: Point,
}

impl Iterator for Line {
    type Item = Point;

    fn next(&mut self) -> Option<Point> {
        use std::cmp::Ordering::*;
        let Point { x: x1, y: y1 } = self.current;
        let Point { x: x2, y: y2 } = self.end;
        let x = &mut self.current.x;
        let y = &mut self.current.y;

        match x1.cmp(&x2) {
            Less => *x += 1,
            Greater => *x -= 1,
            Equal => match y1.cmp(&y2) {
                Less => *y += 1,
                Greater => *y -= 1,
                Equal => return None,
            },
        }

        Some(self.current)
    }
}

type Move = (Rotation, i32);

fn parse_instructions(input: &str) -> Result<Vec<Move>> {
    fn parse_inst<T>(mut chars: T) -> Result<Move>
    where
        T: Iterator<Item = char>,
    {
        let rot = chars.next().unwrap();
        let dist = chars.collect::<String>().parse::<i32>()?;
        let result = match rot {
            'R' => (Right, dist),
            'L' => (Left, dist),
            other => bail!("unexpected rotation: {}", other),
        };

        Ok(result)
    }

    input
        .split(',')
        .map(|s| parse_inst(s.trim().chars()))
        .collect()
}

fn find_hq(instructions: &[Move]) -> Result<u32> {
    let position = instructions
        .into_iter()
        .fold(Position::new(), |mut pos: Position, mv| pos.exec(*mv));

    Ok(position.location.distance_from_start())
}

fn find_cycle(instructions: &[Move]) -> Result<u32> {
    let mut current = Position::new();
    let mut visited = HashSet::default();

    for mv in instructions {
        let prev = current.location;
        current = current.exec(*mv);
        let line = prev.to_line(current.location);

        for p in line {
            if !visited.insert(p) {
                return Ok(p.distance_from_start());
            }
        }
    }

    Err(err_msg("no cycle present in input"))
}

pub fn solve() -> Result<()> {
    let input = super::get_input()?;
    let instructions = parse_instructions(&input)?;
    let part1 = find_hq(&instructions)?;
    let part2 = find_cycle(&instructions)?;

    super::print_output(1, part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use check;

    #[test]
    fn find_hq1() {
        let input = [(Right, 2), (Left, 3)];
        check(find_hq(&input), 5);
    }

    #[test]
    fn find_hq2() {
        let input = [(Right, 2), (Right, 2), (Right, 2)];
        check(find_hq(&input), 2);
    }

    #[test]
    fn find_hq3() {
        let input = [(Right, 5), (Left, 5), (Right, 5), (Right, 3)];
        check(find_hq(&input), 12);
    }

    #[test]
    fn find_cycle1() {
        let input = [(Right, 8), (Right, 4), (Right, 4), (Right, 8)];
        check(find_cycle(&input), 4);
    }

    #[test]
    fn find_cycle2() {
        let input = [
            (Right, 4),
            (Right, 4),
            (Right, 1),
            (Left, 0),
            (Left, 1),
            (Right, 0),
            (Right, 4),
            (Right, 8),
        ];
        check(find_cycle(&input), 8);
    }
}
