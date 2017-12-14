
fn reverse<T>(d: &mut [T], pos: usize, length: usize) {
    let len = d.len();

    for (a, b) in (0..length / 2).zip((0..length).rev()) {
        d.swap((pos + a) % len, (pos + b) % len);
    }
}

pub fn knothash(input: &str) -> Vec<u8> {
    let line = input.trim();

    let lengths: Vec<usize> = line.as_bytes()
        .iter()
        .chain(&[17, 31, 73, 47, 23])
        .map(|l| *l as usize)
        .collect();

    let mut sparse: Vec<u8> = (0..=255).collect();

    let mut pos = 0usize;
    let mut skip = 0usize..;

    for _ in 0..64 {
        for (l, skip) in lengths.iter().zip(&mut skip) {
            reverse(&mut sparse, pos, *l);
            pos = (pos + skip + *l) % sparse.len();
        }
    }

    let out: Vec<u8> = sparse
        .chunks(16)
        .map(|chunk| chunk.into_iter().fold(0u8, |s, v| s ^ v))
        .collect();

    out
}

#[cfg(test)]
mod tests {
    use seventeen::d10::*;

    #[test]
    fn test_reverse() {
        let mut d = vec![1, 2, 3, 4, 5, 6];
        reverse(&mut d, 0, 6);
        assert_eq!(vec![6, 5, 4, 3, 2, 1], d);

        let mut d = vec![1, 2, 3, 4, 5, 6];
        reverse(&mut d, 1, 6);
        assert_eq!(vec![2, 1, 6, 5, 4, 3], d);
    }

//     #[test]
//     fn test_knothash1() {
//         assert_eq!(knothash("").as_str(), "a2582a3a0e66e6e86e3812dcb672a272");
//     }

//     #[test]
//     fn test_knothash2() {
//         assert_eq!(knothash("AoC 2017").as_str(), "33efeb34ea91902bb2f59c9920caa6cd");
//     }

//     #[test]
//     fn test_knothash3() {
//         assert_eq!(knothash("1,2,3").as_str(), "3efbe78a8d82f29979031a4aa0b16a9d");
//     }

//     #[test]
//     fn test_knothash4() {
//         assert_eq!(knothash("1,2,4").as_str(), "63960835bcdc130f0b66d7ff4f6a5a8e");
//     }
}
