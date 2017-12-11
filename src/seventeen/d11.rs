
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

    fn dist(&self, dest: Point) -> u32 {
        use super::d11::Direction::*;

        let mut cur = *self;
        let mut dist = 0;

        while cur != dest {
            let Point{x: x1, y: y1} = cur;
            let Point{x: x2, y: y2} = dest;
            
            cur = 
                if x1 < x2 {
                    if y1 < y2 {
                        dist += 1;
                        cur.neighbour(NE)
                    } else if y1 == y2 {
                        dist += (x2 - x1).abs();
                        break
                    } else {
                        dist += 1;
                        cur.neighbour(SE)
                    }
                } else if x1 == x2 {
                    dist += (y1 - y2).abs() / 2;
                    break;
                } else {
                    if y1 < y2 {
                        dist += 1;
                        cur.neighbour(NW)
                    } else if y1 == y2 {
                        dist += (x2 - x1).abs();
                        break
                    } else {
                        dist += 1;
                        cur.neighbour(SW)
                    }
                }
        }
        dist as u32
    }

    fn to_origin(&self) -> u32 {
        self.dist(Point::new())
    }
}

pub fn hexfind(input: &str) -> u32 {
    let mut dist = 0;
    let end = input.trim().split(',').map(Direction::new).fold(Point::new(), |p, d| {
        let new = p.neighbour(d);
        let nd = new.to_origin();
        if nd > dist {
            println!("new: {}", nd);
            dist = nd;
        }
        //dist = u32::max(dist, p.to_origin());
        new
    });
    //end.to_origin()
    dist
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
        assert_eq!(hexfind("ne,ne,sw,sw"), 2);
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
