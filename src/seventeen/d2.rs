use std::iter;

use super::Result;

fn parse(input: &str) -> Result<Vec<Vec<u32>>> {
    input
        .trim()
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|s| s.parse().map_err(Into::into))
                .collect()
        })
        .collect()
}

fn checksum(lines: &[Vec<u32>]) -> u32 {
    lines
        .iter()
        .map(|l| {
            let min = l.iter().min().unwrap();
            let max = l.iter().max().unwrap();
            max - min
        })
        .sum()
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

fn divsum(lines: &[Vec<u32>]) -> u32 {
    let mut result = 0;

    for l in lines {
        for v in l.iter()
            .enumerate()
            .flat_map(|(i, val)| iter::repeat(val).zip(l.iter().skip(i + 1)))
            .filter_map(divides)
        {
            result += v;
        }
    }

    result
}

pub fn solve() -> Result<()> {
    let input = super::get_input()?;
    let parsed = parse(&input)?;
    let first = checksum(&parsed);
    let second = divsum(&parsed);

    println!(
        "Day 2:\n\
         Part 1: {}\n\
         Part 2: {}\n",
        first, second
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_checksum() {
        let input = [vec![5, 1, 9, 5], vec![7, 5, 3], vec![2, 4, 6, 8]];
        assert_eq!(checksum(&input), 18);
    }

    #[test]
    fn test_divsum() {
        let input = [vec![5, 9, 2, 8], vec![9, 4, 7, 3], vec![3, 8, 6, 5]];
        assert_eq!(divsum(&input), 9);
    }

    use test::Bencher;
    const FULL: &str = include_str!("../../data/d2-test");

    #[bench]
    fn bench_p1(b: &mut Bencher) {
        let input = parse(FULL).unwrap();
        b.iter(|| assert_eq!(checksum(&input), 45351))
    }

    #[bench]
    fn bench_p2(b: &mut Bencher) {
        let input = parse(FULL).unwrap();
        b.iter(|| assert_eq!(divsum(&input), 275))
    }
}
