use std::collections::HashMap;

use failure::*;

use self::Edge::{Corner, Letter, Line};
use self::Direction::{East, North, South, West};

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

    fn neigh((x, y): Node, dir: Direction) -> Node {
        match dir {
            North => (x, y - 1),
            South => (x, y + 1),
            West => (x - 1, y),
            East => (x + 1, y),
        }
    }
}

type Node = (i32, i32);

struct Path {
    network: HashMap<Node, Edge>,
    current: Node,
    direction: Direction,
}

impl Path {
    fn new(network: HashMap<Node, Edge>, init: Node) -> Path {
        Path {
            network,
            current: init,
            direction: South,
        }
    }

    fn parse(input: &str) -> Result<Path, Error> {
        let mut network = HashMap::new();
        let mut init = (0, 0);
        for (y, line) in input.trim().lines().enumerate() {
            for (x, c) in line.chars()
                .enumerate()
                .filter(|&(_, c)| !c.is_whitespace())
            {
                let edge = match c {
                    '|' | '-' => Line,
                    '+' => Corner,
                    'A'...'Z' => Letter(c),
                    _ => bail!("unexpected character: {}", c),
                };

                let x = x as i32;
                let y = y as i32;

                if y == 0 {
                    init = (x, y);
                }

                network.insert((x, y), edge);
            }
        }

        Ok(Path::new(network, init))
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
            let l = self.direction.left();
            let k = &Direction::neigh(self.current, l);

            if self.network.contains_key(k) {
                self.direction = l;
            } else {
                self.direction = self.direction.right();
            }
        }

        let (x, y) = self.current;
        let neigh = match self.direction {
            North => (x, y - 1),
            South => (x, y + 1),
            West => (x - 1, y),
            East => (x + 1, y),
        };

        self.current = neigh;
        Some(result)
    }
}

fn first(input: &str) -> Result<String, Error> {
    let result = Path::parse(input)?.filter_map(|e| e.get_letter()).collect();

    Ok(result)
}

fn second(input: &str) -> Result<usize, Error> {
    Ok(Path::parse(input)?.count())
}

pub fn run(input: &str) -> Result<usize, Error> {
    second(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use seventeen::check;

    use test::Bencher;
    const FULL: &str = include_str!("../../data/d19-test");

    #[bench]
    fn bench_p1(b: &mut Bencher) {
        b.iter(|| check(first(FULL), "ABCDEF".to_owned()))
    }

    #[bench]
    fn bench_p2(b: &mut Bencher) {
        b.iter(|| check(second(FULL), 38))
    }
}
