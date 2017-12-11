
enum Direction {
    N,
    NE,
    SE,
    S,
    SW,
    NW,
}

impl Direction {
    fn new(input: &str) -> Direction {
        use super::d11::Direction::*;

        match input {
            "n" => N,
            "ne" => NE,
            "se" => SE,
            "s" => S,
            "sw" => SW,
            "nw" => NW,
            _ => panic!("faulty input")
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
        Point {
            x: 0,
            y: 0,
        }
    }

    fn neighbour(&self, dir: Direction) -> Point {
        let &Point{x, y} = self;

        use super::d11::Direction::*;
        let (x, y) = match dir {
            N => (x, y+2),
            NE => (x+1, y+1),
            SE => (x + 1, y - 1),
            S => (x, y-2),
            SW => (x-1, y-1),
            NW => (x-1, y+1),
        };

        Point{x, y}
    }

    fn dist(&self, dest: Point, dist: u32) -> u32 {
        use super::d11::Direction::*;

        let cur = *self;
        if cur == dest {
            dist
        } else {
            let Point{x: x1, y: y1} = cur;
            let Point{x: x2, y: y2} = dest;

            if x1 < x2 {
                if y1 < y2 {
                    u32::min(
                        cur.neighbour(NE).dist(dest, dist+1),
                        cur.neighbour(N).dist(dest, dist+1))
                } else if y1 == y2 {
                    u32::min(
                        cur.neighbour(NE).dist(dest, dist+1),
                        cur.neighbour(SE).dist(dest, dist+1))
                } else {
                    cur.neighbour(SE).dist(dest, dist+1)
                }
            } else if x1 == x2 {
                dist + (y1 - y2).abs() as u32
            } else {
                if y1 < y2 {
                    u32::min(
                        cur.neighbour(NW).dist(dest, dist+1),
                        cur.neighbour(N).dist(dest, dist+1))
                } else if y1 == y2 {
                    u32::min(
                        cur.neighbour(NW).dist(dest, dist+1),
                        cur.neighbour(SW).dist(dest, dist+1))
                } else {
                    cur.neighbour(SW).dist(dest, dist+1)
                }
            }
        }
    }

    fn to_origin(&self) -> u32 {
        self.dist(Point::new(), 0)
    }
}

pub fn hexfind(input: &str) -> u32 {
    let end = input.trim().split(',').map(Direction::new).fold(Point::new(), |p, d| p.neighbour(d));
    end.to_origin()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hexfind1() {
        assert_eq!(hexfind("ne,ne,ne"), 3);
    }

    #[test]
    fn test_hexfind2() {
        assert_eq!(hexfind("ne,ne,sw,sw"), 0);
    }

    #[test]
    fn test_hexfind3() {
        assert_eq!(hexfind("ne,ne,s,s"), 2);
    }

    #[test]
    fn test_hexfind4() {
        assert_eq!(hexfind("se,sw,se,sw,sw"), 3);
    }
}
