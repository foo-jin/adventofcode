use std::collections::VecDeque;

use fnv::FnvHashSet;
use bit_vec::BitVec;

use super::day10::knothash;
use super::Result;

type Point = (i32, i32);
type Grid = FnvHashSet<Point>;

fn parse_grid(input: &str) -> Grid {
    let mut grid = FnvHashSet::default();

    for y in 0..128 {
        let bytes = knothash(&format!("{}-{}", input.trim(), y));
        let bits = BitVec::from_bytes(&bytes);
        grid.extend(
            bits.into_iter()
                .enumerate()
                .filter(|(_, b)| *b)
                .map(|(i, _)| (i as i32, y as i32)),
        );
    }

    grid
}

fn squares_used(input: &str) -> u32 {
    let mut squares = 0;

    for line in 0..128 {
        for mut b in knothash(&format!("{}-{}", input, line)) {
            while b > 0 {
                if b & 1 == 1 {
                    squares += 1;
                }

                b >>= 1;
            }
        }
    }

    squares
}

fn regions(mut grid: Grid) -> u32 {
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

    regions
}

pub fn solve() -> Result<()> {
    let input = super::get_input()?;
    let grid = parse_grid(&input);
    let first = squares_used(&input);
    let second = regions(grid);

    println!(
        "Day 14:\n\
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
        let input = "flqrgnkx";
        assert_eq!(squares_used(input), 8108);
    }

    #[test]
    fn test_second() {
        let input = parse_grid("flqrgnkx");
        assert_eq!(regions(input), 1242);
    }

    use test::Bencher;
    const FULL: &str = "oundnydw";

    #[bench]
    fn bench_p1(b: &mut Bencher) {
        b.iter(|| assert_eq!(squares_used(FULL), 8106))
    }

    #[bench]
    fn bench_p2(b: &mut Bencher) {
        let grid = parse_grid(FULL);
        b.iter(|| assert_eq!(regions(grid.clone()), 1164))
    }
}
