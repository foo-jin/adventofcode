use failure::Error;
use std::collections::{HashSet, VecDeque};
use bit_vec::BitVec;

use super::d10::knothash;

pub fn defrag(input: &str) -> Result<u32, Error> {
    let mut squares = 0;

    for line in 0..128 {
        for mut b in knothash(&format!("{}-{}", input, line)) {
            while b > 0 {
                if b % 2 == 1 {
                    squares += 1;
                }

                b >>= 1;
            }
        }
    }

    Ok(squares)
}

pub fn regions(input: &str) -> Result<u32, Error> {
    let mut grid = HashSet::new();

    for y in 0..128 {
        let bytes = knothash(&format!("{}-{}", input, y));
        let bits = BitVec::from_bytes(&bytes);
        grid.extend(
            bits.into_iter()
                .enumerate()
                .filter(|v| v.1)
                .map(|v| (v.0 as i32, y as i32)),
        );
    }

    let mut regions = 0;
    let mut queue = VecDeque::new();

    while let Some(&k) = grid.iter().next() {
        grid.remove(&k);
        regions += 1;
        queue.push_back(k);
        while let Some((x, y)) = queue.pop_front() {
            queue.extend(grid.take(&(x - 1, y)));
            queue.extend(grid.take(&(x, y - 1)));
            queue.extend(grid.take(&(x + 1, y)));
            queue.extend(grid.take(&(x, y + 1)));
        }
    }

    Ok(regions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_defrag() {
        let input = "flqrgnkx";
        assert_eq!(defrag(input).unwrap(), 8108);
    }

    #[test]
    fn test_regions() {
        let input = "flqrgnkx";
        assert_eq!(regions(input).unwrap(), 1242);
    }
}
