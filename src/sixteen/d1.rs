use std::collections::HashSet;

use seventeen::Result;

use self::Rotation::{Left, Right};
use self::Direction::{East, North, South, West};

#[derive(Debug)]
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

    fn exec(mut self, (rotation, distance): Move) -> Position {
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

impl<'a> Iterator for Line {
    type Item = Point;

    fn next(&mut self) -> Option<Point> {
        let Point { x: x1, y: y1 } = self.current;
        let Point { x: x2, y: y2 } = self.end;

        if x1 < x2 {
            self.current.x += 1;
        } else if x1 > x2 {
            self.current.x -= 1;
        } else if y1 < y2 {
            self.current.y += 1;
        } else if y1 > y2 {
            self.current.y -= 1;
        } else {
            return None;
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

pub fn find_hq(input: &str) -> Result<u32> {
    let instructions = parse_instructions(input)?;
    let mut position = Position::new();

    for mv in instructions {
        position = position.exec(mv);
    }

    Ok(position.location.distance_from_start())
}

pub fn find_cycle(input: &str) -> Result<u32> {
    let instructions = parse_instructions(input)?;

    let mut current = Position::new();
    let mut visited = HashSet::new();

    for mv in instructions {
        let prev = current.location;
        current = current.exec(mv);
        let line = prev.to_line(current.location);

        for p in line {
            if !visited.insert(p) {
                return Ok(p.distance_from_start());
            }
        }
    }

    bail!("no cycle present in input");
}


#[cfg(test)]
mod tests {
    use super::*;
    use seventeen::check;

    #[test]
    fn test_find_hq1() {
        check(find_hq("R2, L3"), 5);
    }

    #[test]
    fn test_find_hq2() {
        check(find_hq("R2, R2, R2"), 2);
    }

    #[test]
    fn test_find_hq3() {
        check(find_hq("R5, L5, R5, R3"), 12);
    }

    #[test]
    fn test_find_cycle1() {
        check(find_cycle("R8, R4, R4, R8"), 4);
    }

    #[test]
    fn test_find_cycle2() {
        check(find_cycle("R4, R4, R1, L0, L1, R0, R4, R8"), 8);
    }
}
