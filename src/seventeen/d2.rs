use std::iter;

fn parse(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|l: &str| {
            l.split_whitespace()
                .map(|s| s.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

pub fn checksum(input: &str) -> u32 {
    let lines = parse(input);
    let mut result = 0;

    for l in lines {
        let min = l.iter().min().unwrap();
        let max = l.iter().max().unwrap();
        result += max - min;
    }
    result
}

fn divides(x: u32, y: u32) -> Option<u32> {
    if x % y == 0 {
        Some(x / y)
    } else if y % x == 0 {
        Some(y / x)
    } else {
        None
    }
}

pub fn divsum(input: &str) -> u32 {
    let lines = parse(input);
    let mut result = 0;

    for l in lines {
        for (v1, v2) in l.iter().enumerate().flat_map(|(i, val)| {
            iter::repeat(val).zip(l.iter().skip(i + 1))
        })
        {
            if let Some(x) = divides(*v1, *v2) {
                result += x
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use seventeen::d2::*;

    #[test]
    fn test_checksum() {
        const input: &str = "5 1 9 5\n7 5 3\n2 4 6 8";
        assert_eq!(checksum(input), 18);
    }

    #[test]
    fn test_divsum() {
        const input: &str = "5 9 2 8\n9 4 7 3\n3 8 6 5";
        assert_eq!(divsum(input), 9);
    }
}