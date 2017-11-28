use std::collections::HashSet;

#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn turn(&self, rot: Rotation) -> Direction {
        use sixteen::d1::Rotation::*;
        match rot {
            Right => self.right(),
            Left => self.left(),
        }
    }

    fn right(&self) -> Direction {
        use sixteen::d1::Direction::*;
        match *self {
            North => East,
            South => West,
            East => South,
            West => North,
        }
    }

    fn left(&self) -> Direction {
        use sixteen::d1::Direction::*;
        match *self {
            North => West,
            South => East,
            East => North,
            West => South,
        }
    }
}

#[derive(Debug)]
enum Rotation {
    Right,
    Left,
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
        use sixteen::d1::Direction::*;
        let Point{x, y} = self.location;

        let (x, y) = match self.direction {
            North => (x, y + k),
            South => (x, y - k),
            East => (x + k, y),
            West => (x - k, y),
        };

        Point{x, y}
    }

    fn turn(&mut self, rot: Rotation) {
        self.direction = self.direction.turn(rot);
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

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Point {
    x: i32,
    y: i32
}

impl Point {
    fn new() -> Point {
        Point {
            x: 0,
            y: 0,
        }
    }

    fn distance(&self, p: Point) -> u32 {
        ((self.x - p.x).abs() + (self.y - p.y).abs()) as u32
    }

    fn distance_from_start(&self) -> u32 {
        self.distance(Point::new())
    }

    fn to_line(self, end: Point) -> Line {
        Line{current: self, end}
    }
}

struct Line {
    current: Point,
    end: Point,
}

impl<'a> Iterator for Line {
    type Item = Point;

    fn next(&mut self) -> Option<Point> {
        if self.current.x < self.end.x {
            self.current.x += 1;
        } else if self.current.x > self.end.x {
            self.current.x -= 1;
        } else if self.current.y < self.end.y {
            self.current.y += 1;
        } else if self.current.y > self.end.y {
            self.current.y -= 1;
        } else {
            return None
        }

        Some(self.current)
    }
}

type Move = (Rotation, i32);

fn parse_instructions(input: &str) -> Vec<Move> {
    fn parse<'a, T>(mut chars: T) -> Move
    where
        T: Iterator<Item = char>,
    {
        let rot = chars.next().unwrap();
        let dist = chars.collect::<String>().parse::<i32>().unwrap();
        match rot {
            'R' => (Rotation::Right, dist),
            'L' => (Rotation::Left, dist),
            _ => panic!("Invalid input"),
        }
    }
    let result: Vec<Move> = input.split(',').map(|s| parse(s.trim().chars())).collect();
    result
}

pub fn find_hq(input: &str) -> u32 {
    let instructions = parse_instructions(input);
    let mut position = Position::new();

    for mv in instructions {
        position = position.exec(mv);
    }

    position.location.distance_from_start()
}

pub fn find_cycle(input: &str) -> u32 {
    let instructions = parse_instructions(input);

    let mut current = Position::new();
    let mut visited = HashSet::new();

    for mv in instructions {
        let prev = current.location.clone();
        current = current.exec(mv);
        let line = prev.to_line(current.location);

        for p in line {
            if !visited.insert(p) {
                return p.distance_from_start();
            }
        }
    }
    
    panic!("invalid input");
}


#[cfg(test)]
mod tests {
    use sixteen::d1::*;

    #[test]
    fn test_find_hq1() {
        assert_eq!(find_hq("R2, L3"), 5);
    }

    #[test]
    fn test_find_hq2() {
        assert_eq!(find_hq("R2, R2, R2"), 2);
    }

    #[test]
    fn test_find_hq3() {
        assert_eq!(find_hq("R5, L5, R5, R3"), 12);
    }

    #[test]
    fn test_find_cycle1() {
        assert_eq!(find_cycle("R8, R4, R4, R8"), 4);
    }

    #[test]
    fn test_find_cycle2() {
        assert_eq!(find_cycle("R4, R4, R1, L0, L1, R0, R4, R8"), 8);
    }
}
