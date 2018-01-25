use super::Result;

const A: u64 = 16_807;
const B: u64 = 48_271;
const DIV: u64 = 2_147_483_647;

struct Generator {
    value: u64,
    factor: u64,
    check: u64
}

impl Generator {
    fn new(value: u64, factor: u64, check: u64) -> Self {
        Generator { value, factor, check }
    }
}

impl Iterator for Generator {
    type Item = u16;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.value = (self.value * self.factor) % DIV;

            if self.value % self.check == 0 {
                return Some(self.value as u16)
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

fn first(input: &str) -> Result<u32> {
    let (a, b) = parse(input)?;
    let a = Generator::new(a, A, 1);
    let b = Generator::new(b, B, 1);
    let matches = a.zip(b).take(40_000_000).filter(|&(a, b)| a == b).count();

    Ok(matches as u32)
}

pub fn second(input: &str) -> Result<u32> {
    let (a, b) = parse(input)?;
    let a = Generator::new(a, A, 4);
    let b = Generator::new(b, B, 8);
    let matches = a.zip(b).take(5_000_000).filter(|&(a, b)| a == b).count();

    Ok(matches as u32)
}

#[cfg(test)]
mod tests {
    use super::*;
    use seventeen::check;

    const IN: &str = "65\n8921";

    #[test]
    fn test_first() {
        check(first(IN), 588);
    }

    #[test]
    fn test_second() {
        check(second(IN), 309);
    }

    use test::Bencher;
    const FULL: &str = "634\n301";

    #[bench]
    fn bench_p1(b: &mut Bencher) {
        b.iter(|| check(first(FULL), 573))
    }

    #[bench]
    fn bench_p2(b: &mut Bencher) {
        b.iter(|| check(second(FULL), 294))
    }
}
