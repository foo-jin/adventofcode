use super::Result;
use self::Direction::{N, NE, NW, S, SE, SW};

#[derive(Clone, Copy)]
enum Direction {
    N,
    NE,
    SE,
    S,
    SW,
    NW,
}

impl Direction {
    fn parse(input: &str) -> Direction {
        match input {
            "n" => N,
            "ne" => NE,
            "se" => SE,
            "s" => S,
            "sw" => SW,
            "nw" => NW,
            _ => panic!("unexpected direction: {}", input),
        }
    }
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

    fn neighbour(&self, dir: Direction) -> Point {
        let &Point { x, y } = self;
        let (x, y) = match dir {
            N => (x, y + 2),
            NE => (x + 1, y + 1),
            SE => (x + 1, y - 1),
            S => (x, y - 2),
            SW => (x - 1, y - 1),
            NW => (x - 1, y + 1),
        };

        Point { x, y }
    }

    fn dist(&self, dest: Point) -> u32 {
        let mut cur = *self;
        let mut dist = 0;

        while cur != dest {
            let Point { x: x1, y: y1 } = cur;
            let Point { x: x2, y: y2 } = dest;

            cur = if x1 < x2 {
                if y1 < y2 {
                    dist += 1;
                    cur.neighbour(NE)
                } else if y1 == y2 {
                    dist += (x2 - x1).abs();
                    break;
                } else {
                    dist += 1;
                    cur.neighbour(SE)
                }
            } else if x1 == x2 {
                dist += (y1 - y2).abs() / 2;
                break;
            } else if y1 < y2 {
                dist += 1;
                cur.neighbour(NW)
            } else if y1 == y2 {
                dist += (x2 - x1).abs();
                break;
            } else {
                dist += 1;
                cur.neighbour(SW)
            };
        }
        dist as u32
    }

    fn to_origin(&self) -> u32 {
        self.dist(Point::new())
    }
}

pub fn hexgrid(input: &str) -> (u32, u32) {
    let mut max = 0;
    let last = input
        .trim()
        .split(',')
        .map(Direction::parse)
        .fold(Point::new(), |p, d| {
            let new = p.neighbour(d);
            max = max.max(new.to_origin());
            new
        })
        .to_origin();

    (last, max)
}

pub fn solve() -> Result<()> {
    let input = super::get_input()?;
    let (first, second) = hexgrid(&input);

    println!(
        "Day 11:\n\
         Part 1: {}\n\
         Part 2: {}\n",
        first, second
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_both1() {
        assert_eq!(hexgrid("ne,ne,ne"), (3, 3));
    }

    #[test]
    fn test_both2() {
        assert_eq!(hexgrid("ne,ne,sw,sw"), (0, 2));
    }

    #[test]
    fn test_both3() {
        assert_eq!(hexgrid("ne,ne,s,s"), (2, 2));
    }

    #[test]
    fn test_both4() {
        assert_eq!(hexgrid("se,sw,se,sw,sw"), (3, 3));
    }
}
