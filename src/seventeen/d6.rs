use std::collections::HashMap;

pub fn redistribute(input: &str) -> u32 {
    let mut input: Vec<u32> = input
        .split_whitespace()
        .map(|s| s.parse::<u32>().unwrap())
        .collect();

    let n = input.len();
    let mut i = 0;
    let mut seen = HashMap::new();

    loop {
        let mut clone = input.clone();
        let (mut key, el) = clone
            .iter_mut()
            .enumerate()
            .rev()
            .max_by_key(|&(_, &mut v)| v)
            .unwrap();

        input[key] = 0;
        key += 1;
        while *el > 0 {
            input[key % n] += 1;
            key += 1;
            *el -= 1;
        }

        i += 1;

        let cpy = input.clone();
        if let Some(x) = seen.insert(cpy, i) {
            return i - x;
        }

    }
}

#[cfg(test)]
mod tests {
    use seventeen::d6::*;

    #[test]
    fn test_redistribute() {
        assert_eq!(redistribute("0\t2\t7\t0"), 4);
    }
}