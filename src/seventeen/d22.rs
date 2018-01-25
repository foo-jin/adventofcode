use std::iter;

use fnv::FnvHashMap;

use super::Result;
use self::Direction::{Down, Left, Right, Up};
use self::State::{Clean, Flagged, Infected, Weakened};

type Coord = (isize, isize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
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
    fn parse(c: char) -> Result<State> {
        let result = match c {
            '.' => Clean,
            '#' => Infected,
            other => bail!("Unexpected state character: {}", other),
        };

        Ok(result)
    }

    fn parse_grid(s: &str) -> Result<FnvHashMap<Coord, State>> {
        let mut grid = FnvHashMap::default();
        
        for (y, line) in s.lines().enumerate() {
            let offset = (line.len() / 2) as isize;
            for (x, c) in line.chars().enumerate() {
                let x = x as isize - offset;
                let y = y as isize - offset;
                let p = (x, y);
                let state = State::parse(c)?;
                grid.insert(p, state);
            }
        }

        Ok(grid)
    }

    fn is_infected(&self) -> bool {
        match self {
            Infected => true,
            _ => false,
        }
    }
}

struct Carrier<F>
where
    F: Fn(State) -> State,
{
    pos: Coord,
    dir: Direction,
    grid: FnvHashMap<Coord, State>,
    progressor: F,
    count: usize,
}

impl<F> Carrier<F>
where
    F: Fn(State) -> State,
{
    fn new(grid: FnvHashMap<Coord, State>, progressor: F) -> Carrier<F> {
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

        self.pos = match self.dir {
            Up => (x, y - 1),
            Down => (x, y + 1),
            Right => (x + 1, y),
            Left => (x - 1, y),
        };

        self.grid.entry(self.pos).or_insert(Clean);
    }

    fn update(&mut self) {
        let state = self.grid.get_mut(&self.pos).unwrap();

        match state {
            Clean => self.dir = self.dir.left(),
            Infected => self.dir = self.dir.right(),
            Flagged => self.dir = self.dir.rev(),
            Weakened => (),
        }

        *state = (self.progressor)(*state);

        if state.is_infected() {
            self.count += 1;
        }

        self.forward();
    }
}

fn exec<F>(input: &str, n: usize, next: F) -> Result<usize>
where
    F: Fn(State) -> State,
{
    let grid = State::parse_grid(input)?;
    let mut carrier = Carrier::new(grid, next);

    for _ in 0..n {
        carrier.update();
    }

    Ok(carrier.count)
}

fn first(input: &str, n: usize) -> Result<usize> {
    let next = |state| match state {
        Clean => Infected,
        Infected => Clean,
        _ => panic!("unexpected state"),
    };
    exec(input, n, next)
}

fn second(input: &str, n: usize) -> Result<usize> {
    let next = |state| match state {
        Clean => Weakened,
        Weakened => Infected,
        Infected => Flagged,
        Flagged => Clean,
    };

    let grid = State::parse_grid(input)?;
    let mut carrier = Carrier::new(grid, next);

    for _ in 0..n {
        carrier.update();
    }

    Ok(carrier.count)
}

pub fn run(input: &str) -> Result<usize> {
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

    use test::Bencher;
    const FULL: &str = include_str!("../../data/d22-test");

    #[bench]
    fn bench_p1(b: &mut Bencher) {
        b.iter(|| check(first(FULL, 10_000), 5433))
    }

    #[bench]
    fn bench_p2(b: &mut Bencher) {
        b.iter(|| check(second(FULL, 10_000_000), 2_512_599))
    }
}
