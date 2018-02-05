use fnv::FnvHashMap;

use super::Result;

pub fn parse_memory(s: &str) -> Result<Vec<u32>> {
    s.trim()
        .split_whitespace()
        .map(|s| s.parse().map_err(Into::into))
        .collect::<Result<_>>()
}

pub fn redistribute(input: &mut [u32]) -> (u32, u32) {
    let mut seen: FnvHashMap<Vec<u32>, u32> = FnvHashMap::default();
    let n = input.len();

    for i in 0.. {
        if let Some(x) = seen.insert(input.to_vec(), i) {
            return (i, i - x);
        }

        let (mut key, &el) = input
            .iter()
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
    unreachable!()
}

pub fn solve() -> Result<()> {
    let input = super::get_input()?;
    let mut memory = parse_memory(&input)?;
    let (first, second) = redistribute(&mut memory);

    println!(
        "Day 6:\n\
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
    fn test_both() {
        let mut input = [0, 2, 7, 0];
        assert_eq!(redistribute(&mut input), (5, 4));
    }
}
