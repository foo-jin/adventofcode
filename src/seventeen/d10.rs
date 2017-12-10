
pub fn knothash(input: &str) -> String {
    let mut chars: Vec<char> = input.chars().collect();
    chars.extend(vec![
        17u8 as char,
        31u8 as char,
        73u8 as char,
        47u8 as char,
        23u8 as char,
    ]);

    let lens = chars.iter().collect::<String>();
    let lens = lens.as_bytes();

    let mut nums: Vec<u32> = (0..256).collect();
    let n = nums.len();

    let mut cur = 0;
    let mut skip = 0;

    for _ in 0 .. 64 {
        for len in lens {
            let len = *len as u32;
            let end = cur + len;
            let mut temp = Vec::new();

            for j in cur..end {
                temp.push(nums[j as usize % n])
            }

            for j in cur..end {
                nums[j as usize % n] = temp.pop().unwrap();
            }

            cur += len + skip;
            skip += 1;
        }
    }

    let mut xord = Vec::new();
    let mut clone = nums.clone();

    for _ in 0 .. 16 {
        let mut it = clone.splice(0 .. 16, vec![]);
        let first = it.next().unwrap();
        xord.push(it.fold(first, |x, y| x ^ y))
    }

    let mut result = String::new();
    for s in xord.iter().map(|i| format!("{:x}", i)) {
        let s = if s.len() != 2 {
            format!("0{}", s)
        } else {
            s
        };
        result.push_str(&s);
    }

    result
}

#[cfg(test)]
mod tests {
    use seventeen::d10::*;

    #[test]
    fn test_knothash1() {
        assert_eq!(&knothash(""), "a2582a3a0e66e6e86e3812dcb672a272");
    }

    #[test]
    fn test_knothash2() {
        assert_eq!(&knothash("AoC 2017"), "33efeb34ea91902bb2f59c9920caa6cd");
    }

    #[test]
    fn test_knothash3() {
        assert_eq!(&knothash("1,2,3"), "3efbe78a8d82f29979031a4aa0b16a9d");
    }

    #[test]
    fn test_knothash4() {
        assert_eq!(&knothash("1,2,4"), "63960835bcdc130f0b66d7ff4f6a5a8e");
    }
}
