use std::collections::HashMap;

use super::Result;

fn parse(s: &str) -> Result<Vec<u32>> {
    s.trim()
        .split_whitespace()
        .map(|s| s.parse::<u32>().map_err(Into::into))
        .collect::<Result<_>>()
}

pub fn run(input: &str) -> Result<(u32, u32)> {
    let mut input = parse(input)?;
    let mut seen = HashMap::new();
    let mut result = (0, 0);
    let n = input.len();

    for i in 0.. {
        let mut clone = input.clone();
        if let Some(x) = seen.insert(clone, i) {
            result = (i, i - x);
            break;
        }

        let (mut key, el) = input
            .iter()
            .cloned()
            .enumerate()
            .max_by_key(|&(i, v)| (v, -(i as i32)))
            .unwrap();

        input[key] = 0;
        key += 1;

        for _ in 0..el {
            input[key % n] += 1;
            key += 1;
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use seventeen::check;

    #[test]
    fn test_both() {
        check(run("0\t2\t7\t0"), (5, 4));
    }

    use test::Bencher;
    const FULL: &str = include_str!("../../data/d6-test");

    #[bench]
    fn bench_both(b: &mut Bencher) {
        b.iter(|| check(run(FULL), (12841, 8038)))
    }
}
