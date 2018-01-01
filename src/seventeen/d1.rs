use failure::*;

use super::Result;

fn parse(input: &str) -> Result<Vec<u32>> {
    input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).ok_or(err_msg("unexpected token")))
        .collect::<Result<Vec<_>>>()
}

pub fn reverse_captcha(input: &str) -> Result<u32> {
    let input = parse(input)?;
    let n = input.len();
    let mut result = 0;

    for i in 0..n {
        if input[i] == input[(i + 1) % n] {
            result += input[i];
        }
    }

    Ok(result)
}

pub fn reverse_captcha_half(input: &str) -> Result<u32> {
    let input = parse(input)?;
    let n = input.len();
    let mut result = 0;

    for i in 0..n {
        if input[i] == input[(i + (n / 2)) % n] {
            result += input[i];
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use seventeen::check;

    #[test]
    fn test_reverse_captcha1122() {
        check(reverse_captcha("1122"), 3);
    }

    #[test]
    fn test_reverse_captcha1111() {
        check(reverse_captcha("1111"), 4);
    }

    #[test]
    fn test_reverse_captcha1234() {
        check(reverse_captcha("1234"), 0);
    }

    #[test]
    fn test_reverse_captcha_long() {
        check(reverse_captcha("91212129"), 9);
    }

    #[test]
    fn test_rev_captcha_half1212() {
        check(reverse_captcha_half("1212"), 6);
    }

    #[test]
    fn test_rev_captcha_half1221() {
        check(reverse_captcha_half("1221"), 0);
    }

    #[test]
    fn test_rev_captcha_half_long1() {
        check(reverse_captcha_half("123123"), 12);
    }

    #[test]
    fn test_rev_captcha_half_long2() {
        check(reverse_captcha_half("12131415"), 4);
    }

    use test::Bencher;
    const FULL: &str = include_str!("../../data/d1-test");

    #[bench]
    fn bench_p1(b: &mut Bencher) {
        b.iter(|| check(reverse_captcha(FULL), 1049))
    }

    #[bench]
    fn bench_p2(b: &mut Bencher) {
        b.iter(|| check(reverse_captcha_half(FULL), 1508))
    }
}
