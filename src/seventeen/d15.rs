use super::Result;

const A: u64 = 16_807;
const B: u64 = 48_271;
const DIV: u64 = 2_147_483_647;

struct Generator {
    val: u64,
    mul: u64,
}

impl Generator {
    fn new(val: u64, mul: u64) -> Self {
        Generator { val, mul }
    }
}

impl Iterator for Generator {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        self.val *= self.mul;
        self.val %= DIV;

        Some(self.val)
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
    let a = Generator::new(a, A);
    let b = Generator::new(b, B);
    let matches = a.zip(b).take(40_000_000).filter(|&(a, b)| a as u16 == b as u16).count();

    Ok(matches as u32)
}

pub fn second(input: &str) -> Result<u32> {
    let (a, b) = parse(input)?;
    let a = Generator::new(a, A).filter(|a| a % 4 == 0);
    let b = Generator::new(b, B).filter(|b| b % 8 == 0);
    let matches = a.zip(b).take(5_000_000).filter(|&(a, b)| a as u16 == b as u16).count();

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
