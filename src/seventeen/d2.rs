use std::iter;

use super::Result;

fn parse(input: &str) -> Result<Vec<Vec<u32>>> {
    input
        .trim()
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|s| s.parse().map_err(Into::into))
                .collect::<Result<_>>()
        })
        .collect::<Result<_>>()
}

pub fn checksum(input: &str) -> Result<u32> {
    let lines = parse(input)?;
    let mut result = 0;

    for l in lines {
        let min = l.iter().min().unwrap();
        let max = l.iter().max().unwrap();
        result += max - min;
    }

    Ok(result)
}

fn divides((x, y): (&u32, &u32)) -> Option<u32> {
    if x % y == 0 {
        Some(x / y)
    } else if y % x == 0 {
        Some(y / x)
    } else {
        None
    }
}

pub fn divsum(input: &str) -> Result<u32> {
    let lines = parse(input)?;
    let mut result = 0u32;

    for l in lines {
        for v in l.iter()
            .enumerate()
            .flat_map(|(i, val)| iter::repeat(val).zip(l.iter().skip(i + 1)))
            .filter_map(divides)
        {
            result += v;
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use seventeen::check;

    #[test]
    fn test_checksum() {
        const IN: &str = "5 1 9 5\n7 5 3\n2 4 6 8";
        check(checksum(IN), 18);
    }

    #[test]
    fn test_divsum() {
        const IN: &str = "5 9 2 8\n9 4 7 3\n3 8 6 5";
        check(divsum(IN), 9);
    }

    use test::Bencher;
    const FULL: &str = include_str!("../../data/d2-test");

    #[bench]
    fn bench_p1(b: &mut Bencher) {
        b.iter(|| check(checksum(FULL), 45351))
    }

    #[bench]
    fn bench_p2(b: &mut Bencher) {
        b.iter(|| check(divsum(FULL), 275))
    }
}
