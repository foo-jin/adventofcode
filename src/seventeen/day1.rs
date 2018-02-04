use failure::*;

use super::Result;

fn parse(input: &str) -> Result<Vec<u32>> {
    input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).ok_or_else(|| err_msg("unexpected token")))
        .collect()
}

fn reverse_captcha(xs: &[u32]) -> u32 {
    xs.iter()
        .zip(xs.iter().cycle().skip(1))
        .filter_map(|(a, b)| if a == b { Some(a) } else { None })
        .sum()
}

fn reverse_captcha_half(xs: &[u32]) -> u32 {
    xs.iter()
        .zip(xs.iter().cycle().skip(xs.len() / 2))
        .filter_map(|(a, b)| if a == b { Some(a) } else { None })
        .sum()
}

pub fn solve() -> Result<()> {
    let input = super::get_input()?;
    let parsed = parse(&input)?;
    let first = reverse_captcha(&parsed);
    let second = reverse_captcha_half(&parsed);

    println!(
        "Day 1:\n\
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
    fn test_reverse_captcha1122() {
        assert_eq!(reverse_captcha(&[1, 1, 2, 2]), 3);
    }

    #[test]
    fn test_reverse_captcha1111() {
        assert_eq!(reverse_captcha(&[1, 1, 1, 1]), 4);
    }

    #[test]
    fn test_reverse_captcha1234() {
        assert_eq!(reverse_captcha(&[1, 2, 3, 4]), 0);
    }

    #[test]
    fn test_reverse_captcha_long() {
        assert_eq!(reverse_captcha(&[9, 1, 2, 1, 2, 1, 2, 9]), 9);
    }

    #[test]
    fn test_rev_captcha_half1212() {
        assert_eq!(reverse_captcha_half(&[1, 2, 1, 2]), 6);
    }

    #[test]
    fn test_rev_captcha_half1221() {
        assert_eq!(reverse_captcha_half(&[1, 2, 2, 1]), 0);
    }

    #[test]
    fn test_rev_captcha_half_long1() {
        assert_eq!(reverse_captcha_half(&[1, 2, 3, 1, 2, 3]), 12);
    }

    #[test]
    fn test_rev_captcha_half_long2() {
        assert_eq!(reverse_captcha_half(&[1, 2, 1, 3, 1, 4, 1, 5]), 4);
    }

    use test::Bencher;
    const FULL: &str = include_str!("../../data/d1-test");

    #[bench]
    fn bench_p1(b: &mut Bencher) {
        let input = parse(FULL).unwrap();
        b.iter(|| assert_eq!(reverse_captcha(&input), 1049))
    }

    #[bench]
    fn bench_p2(b: &mut Bencher) {
        let input = parse(FULL).unwrap();
        b.iter(|| assert_eq!(reverse_captcha_half(&input), 1508))
    }
}
