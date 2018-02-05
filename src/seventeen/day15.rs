use super::Result;

const A: u64 = 16_807;
const B: u64 = 48_271;
const DIV: u64 = 2_147_483_647;

struct Generator {
    value: u64,
    factor: u64,
    check: u64,
}

impl Generator {
    fn new(value: u64, factor: u64, check: u64) -> Self {
        Generator {
            value,
            factor,
            check,
        }
    }
}

impl Iterator for Generator {
    type Item = u16;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.value = (self.value * self.factor) % DIV;

            if self.value % self.check == 0 {
                return Some(self.value as u16);
            }
        }
    }
}

fn parse(s: &str) -> Result<(u64, u64)> {
    let nums: Vec<u64> = s.trim()
        .lines()
        .map(|s| {
            s.trim()
                .split_whitespace()
                .last()
                .unwrap()
                .parse()
                .map_err(Into::into)
        })
        .collect::<Result<_>>()?;

    Ok((nums[0], nums[1]))
}

pub fn first(a: u64, b: u64) -> u32 {
    let a = Generator::new(a, A, 1);
    let b = Generator::new(b, B, 1);
    a.zip(b).take(40_000_000).filter(|&(a, b)| a == b).count() as u32
}

pub fn second(a: u64, b: u64) -> u32 {
    let a = Generator::new(a, A, 4);
    let b = Generator::new(b, B, 8);
    a.zip(b).take(5_000_000).filter(|&(a, b)| a == b).count() as u32
}

pub fn solve() -> Result<()> {
    let input = super::get_input()?;
    let (a, b) = parse(&input)?;
    let first = first(a, b);
    let second = second(a, b);

    println!("Day 15:\n\
         Part 1: {}\n\
         Part 2: {}\n",
        first, second);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() {
        assert_eq!(first(65, 8921), 588);
    }

    #[test]
    fn test_second() {
        assert_eq!(second(65, 8921), 309);
    }
}
