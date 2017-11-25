pub mod day1 {
    #[derive(Debug)]
    enum Direction {
        North,
        South,
        East,
        West,
    }

    impl Direction {
        fn turn(&self, rot: Rotation) -> Direction {
            use day1::Rotation::*;
            match rot {
                Right => self.right(),
                Left => self.left(),
            }
        }

        fn right(&self) -> Direction {
            use day1::Direction::*;
            match *self {
                North => East,
                South => West,
                East => South,
                West => North,
            }
        }

        fn left(&self) -> Direction {
            use day1::Direction::*;
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

    type Move = (Rotation, i32);

    type Point = (i32, i32);

    fn distance((x, y): Point) -> u32 {
        (x.abs() + y.abs()) as u32
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
                location: (0, 0),
            }
        }

        fn travel(&self, k: i32) -> Point {
            use day1::Direction::*;
            let (x, y) = self.location;

            match self.direction {
                North => (x, y + k),
                South => (x, y - k),
                East => (x + k, y),
                West => (x - k, y),
            }
        }

        fn turn(&mut self, rot: Rotation) {
            self.direction = self.direction.turn(rot);
        }

        fn exec(mut self, (rotation, distance): Move) -> Position {
            self.turn(rotation);
            let location = self.travel(distance);
            Position {
                direction: self.direction,
                location,
            }
        }
    }

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

        distance(position.location)
    }
}

#[cfg(test)]
mod tests {
    use day1::*;
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
}
