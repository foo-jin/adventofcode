use super::Result;

fn first(steps: usize) -> u32 {
    let mut buf = Vec::with_capacity(2018);
    buf.push(0);
    let mut i = 0;

    for k in 1..=2017 {
        let n = k;
        i = ((i + steps) % n) + 1;

        buf.insert(i, k as u32);
    }

    let n = buf.len();
    buf[i + 1 % n]
}


fn second(steps: u32, limit: u32) -> u32 {
    let mut i = 0;
    let mut result = 0;

    for k in 1..=limit {
        let n = k;
        i = ((i + steps) % n) + 1;

        if i == 1 {
            result = k;
        }
    }

    result
}

pub fn run(input: &str) -> Result<u32> {
    let steps: u32 = input.parse()?;
    Ok(second(steps, 50_000_000))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() {
        assert_eq!(first(3), 638);
    }

    #[test]
    fn test_second() {
        assert_eq!(second(3, 9), 9);
    }

    use test::Bencher;

    #[bench]
    fn bench_p1(b: &mut Bencher) {
        let input = 354;
        b.iter(|| assert_eq!(first(input), 2000))
    }

    #[bench]
    fn bench_p2(b: &mut Bencher) {
        let input = 354;
        b.iter(|| assert_eq!(second(input, 50_000_000), 10_242_889))
    }
}
