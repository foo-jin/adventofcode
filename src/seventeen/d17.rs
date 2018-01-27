use super::Result;

fn spinlock(steps: usize) -> u32 {
    let mut buf = Vec::with_capacity(2018);
    buf.push(0);
    let mut i = 0;

    for k in 1..2018 {
        let n = k;
        i = ((i + steps) % n) + 1;

        buf.insert(i, k as u32);
    }

    buf[i + 1]
}


fn angry_spinlock(steps: u32, limit: u32) -> u32 {
    let mut i = 0;
    let mut result = 0;

    for k in 1..=limit {
        i = (i + steps) % k + 1;

        if i == 1 {
            result = k;
        }
    }

    result
}

pub fn solve() -> Result<()> {
    let input = super::get_input()?;
    let steps = input.parse()?;
    let first = spinlock(steps);
    let second = angry_spinlock(steps as u32, 50_000_000);

    println!(
        "Day 17:\n\
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
    fn test_first() {
        assert_eq!(spinlock(3), 638);
    }

    #[test]
    fn test_second() {
        assert_eq!(angry_spinlock(3, 9), 9);
    }

    use test::Bencher;

    #[bench]
    fn bench_p1(b: &mut Bencher) {
        let input = 354;
        b.iter(|| assert_eq!(spinlock(input), 2000))
    }

    #[bench]
    fn bench_p2(b: &mut Bencher) {
        let input = 354;
        b.iter(|| assert_eq!(angry_spinlock(input, 50_000_000), 10_242_889))
    }
}
