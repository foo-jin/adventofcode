use std::collections::HashMap;

use failure::Error;

use self::Direction::*;
use self::State::*;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn left(&self) -> Direction {
        match *self {
            North => West,
            South => East,
            East => North,
            West => South,
        }
    }

    fn right(&self) -> Direction {
        match *self {
            North => East,
            South => West,
            East => South,
            West => North,
        }
    }

    fn rev(&self) -> Direction {
        self.right().right()
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum State {
    Clean,
    Weakened,
    Infected,
    Flagged,
}

impl State {
    fn new(c: char) -> State {
        match c {
            '.' => Clean,
            '#' => Infected,
            _ => panic!("invalid state character"),
        }
    }
}

type Coord = (i64, i64);

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Node {
    pos: Coord,
    state: State,
}

impl Node {
    fn new(pos: Coord, state: State) -> Node {
        Node { pos, state }
    }

    fn progress(&mut self) {
        self.state = match self.state {
            Clean => Weakened,
            Weakened => Infected,
            Infected => Flagged,
            Flagged => Clean,   
        }
    }
}

struct Carrier {
    pos: Node,
    dir: Direction,
    grid: HashMap<Coord, Node>,
    infected: usize,
}

impl Carrier {
    fn new(grid: HashMap<Coord, Node>) -> Carrier {
        let pos = *grid.get(&(0, 0)).expect("Origin not in grid");
        let dir = North;
        let infected = 0;
        Carrier { pos, dir, grid, infected }
    }

    fn next(&mut self) {
        let Node{ pos: (x, y), .. } = self.pos;
        let pos = match self.dir {
            North => (x, y - 1),
            South => (x, y + 1),
            East => (x + 1, y),
            West => (x - 1, y),
        };

        match self.grid.get(&pos) {
            Some(node) => self.pos = *node,
            None => {
                self.pos = Node::new(pos, Clean);
            }
        }
    }

    fn update(&mut self) {
        let mut cur = self.pos;
        self.dir = match cur.state {
            Clean => self.dir.left(),
            Infected => self.dir.right(),
            Flagged => self.dir.rev(),
            Weakened => {
                self.infected += 1;
                self.dir
            }
        };

        cur.progress();
        self.grid.insert(cur.pos, cur);
        self.next();
    }
}

fn parse_grid(s: &str) -> HashMap<Coord, Node> {
    let mut out = HashMap::new();
    for (y, line) in s.lines().enumerate() {
        let offset = (line.len() / 2) as i64;
        for (x, c) in line.chars().enumerate() {
            let x = x as i64 - offset;
            let y = y as i64 - offset;
            let p = (x, y);
            let state = State::new(c);
            out.insert(p, Node::new(p, state));
        }
    }
    out
}

fn exec(input: &str, bursts: usize) -> Result<usize, Error> {
    let grid = parse_grid(input);
    let mut carrier = Carrier::new(grid);
    for _ in 0..bursts {
        carrier.update();
    }
    Ok(carrier.infected)
}

pub fn run(input: &str) -> Result<usize, Error> {
    exec(input, 10_000_000)
}

#[cfg(test)]
mod tests {
    use super::*;
    use seventeen::check;

    const input: &str = "..#\n#..\n...";

    #[test]
    fn test_exec1() {
        let result = exec(input, 100);
        let expected = 26;
        check(result, expected);
    }

    #[test]
    fn test_exec2() {
        let result = exec(input, 10_000_000);
        let expected = 2_511_944;
        check(result, expected);
    }
}