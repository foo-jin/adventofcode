use std::collections::HashMap;

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

    fn neighbours(&self) -> Vec<Point> {
        let Point { x, y } = *self;
        let mut result = Vec::new();
        for x in x - 1..=x + 1 {
            for y in y - 1..=y + 1 {
                result.push(Point { x, y });
            }
        }
        result
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn turn(&self) -> Direction {
        use seventeen::d3::Direction::*;
        match *self {
            North => West,
            South => East,
            East => North,
            West => South,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Position {
    direction: Direction,
    location: Point,
}

impl Position {
    fn new() -> Position {
        Position {
            direction: Direction::South,
            location: Point::new(),
        }
    }

    fn turn(&mut self) {
        self.direction = self.direction.turn();
    }

    fn travel(&mut self, k: i32) {
        use seventeen::d3::Direction::*;
        let Point { x, y } = self.location;

        let (x, y) = match self.direction {
            North => (x, y + k),
            South => (x, y - k),
            East => (x + k, y),
            West => (x - k, y),
        };

        self.location = Point { x, y };
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Spiral {
    pos: Position,
    count: u32,
    dist: u32,
    turns: u32,
}

impl Spiral {
    fn new() -> Spiral {
        Spiral {
            pos: Position::new(),
            count: 0,
            dist: 0,
            turns: 0,
        }
    }
}

impl<'a> Iterator for Spiral {
    type Item = Point;

    fn next(&mut self) -> Option<Point> {
        let Spiral { count, dist, .. } = *self;
        if count == 0 {
            self.count = 1;
            Some(Point::new())
        } else {
            if dist == 0 {
                self.pos.turn();
                self.turns += 1;
                self.dist = (self.turns + 1) / 2;
            }

            self.pos.travel(1);
            self.dist -= 1;
            self.count += 1;

            Some(self.pos.location)
        }
    }
}

struct SumSpiral {
    sp: Spiral,
    seen: HashMap<Point, u32>,
}

impl SumSpiral {
    fn new() -> SumSpiral {
        let sp = Spiral::new();
        let mut seen = HashMap::new();
        seen.insert(Point::new(), 1);

        SumSpiral { sp, seen }
    }
}

impl Iterator for SumSpiral {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        let p = self.sp.next().unwrap();
        let val: u32 = p.neighbours()
            .iter()
            .filter_map(|i| self.seen.get(i))
            .sum();

        self.seen.insert(p, val);
        Some(val)
    }
}

pub fn nthspiral(n: usize) -> u32 {
    Spiral::new()
        .nth(n - 1)
        .unwrap()
        .distance_from_start()
}

pub fn firstlarger(n: u32) -> u32 {
    let mut result = 0;
    for v in SumSpiral::new() {
        if v > n {
            result = v;
            break;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nthspiral1() {
        assert_eq!(nthspiral(1), 0);
    }

    #[test]
    fn test_nthspiral2() {
        assert_eq!(nthspiral(2), 1);
    }

    #[test]
    fn test_nthspiral4() {
        assert_eq!(nthspiral(4), 1);
    }

    #[test]
    fn test_nthspiral6() {
        assert_eq!(nthspiral(6), 1);
    }

    #[test]
    fn test_nthspiral8() {
        assert_eq!(nthspiral(8), 1);
    }

    #[test]
    fn test_nthspiral12() {
        assert_eq!(nthspiral(12), 3);
    }

    #[test]
    fn test_nthspiral23() {
        assert_eq!(nthspiral(23), 2);
    }

    #[test]
    fn test_nthspiral1024() {
        assert_eq!(nthspiral(1024), 31);
    }

    #[test]
    fn test_firstlarger0() {
        assert_eq!(firstlarger(0), 1);
    }

    #[test]
    fn test_firstlarger2() {
        assert_eq!(firstlarger(2), 4);
    }

    #[test]
    fn test_firstlarger5() {
        assert_eq!(firstlarger(5), 10);
    }

    #[test]
    fn test_firstlarger11() {
        assert_eq!(firstlarger(11), 23);
    }

    #[test]
    fn test_firstlarger54() {
        assert_eq!(firstlarger(54), 57);
    }

    #[test]
    fn test_firstlarger747() {
        assert_eq!(firstlarger(747), 806);
    }

    use test::Bencher;

    #[bench]
    fn bench_p1(b: &mut Bencher) {
        b.iter(|| assert_eq!(nthspiral(265149), 438))
    }

    #[bench]
    fn bench_p2(b: &mut Bencher) {
        b.iter(|| assert_eq!(firstlarger(265149), 266330))
    }
}
