use fnv::FnvHashMap;

use super::Result;
use self::Direction::{Down, Left, Right, Up};
use self::State::{Clean, Flagged, Infected, Weakened};

type Coord = (isize, isize);

type Grid = FnvHashMap<Coord, State>;

fn parse_grid(s: &str) -> Result<FnvHashMap<Coord, State>> {
    let mut grid = FnvHashMap::default();

    for (y, line) in s.lines().enumerate() {
        let offset = (line.len() / 2) as isize;
        for (x, c) in line.chars().enumerate() {
            let x = x as isize - offset;
            let y = y as isize - offset;
            let p = (x, y);
            let state = State::from_char(c)?;
            grid.insert(p, state);
        }
    }

    Ok(grid)
}

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
    fn from_char(c: char) -> Result<State> {
        let result = match c {
            '.' => Clean,
            '#' => Infected,
            other => bail!("Unexpected state character: {}", other),
        };

        Ok(result)
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
        {
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
        }

        self.forward();
    }
}

fn exec<F>(grid: Grid, n: usize, transition: F) -> usize
where
    F: Fn(State) -> State,
{
    let mut carrier = Carrier::new(grid, transition);
    (0..n).for_each(|_| carrier.update());
    carrier.count
}

fn infection(grid: Grid, n: usize) -> usize {
    let evolve = |state| match state {
        Clean => Infected,
        Infected => Clean,
        _ => panic!("unexpected state"),
    };
    
    exec(grid, n, evolve)
}

fn evolved_infection(grid: Grid, n: usize) -> usize {
    let evolve = |state| match state {
        Clean => Weakened,
        Weakened => Infected,
        Infected => Flagged,
        Flagged => Clean,
    };

    exec(grid, n, evolve)
}

pub fn solve() -> Result<()> {
    let input = super::get_input()?;
    let grid = parse_grid(&input)?;
    let first = infection(grid.clone(), 10_000);
    let second = evolved_infection(grid, 10_000_000);

    println!(
        "Day 22:\n\
         Part 1: {}\n\
         Part 2: {}\n",
        first, second
    );
    Ok(())
}

#[allow(dead_code)]
#[cfg(test)]
mod tests {
    use super::*;

    const IN: &str = "..#\n#..\n...";

    #[test]
    fn test_first1() {
        let grid = parse_grid(IN).unwrap();
        let result = infection(grid, 7);
        let expected = 5;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_first2() {
        let grid = parse_grid(IN).unwrap();
        let result = infection(grid, 70);
        let expected = 41;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_first3() {
        let grid = parse_grid(IN).unwrap();
        let result = infection(grid, 10_000);
        let expected = 5587;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_second1() {
        let grid = parse_grid(IN).unwrap();
        let result = evolved_infection(grid, 100);
        let expected = 26;
        assert_eq!(result, expected);
    }

    use test::Bencher;
    const FULL: &str = include_str!("../../data/d22-test");

    #[bench]
    fn bench_p1(b: &mut Bencher) {
        let grid = parse_grid(FULL).unwrap();
        b.iter(|| assert_eq!(infection(grid.clone(), 10_000), 5433))
    }

    #[bench]
    fn bench_p2(b: &mut Bencher) {
        let grid = parse_grid(FULL).unwrap();
        b.iter(|| assert_eq!(evolved_infection(grid.clone(), 10_000_000), 2_512_599))
    }
}
