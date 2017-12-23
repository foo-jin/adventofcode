use std::collections::HashMap;

use failure::Error;

use self::Direction::*;
use self::State::*;

type Coord = (isize, isize);

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Direction {
    fn left(&self) -> Direction {
        match *self {
            Up => Left,
            Down => Right,
            Right => Up,
            Left => Down,
        }
    }

    fn right(&self) -> Direction {
        match *self {
            Up => Right,
            Down => Left,
            Right => Down,
            Left => Up,
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
    
    fn parse_grid(s: &str) -> HashMap<Coord, State> {
        let mut grid = HashMap::new();
        for (y, line) in s.lines().enumerate() {
            let offset = (line.len() / 2) as isize;
            for (x, c) in line.chars().enumerate() {
                let x = x as isize - offset;
                let y = y as isize - offset;
                let p = (x, y);
                let state = State::new(c);
                grid.insert(p, state);
            }
        }
        grid
    }

    fn is_infected(&self) -> bool {
        match self {
            Infected => true,
            _ => false,
        }
    }

    fn flip(self) -> State {
        match self {
            Clean => Infected,
            Infected => Clean,
            _ => panic!("unexpected state"),
        }
    }

    fn escalate(self) -> State {
        match self {
            Clean => Weakened,
            Weakened => Infected,
            Infected => Flagged,
            Flagged => Clean,
        }
    }
}

struct Carrier<F>
where
    F: Fn(State) -> State,
{
    pos: Coord,
    dir: Direction,
    grid: HashMap<Coord, State>,
    progressor: F,
    count: usize,
}

impl<F> Carrier<F>
where
    F: Fn(State) -> State,
{
    fn new(grid: HashMap<Coord, State>, progressor: F) -> Carrier<F> {
        let pos = (0, 0);
        let dir = Up;
        let count = 0;
        Carrier {
            pos,
            dir,
            grid,
            progressor,
            count,
        }
    }

    fn forward(&mut self) {
        let (x, y) = self.pos;
        let pos = match self.dir {
            Up => (x, y - 1),
            Down => (x, y + 1),
            Right => (x + 1, y),
            Left => (x - 1, y),
        };

        self.grid.entry(pos).or_insert(Clean);
        self.pos = pos
    }

    fn update(&mut self, n: usize) {
        for _ in 0..n {
            let state = self.grid[&self.pos];

            self.dir = match state {
                Clean => self.dir.left(),
                Infected => self.dir.right(),
                Flagged => self.dir.rev(),
                Weakened => self.dir,
            };

            let new = (self.progressor)(state);

            if new.is_infected() {
                self.count += 1;
            }

            self.grid.insert(self.pos, new);
            self.forward();
        }
    }
}

fn exec<F>(input: &str, n: usize, next: F) -> Result<usize, Error>
where
    F: Fn(State) -> State,
{
    let grid = State::parse_grid(input);
    let mut carrier = Carrier::new(grid, next);
    carrier.update(n);

    Ok(carrier.count)
}

#[allow(dead_code)]
fn first(input: &str, n: usize) -> Result<usize, Error> {
    let next = State::flip;
    exec(input, n, next)
}

fn second(input: &str, n: usize) -> Result<usize, Error> {
    let next = State::escalate;
    exec(input, n, next)
}

pub fn run(input: &str) -> Result<usize, Error> {
    second(input, 10_000_000)
}

#[allow(dead_code)]
#[cfg(test)]
mod tests {
    use super::*;
    use seventeen::check;

    const IN: &str = "..#\n#..\n...";

    #[test]
    fn test_first1() {
        let result = first(IN, 7);
        let expected = 5;
        check(result, expected);
    }

    #[test]
    fn test_first2() {
        let result = first(IN, 70);
        let expected = 41;
        check(result, expected);
    }

    #[test]
    fn test_first3() {
        let result = first(IN, 10_000);
        let expected = 5587;
        check(result, expected);
    }

    #[test]
    fn test_second1() {
        let result = second(IN, 100);
        let expected = 26;
        check(result, expected);
    }

    #[test]
    fn test_second2() {
        let result = second(IN, 10_000_000);
        let expected = 2_511_944;
        check(result, expected);
    }
}
