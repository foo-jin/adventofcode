use std::collections::HashMap;
use std::collections::VecDeque;

use super::Result;
use self::Direction::{Left, Right};

const FILTER: [char; 4] = ['.', '-', ':', ' '];

#[derive(Eq, PartialEq, Clone, Copy)]
enum Direction {
    Right,
    Left,
}

impl Direction {
    fn parse(s: &str) -> Result<Direction> {
        let s = s.to_lowercase();

        let result = match s.split_whitespace().last().unwrap() {
            "right" => Right,
            "left" => Left,
            _ => bail!("unexpected direction"),
        };

        Ok(result)
    }
}

#[derive(Eq, PartialEq, Clone, Copy, Hash)]
struct State(char);

impl State {
    fn parse(s: &str) -> Result<State> {
        match s.split_whitespace().last().unwrap().chars().next().unwrap() {
            state @ 'A'...'Z' => Ok(State(state)),
            other => bail!("unexpected state: {}", other),
        }
    }
}

type Value = u8;

fn parse_value(s: &str) -> Result<Value> {
    s.split_whitespace()
        .last()
        .unwrap()
        .parse()
        .map_err(Into::into)
}

#[derive(Eq, PartialEq, Clone, Copy)]
struct Actions(Value, Direction, State);

impl Actions {
    fn parse<'a, T>(lines: T) -> Result<Actions>
    where
        T: Iterator<Item = &'a str>,
    {
        let mut lines = lines.map(|s| s.trim().trim_matches(&FILTER[..]).trim());
        let write = parse_value(lines.next().unwrap())?;
        let next = Direction::parse(lines.next().unwrap())?;
        let state = State::parse(lines.next().unwrap())?;

        Ok(Actions(write, next, state))
    }
}

type Instructions = HashMap<(State, Value), Actions>;

#[derive(Eq, PartialEq, Clone)]
struct Program {
    cursor: usize,
    state: State,
    steps: usize,
    tape: VecDeque<Value>,
    inst: Instructions,
}

impl Program {
    fn new(state: State, steps: usize, inst: Instructions) -> Program {
        Program {
            cursor: 50,
            tape: VecDeque::from(vec![0; 100]),
            state,
            steps,
            inst,
        }
    }

    fn parse(s: &str) -> Result<Program> {
        let mut it = s.trim().split("\n\n");
        let mut metadata = it.next()
            .unwrap()
            .lines()
            .map(|l| l.trim_matches(&FILTER[..]));

        let state = State::parse(metadata.next().unwrap())?;
        let steps: usize = metadata
            .next()
            .unwrap()
            .split_whitespace()
            .nth(5)
            .unwrap()
            .parse()?;

        let mut inst: Instructions = HashMap::new();
        for block in it {
            let mut sections = block.split("If").map(|s| s.trim());
            let state = State::parse(sections.next().unwrap().trim_matches(&FILTER[..]))?;

            for sect in sections {
                let mut lines = sect.lines().map(|l| l.trim_matches(&FILTER[..]));
                let val = parse_value(lines.next().unwrap())?;
                let actions = Actions::parse(lines)?;

                inst.insert((state, val), actions);
            }
        }

        Ok(Program::new(state, steps, inst))
    }

    fn checksum(&self) -> usize {
        self.tape.iter().fold(0, |acc, i| acc + *i as usize)
    }

    fn extend_tape(&mut self) {
        let n = self.tape.len();
        if self.cursor == 0 {
            self.tape.push_front(0);
            self.cursor += 1;
        } else if !self.cursor < n {
            self.tape.push_back(0);
        }
    }

    fn step(&mut self) {
        let val = self.tape[self.cursor];
        let Actions(val, dir, state) = self.inst[&(self.state, val)];
        self.tape[self.cursor] = val;

        match dir {
            Right => self.cursor += 1,
            Left => self.cursor -= 1,
        }

        self.state = state;
        self.extend_tape();
    }

    fn eval(&mut self) -> usize {
        for _ in 0..self.steps {
            self.step();
        }

        self.checksum()
    }
}

fn first(input: &str) -> Result<usize> {
    let mut prog = Program::parse(input)?;
    Ok(prog.eval())
}

pub fn run(input: &str) -> Result<usize> {
    first(input)
}

#[allow(dead_code)]
#[cfg(test)]
mod tests {
    use super::*;
    use seventeen::check;

    const IN: &str = include_str!("../../data/d25-test");

    #[test]
    fn test_first() {
        let result = first(IN);
        let expected = 3;
        check(result, expected);
    }

    use test::Bencher;
    const FULL: &str = include_str!("../../data/d25-full");

    #[bench]
    fn bench_p1(b: &mut Bencher) {
        b.iter(|| check(first(FULL), 2870))
    }
}
