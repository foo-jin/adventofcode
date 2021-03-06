use std::fmt;

use super::Result;

pub struct HexSlice<'a>(&'a [u8]);

impl<'a> HexSlice<'a> {
    pub fn new<T>(data: &'a T) -> HexSlice<'a>
    where
        T: ?Sized + AsRef<[u8]> + 'a,
    {
        HexSlice(data.as_ref())
    }
}

impl<'a> fmt::Display for HexSlice<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for byte in self.0 {
            write!(f, "{:02x}", byte)?;
        }

        Ok(())
    }
}

impl<'a> fmt::Debug for HexSlice<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\"{}\"", self)
    }
}

fn reverse<T>(d: &mut [T], pos: usize, length: usize) {
    let len = d.len();

    for (a, b) in (0..length / 2).zip((0..length).rev()) {
        d.swap((pos + a) % len, (pos + b) % len);
    }
}

pub fn knothash(input: &str) -> Vec<u8> {
    let lengths: Vec<usize> = input
        .trim()
        .as_bytes()
        .iter()
        .chain(&[17, 31, 73, 47, 23])
        .map(|l| *l as usize)
        .collect();

    let mut sparse: Vec<u8> = (0..=255).collect();

    let mut pos = 0;
    let mut skip = 0..;

    for _ in 0..64 {
        for (l, skip) in lengths.iter().zip(&mut skip) {
            reverse(&mut sparse, pos, *l);
            pos = (pos + skip + *l) % sparse.len();
        }
    }

    sparse
        .chunks(16)
        .map(|chunk| chunk.into_iter().fold(0u8, |acc, v| acc ^ v))
        .collect()
}

pub fn solve() -> Result<()> {
    let input = super::get_input()?;
    let second = knothash(&input);

    println!("Day 10:\nPart 2: {}\n", HexSlice::new(&second));
    Ok(())
}

pub fn check_knothash(input: &str, expected: &str) {
    let out = knothash(input);
    let result = HexSlice::new(&out).to_string();
    assert_eq!(result.as_str(), expected);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse() {
        let mut d = vec![1, 2, 3, 4, 5, 6];
        reverse(&mut d, 0, 6);
        assert_eq!(vec![6, 5, 4, 3, 2, 1], d);

        let mut d = vec![1, 2, 3, 4, 5, 6];
        reverse(&mut d, 1, 6);
        assert_eq!(vec![2, 1, 6, 5, 4, 3], d);
    }

    #[test]
    fn test_knothash1() {
        check_knothash("", "a2582a3a0e66e6e86e3812dcb672a272");
    }

    #[test]
    fn test_knothash2() {
        check_knothash("AoC 2017", "33efeb34ea91902bb2f59c9920caa6cd");
    }

    #[test]
    fn test_knothash3() {
        check_knothash("1,2,3", "3efbe78a8d82f29979031a4aa0b16a9d");
    }

    #[test]
    fn test_knothash4() {
        check_knothash("1,2,4", "63960835bcdc130f0b66d7ff4f6a5a8e");
    }
}
