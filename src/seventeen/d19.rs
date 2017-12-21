use failure::Error;
use std::collections::HashMap;
use self::Edge::*;
use self::Direction::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Edge {
    Line,
    Corner,
    Letter(char),
}

impl Edge {
    fn letter(self) -> Option<char> {
        if let Letter(c) = self {
            Some(c)
        } else {
            None
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

struct Graph {
    network: HashMap<Node, Edge>,
    init: Node,
}

impl Graph {
    fn new(network: HashMap<Node, Edge>, init: Node) -> Graph {
        Graph { network, init }
    }

    fn from_str(input: &str) -> Graph {
        let mut network = HashMap::new();
        let mut init = (0, 0);
        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars()
                .enumerate()
                .filter(|&(_, c)| !c.is_whitespace())
            {
                let edge = match c {
                    '|' | '-' => Line,
                    '+' => Corner,
                    'A'...'Z' => Letter(c),
                    _ => panic!("Graph::from_str: invalid character"),
                };

                let x = x as i32;
                let y = y as i32;

                if y == 0 {
                    init = (x, y);
                }

                network.insert((x, y), edge);
            }
        }

        Graph::new(network, init)
    }
}

impl IntoIterator for Graph {
    type Item = Edge;
    type IntoIter = Path;

    fn into_iter(self) -> Self::IntoIter {
        Path::new(self.network, self.init, South)
    }
}

struct Path {
    network: HashMap<Node, Edge>,
    current: Node,
    direction: Direction,
}

impl Path {
    fn new(network: HashMap<Node, Edge>, current: Node, direction: Direction) -> Path {
        Path {
            network,
            current,
            direction,
        }
    }
}

impl Iterator for Path {
    type Item = Edge;

    fn next(&mut self) -> Option<Edge> {
        let cur = self.current;
        let dir = self.direction;
        let &result = match self.network.get(&self.current) {
            Some(n) => n,
            None => return None
        };

        if let Corner = result {
            let l = dir.left();
            let ln = self.network.get(&Direction::neigh(cur, l));

            if ln.is_some() {
                self.direction = l;
            } else {
                self.direction = dir.right();
            }
        }

        let dir = self.direction;
        let (x, y) = cur;

        let neigh = match dir {
            North => (x, y - 1),
            South => (x, y + 1),
            West => (x - 1, y),
            East => (x + 1, y),
        };

        self.current = neigh;
        Some(result)
    }
}

#[allow(dead_code)]
fn first(input: &str) -> String {
    Graph::from_str(input).into_iter().filter_map(|e| e.letter()).collect()
}

fn second(input: &str) -> usize {
    Graph::from_str(input).into_iter().count()
}

pub fn run(input: &str) -> Result<usize, Error> {
    Ok(second(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() {
        let input = include_str!("../../data/d19-test");
        assert_eq!(first(input).as_str(), "ABCDEF");
    }

    #[test]
    fn test_second() {
        let input = include_str!("../../data/d19-test");
        assert_eq!(second(input), 38);
    }
}
