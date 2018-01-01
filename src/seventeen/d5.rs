use super::Result;

fn parse(s: &str) -> Result<Vec<i32>> {
    s.trim()
        .lines()
        .map(|s| s.parse().map_err(Into::into))
        .collect::<Result<_>>()
}

fn first(input: &str) -> Result<u32> {
    let mut lines = parse(input)?;
    let n = lines.len() as i32;
    let mut i = 0;
    let mut j = 0;

    while (0..n).contains(i) {
        let inst = &mut lines[i as usize];
        i += *inst;
        *inst += 1;
        j += 1;
    }

    Ok(j)
}

pub fn second(input: &str) -> Result<u32> {
    let mut lines = parse(input)?;
    let n = lines.len() as i32;
    let mut i = 0;
    let mut j = 0;

    while (0..n).contains(i) {
        let el = &mut lines[i as usize];
        i += *el;

        if (*el) >= 3 {
            *el -= 1;
        } else {
            *el += 1;
        }

        j += 1;
    }

    Ok(j)
}

#[cfg(test)]
mod tests {
    use super::*;
    use seventeen::check;
    const IN: &str = "0\n3\n0\n1\n-3";

    #[test]
    fn test_first() {
        check(first(IN), 5);
    }

    #[test]
    fn test_second() {
        check(second(IN), 10);
    }

    use test::Bencher;
    const FULL: &str = include_str!("../../data/d5-test");

    #[bench]
    fn bench_p1(b: &mut Bencher) {
        b.iter(|| check(first(FULL), 373_543))
    }

    #[bench]
    fn bench_p2(b: &mut Bencher) {
        b.iter(|| check(second(FULL), 27_502_966))
    }
}
