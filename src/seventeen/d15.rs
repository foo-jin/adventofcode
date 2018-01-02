use super::Result;

const A: u64 = 16_807;
const B: u64 = 48_271;
const DIV: u64 = 2_147_483_647;

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
    let (mut a, mut b) = parse(input)?;
    let mut matches = 0;

    for _ in 0..40_000_000 {
        a = (a * A) % DIV;
        b = (b * B) % DIV;

        if a as u16 == b as u16 {
            matches += 1;
        }
    }

    Ok(matches)
}

pub fn second(input: &str) -> Result<u32> {
    let (mut a, mut b) = parse(input)?;
    let mut matches = 0;

    for _ in 0..5_000_000 {
        a = (a * A) % DIV;

        while a % 4 != 0 {
            a = (a * A) % DIV;
        }

        b = (b * B) % DIV;

        while b % 8 != 0 {
            b = (b * B) % DIV;
        }

        if a as u16 == b as u16 {
            matches += 1;
        }
    }

    Ok(matches)
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
