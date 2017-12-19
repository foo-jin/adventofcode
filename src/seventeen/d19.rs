use failure::Error;
use std::collections::HashMap;
use self::Piece::*;
use self::Direction::*;

#[derive(Debug, Eq, PartialEq)]
enum Piece {
    Road,
    Cross,
    Letter(char),
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

struct Graph {
    network: HashMap<Node, Piece>,
    init: Node,
}

impl Graph {
    fn new(network: HashMap<Node, Piece>, init: Node) -> Graph {
        Graph { network, init }
    }

    fn from_str(input: &str) -> Graph {
        let mut network = HashMap::new();
        let mut init = (0, 0);
        for (y, line) in input.lines().enumerate() {
            for (x, piece) in line.chars()
                .enumerate()
                .filter(|&(_, c)| !c.is_whitespace())
            {
                let piece = match piece {
                    '|' | '-' => Road,
                    '+' => Cross,
                    c => Letter(c),
                };

                let x = x as i32;
                let y = y as i32;

                if y == 0 {
                    init = (x, y);
                }

                network.insert((x, y), piece);
            }
        }

        Graph::new(network, init)
    }
}

impl IntoIterator for Graph {
    type Item = char;
    type IntoIter = Path;

    fn into_iter(self) -> Self::IntoIter {
        Path::new(self.network, self.init, South)
    }
}

struct Path {
    network: HashMap<Node, Piece>,
    current: Node,
    direction: Direction,
    steps: usize,
}

impl Path {
    fn new(network: HashMap<Node, Piece>, current: Node, direction: Direction) -> Path {
        Path {
            network,
            current,
            direction,
            steps: 0,
        }
    }
}

impl Iterator for Path {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        loop {
            let dir = self.direction;
            let (x, y) = self.current;
            let neigh = match dir {
                North => (x, y - 1),
                South => (x, y + 1),
                West => (x - 1, y),
                East => (x + 1, y),
            };

            self.current = neigh;
            self.steps += 1;
            let next = match self.network.get(&neigh) {
                Some(n) => n,
                None => return None,
            };
            
            match *next {
                Letter(c) => return Some(c),
                Cross => {
                    let l = dir.left();
                    let ln = self.network.get(&Direction::neigh(neigh, l));
                    let r = dir.right();
                    let rn = self.network.get(&Direction::neigh(neigh, r));
                    if ln.is_some() {
                        self.direction = l;
                    } else if rn.is_some() {
                        self.direction = r;
                    } else {
                        return None;
                    }
                }
                _ => (),
            }
            
        }
    }
}

#[allow(dead_code)]
fn first(input: &str) -> Result<String, Error> {
    let mut result = String::new();
    for c in Graph::from_str(input) {
        result.push(c);
    }

    Ok(result)
}

fn second(input: &str) -> Result<usize, Error> {
    let mut it = Graph::from_str(input).into_iter();
    while let Some(_) = it.next() {}
    Ok(it.steps)
}

pub fn run(input: &str) -> Result<usize, Error> {
    second(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() {
        let input = include_str!("../../data/d19-test");
        assert_eq!(first(input).unwrap().as_str(), "ABCDEF");
    }

    #[test]
    fn test_second() {
        let input = include_str!("../../data/d19-test");
        assert_eq!(second(input).unwrap(), 38);
    }
}
