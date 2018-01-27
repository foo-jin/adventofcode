use fnv::FnvHashMap;

use super::Result;

use self::Edge::{Corner, Letter, Line};
use self::Direction::{East, North, South, West};
use self::Rotation::{Left, Right};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Edge {
    Line,
    Corner,
    Letter(char),
}

impl Edge {
    fn get_letter(self) -> Option<char> {
        if let Letter(c) = self {
            Some(c)
        } else {
            None
        }
    }

    fn is_corner(&self) -> bool {
        match *self {
            Corner => true,
            _ => false,
        }
    }
}

enum Rotation {
    Left,
    Right,
}

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

    fn turn(&self, r: &Rotation) -> Direction {
        match r {
            Right => self.right(),
            Left => self.left(),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Node(i32, i32);

impl Node {
    fn neigh(&self, dir: Direction) -> Node {
        let Node(x, y) = *self;
        let (x, y) = match dir {
            North => (x, y - 1),
            South => (x, y + 1),
            West => (x - 1, y),
            East => (x + 1, y),
        };

        Node(x, y)
    }
}

struct Path {
    network: FnvHashMap<Node, Edge>,
    current: Node,
    direction: Direction,
}

impl Path {
    fn new(network: FnvHashMap<Node, Edge>, init: Node) -> Path {
        Path {
            network,
            current: init,
            direction: South,
        }
    }

    fn from_str(input: &str) -> Result<Path> {
        let mut network = FnvHashMap::default();
        let mut init = Node(0, 0);
        
        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars()
                .enumerate()
                .filter(|&(_, c)| !c.is_whitespace())
            {
                let edge = match c {
                    '|' | '-' => Line,
                    '+' => Corner,
                    alpha @ 'A'...'Z' => Letter(alpha),
                    other => bail!("unexpected character: {}", other),
                };

                let x = x as i32;
                let y = y as i32;

                if y == 0 {
                    init = Node(x, y);
                }

                network.insert(Node(x, y), edge);
            }
        }

        Ok(Path::new(network, init))
    }

    fn neigh(&self, r: &Rotation) -> Node {
        self.current.neigh(self.direction.turn(r))
    }

    fn next_node(&mut self) {
        self.current = self.current.neigh(self.direction);
    }
}

impl Iterator for Path {
    type Item = Edge;

    fn next(&mut self) -> Option<Edge> {
        let &result = match self.network.get(&self.current) {
            Some(n) => n,
            None => return None,
        };

        if result.is_corner() {
            let k = self.neigh(&Left);

            self.direction = if self.network.contains_key(&k) {
                self.direction.turn(&Left)
            } else {
                self.direction.turn(&Right)
            };
        }

        self.next_node();
        Some(result)
    }
}

fn get_letters(path: &[Edge]) -> String {
    path.into_iter().filter_map(|e| e.get_letter()).collect()
}

pub fn solve() -> Result<()> {
    let input = super::get_input()?;
    let path: Vec<Edge> = Path::from_str(&input)?.collect();
    let first = get_letters(&path);
    let second = path.len();

    println!(
        "Day 19:\n\
         Part 1: {}\n\
         Part 2: {}\n",
        first, second
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    use test::Bencher;
    const FULL: &str = include_str!("../../data/d19-test");

    #[bench]
    fn bench_p1(b: &mut Bencher) {
        let path: Vec<Edge> = Path::from_str(FULL).unwrap().collect();
        b.iter(|| assert_eq!(get_letters(&path), "ABCDEF".to_owned()))
    }

    #[bench]
    fn bench_p2(b: &mut Bencher) {
        let path: Vec<_> = Path::from_str(FULL).unwrap().collect();
        b.iter(|| assert_eq!(path.len(), 38))
    }
}
